use brotli;
use dotenv::dotenv;
use futures::StreamExt;
use futures_util::{future, pin_mut};
use serde_json::Value;
use std::{io::Cursor, sync::Arc, time::Duration};
use tokio::{sync::Mutex, time::sleep};
use tokio_postgres::Client;

use std::env;

mod connection;
mod constants;
mod runner;

async fn pull_and_judge(id: String, client: &Client) {
    let lookup_id = String::from(id);
    futures::executor::block_on(async {
        println!("in the scope");
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        println!("out the scope");
    });
    futures::executor::block_on(async {
        println!("in the scope");
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        println!("out the scope");
    });
    let rows = client
        .query(
            "SELECT \"taskId\", language, \
            code, status  FROM \"Submission\" WHERE id = $1",
            &[&lookup_id.parse::<i32>().unwrap()],
        )
        .await
        .unwrap();
    // futures::executor::block_on(async {
    //     println!("in the scope");
    //     tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    //     println!("out the scope");
    // });
    let task_id: String = rows[0].get(0);
    let language: String = rows[0].get(1);
    let code: Vec<u8> = rows[0].get(2);
    let status: String = rows[0].get(3);

    println!("{}", language);

    // futures::executor::block_on(async {
    //     println!("in the scope");
    //     tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    //     println!("out the scope");
    // });

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
                "UPDATE \"Submission\" SET \
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
        futures::executor::block_on(async {
            println!("in the scope");
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            println!("out the scope");
        });
        let result = runner::judge(task_id, lookup_id.clone(), language, &code, &client).await;
        if result.is_err() {
            runner::update_status(constants::ERROR_MSG.to_string(), &client, &lookup_id);
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cert_path = env::var("CERTIFICATE").unwrap();
    let db_string = env::var("DB_STRING").unwrap();
    let limit = env::var("LIMIT").unwrap().parse::<i32>().unwrap();

    let client = connection::connect_db(&cert_path, &db_string).await;

    loop {
        let (tx, rx) = futures::channel::mpsc::unbounded::<String>();

        runner::clear_in_queue(&client, tx.clone()).await;

        let socket_listen = connection::connect_socket(tx.clone());

        // let counter = Arc::new(Mutex::new(0));

        let stream = {
            rx.for_each(|id| async {
                // while (*counter.lock().await) >= limit {
                //     sleep(Duration::from_millis(100)).await;
                // }
                let id_tmp = String::from(id);
                println!("-> {}", id_tmp);
                // let counter_tmp = counter.clone();
                let connection = connection::connect_db(&cert_path, &db_string).await;
                tokio::spawn(async move {
                    // *counter_tmp.lock().await += 1;
                    pull_and_judge(id_tmp, &connection).await;
                    // *counter_tmp.lock().await -= 1;
                });
            })
        };

        tokio::spawn(socket_listen);
        stream.await;
        // pin_mut!(socket_listen, stream);
        // future::select(socket_listen, stream).await;
    }
}
