use cfg::DatabaseConfig;
use futures::{Sink, Stream, StreamExt};
use serde_json::Value;
use std::{io::Cursor, sync::Arc};
use tokio::task::JoinHandle;

mod cfg;
mod connection;
mod constants;
mod error;
mod rmq;
mod runner;

use connection::SharedClient;
use error::Error;

type SubmissionId = String;

async fn pull_and_judge(id: SubmissionId, client: SharedClient) -> Result<(), Error> {
    log::debug!("start judging {id}");

    let lookup_id = id;

    let lookup_id_as_query_args = match lookup_id.parse::<i32>() {
        Ok(x) => x,
        Err(_) => return Err(Error::InvalidSubmissionId(lookup_id)),
    };

    let rows = client
        .db_client
        .query(
            "SELECT task_id, language, \
            code, status  FROM submission WHERE id = $1",
            &[&lookup_id_as_query_args],
        )
        .await?;

    if rows.is_empty() {
        return Err(Error::SubmissionNotFound);
    }

    let task_id: String = rows[0].get(0);
    let language: String = rows[0].get(1);
    let code: Vec<u8> = rows[0].get(2);
    let status: String = rows[0].get(3);

    if status != constants::PULL_MSG {
        return Err(Error::AlreadyJudge);
    }

    let mut cursor = Cursor::new(Vec::new());
    brotli::BrotliDecompress(&mut Cursor::new(code), &mut cursor)?;
    let code = String::from_utf8(cursor.into_inner())?;
    let code: Value = serde_json::from_str(&code)?;
    let code = code
        .as_array()
        .ok_or(Error::InvalidCode)?
        .iter()
        .map(|x| x.as_str().ok_or(Error::InvalidCode).map(|x| x.to_string()))
        .collect::<Result<Vec<String>, Error>>()?;

    let val: i32 = 0;
    let empty_data = serde_json::to_value(Vec::new() as Vec<i32>)?;
    client
        .db_client
        .execute(
            "UPDATE submission SET \
            groups = $1, score = $2, time = $3, \
            memory = $4, status = $5 WHERE id = $6",
            &[
                &empty_data,
                &val,
                &val,
                &val,
                &"Pending",
                &lookup_id_as_query_args,
            ],
        )
        .await?;

    log::debug!("start judging submission {lookup_id}");
    let result = runner::judge(task_id, &lookup_id, language, &code, client.clone()).await;
    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            if (client
                .update_status(&lookup_id, constants::ERROR_MSG.to_string())
                .await)
                .is_err()
            {
                log::warn!("failed to update status to server");
            }
            Err(Error::GraderError(e))
        }
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let config = match cfg::read_config() {
        Ok(x) => x,
        Err(e) => {
            log::error!("Unable to read config file: {e}");
            return;
        }
    };

    log::info!("starting...");

    let (tx, rx) = futures::channel::mpsc::unbounded::<SubmissionId>();

    let (client, connection) = connection::connect_db(&config.database_config).await;

    let client = Arc::new(client);
    let rmq_channel = Arc::new(
        rmq::get_channel(&config.rmq_config)
            .await
            .expect("Unable to create rabbitmq channel"),
    );

    let shared_client = SharedClient::new(client, rmq_channel, &config.rmq_config);

    let db_connection_handler = tokio::spawn(async move {
        if let Err(e) = connection.await {
            log::error!("connection error: {}", e);
        }
    });
    runner::clear_in_queue(shared_client.clone(), tx.clone()).await;

    let db_notification_handler = handle_db_notification(&config.database_config, tx.clone()).await;
    log::info!("start listening for database notification");

    let submission_handler = handle_message(shared_client.clone(), rx);
    log::info!("start listening for submission through channel");

    tokio::select! {
        _ = submission_handler => {
            log::warn!("submission handler died, exiting...");
            std::process::exit(1);
        },
        _ = db_notification_handler => {
            log::warn!("db notification handler died, exiting...");
            std::process::exit(1);
        },
        _ = db_connection_handler => {
            log::warn!("db connection handler died, exiting...");
            std::process::exit(1);
        },
    };
}

async fn handle_message<T>(client: SharedClient, mut reader: T)
where
    T: Stream<Item = SubmissionId> + std::marker::Unpin,
{
    while let Some(id) = reader.next().await {
        let client = client.clone();
        tokio::spawn(async move {
            if let Err(e) = pull_and_judge(id.clone(), client).await {
                log::warn!("failed to judge submission '{id}'\nreason: {e:?}");
            }
        });
    }
}

async fn handle_db_notification<T>(db_config: &DatabaseConfig, tx: T) -> JoinHandle<()>
where
    T: Sink<SubmissionId> + Send + Sync + 'static,
    <T as Sink<SubmissionId>>::Error: std::fmt::Debug + Send + Sync + 'static,
{
    let (listen_client, listen_connection) = connection::connect_db(db_config).await;
    let listen_client = Arc::new(listen_client);
    let listen = runner::listen_new_submission(listen_client, listen_connection, tx);
    tokio::spawn(listen)
}
