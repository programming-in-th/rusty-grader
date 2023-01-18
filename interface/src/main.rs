use brotli;
use dotenv::dotenv;
use futures::StreamExt;
use serde_json::Value;
use std::io::Cursor;
use std::sync::Arc;
use tokio_postgres::Client;

use std::env;

mod connection;
mod constants;
mod runner;

async fn pull_and_judge(id: String, client: Arc<Client>) {
    let lookup_id = String::from(id);

    let rows = client
        .query(
            "SELECT task_id, language, \
            code, status  FROM submission WHERE id = $1",
            &[&lookup_id.parse::<i32>().unwrap()],
        )
        .await
        .unwrap();

    let task_id: String = rows[0].get(0);
    let language: String = rows[0].get(1);
    let code: Vec<u8> = rows[0].get(2);
    let status: String = rows[0].get(3);

    if status == constants::PULL_MSG {
        let mut cursor = Cursor::new(Vec::new());
        brotli::BrotliDecompress(&mut Cursor::new(code), &mut cursor).unwrap();
        let code = String::from_utf8(cursor.into_inner()).unwrap();
        let code: Value = serde_json::from_str(&code).unwrap();
        let code = code
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_str().unwrap().to_string())
            .collect::<Vec<String>>();

        let val: i32 = 0;
        let empty_data = serde_json::to_value(&(Vec::new() as Vec<i32>)).unwrap();
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
                    &lookup_id.parse::<i32>().unwrap(),
                ],
            )
            .await
            .unwrap();

        let result = runner::judge(task_id, &lookup_id, language, &code, Arc::clone(&client)).await;
        if result.is_err() {
            runner::update_status(
                Arc::clone(&client),
                &lookup_id,
                constants::ERROR_MSG.to_string(),
            )
            .await;
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_string = env::var("DB_STRING").unwrap();

    let client = Arc::new(connection::connect_db(&db_string).await);

    let (tx, mut rx) = futures::channel::mpsc::unbounded::<String>();

    runner::clear_in_queue(Arc::clone(&client), tx.clone()).await;

    let handle = async {
        while let Some(id) = rx.next().await {
            let client = Arc::new(connection::connect_db(&db_string).await);
            tokio::spawn(async {
                pull_and_judge(id, client).await;
            });
        }
    };

    handle.await;
}
