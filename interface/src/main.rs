use brotli;
use dotenv::dotenv;
use futures::{Sink, Stream, StreamExt};
use serde_json::Value;
use std::io::Cursor;
use tokio::task::JoinHandle;

use std::env;

mod connection;
mod constants;
mod error;
mod runner;

use connection::SharedClient;
use error::Error;

type SubmissionId = String;

async fn pull_and_judge(id: String, client: SharedClient) -> Result<(), Error> {
    let lookup_id = String::from(id);

    let lookup_id_as_query_args = match lookup_id.parse::<i32>() {
        Ok(x) => x,
        Err(_) => return Err(Error::InvalidSubmissionId(lookup_id)),
    };

    let rows = client
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
    let empty_data = serde_json::to_value(&(Vec::new() as Vec<i32>))?;
    client
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

    let result = runner::judge(task_id, &lookup_id, language, &code, client.clone()).await;
    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            if let Err(e) =
                runner::update_status(client.clone(), &lookup_id, constants::ERROR_MSG.to_string())
                    .await
            {
                eprintln!("failed to update status to server: {e}");
            }
            Err(Error::GraderError(e))
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_string = env::var("DB_STRING").unwrap();

    loop {
        println!("starting interface.");

        let (tx, rx) = futures::channel::mpsc::unbounded::<SubmissionId>();

        let (client, connection) = connection::connect_db(&db_string).await;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });
        runner::clear_in_queue(client.clone(), tx.clone()).await;

        let db_notification_handler = handle_db_notification(&db_string, tx.clone()).await;
        println!("Start listening for database notification");

        let submission_handler = handle_message(client.clone(), rx);
        println!("Start listening for submission through channel");

        tokio::select! {
            _ = submission_handler => {
                eprintln!("submission handler died");
            },
            _ = db_notification_handler => {
                eprintln!("db notification handler died");
            },
        };
    }
}

async fn handle_message<T>(client: SharedClient, mut reader: T)
where
    T: Stream<Item = SubmissionId> + std::marker::Unpin,
{
    while let Some(id) = reader.next().await {
        let client = client.clone();
        tokio::spawn(async {
            match pull_and_judge(id, client).await {
                Err(e) => eprintln!("{e:?}"),
                _ => {}
            }
        });
    }
}

async fn handle_db_notification<T>(db_string: impl ToString, tx: T) -> JoinHandle<()>
where
    T: Sink<SubmissionId> + Send + Sync + 'static,
    <T as Sink<SubmissionId>>::Error: std::fmt::Debug + Send + Sync + 'static,
{
    let (listen_client, listen_connection) = connection::connect_db(&db_string.to_string()).await;
    let listen = runner::listen_new_submission(listen_client, listen_connection, tx);
    tokio::spawn(listen)
}
