use dotenv::dotenv;
use futures::StreamExt;
use futures_util::{future, pin_mut};

use std::env;

mod connection;
mod constants;
mod runner;

#[tokio::main]

async fn main() {
    dotenv().ok();

    let cert_path = env::var("CERTIFICATE").unwrap();
    let url = env::var("SOCKET").unwrap();
    let db_string = env::var("DB_STRING").unwrap();

    let client = connection::connect_db(cert_path, db_string).await;

    loop {
        let (tx, rx) = futures::channel::mpsc::unbounded::<String>();

        runner::clear_in_queue(&client, tx.clone()).await;

        let socket_listen = connection::connect_socket(&url, tx.clone());

        let stream = {
            rx.for_each(|id| async {
                let lookup_id = String::from(id);
                let rows = client
                .query(
                    "SELECT \"taskId\", language, code, status  FROM \"Submission\" WHERE id = $1",
                    &[&lookup_id.parse::<i32>().unwrap()],
                )
                .await
                .unwrap();

                let task_id: String = rows[0].get(0);
                let language: String = rows[0].get(1);
                let code: Vec<String> = rows[0].get(2);
                let status: String = rows[0].get(3);

                if status == constants::PULL_MSG {
                    let result =
                        runner::judge(task_id, lookup_id.clone(), language, &code, &client);
                    if result.is_err() {
                        runner::update_status(
                            constants::ERROR_MSG.to_string(),
                            &client,
                            &lookup_id,
                        );
                    }
                }
            })
        };

        pin_mut!(socket_listen, stream);
        future::select(socket_listen, stream).await;
    }
}
