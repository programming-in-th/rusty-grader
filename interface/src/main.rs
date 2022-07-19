use dotenv::dotenv;
use futures::StreamExt;
use futures_util::{future, pin_mut};

use std::env;

mod connection;
mod constants;
mod runner;
mod utils;

use connection::Data;

#[tokio::main]

async fn main() {
    dotenv().ok();

    let cert_path = env::var("CERTIFICATE").unwrap();
    let url = env::var("SOCKET").unwrap();
    let db_string = env::var("DB_STRING").unwrap();

    let (tx, rx) = futures::channel::mpsc::unbounded::<Data>();

    let client = connection::connect_db(cert_path, db_string).await;

    runner::clear_in_queue(&client, tx.clone()).await;

    let socket_listen = connection::connect_socket(&url, tx.clone());

    let stream = {
        rx.for_each(|data| async {
            let (task_id, id, language, code) = data;
            // feature: check error
            runner::judge(task_id, id, language, &code, &client).ok();
        })
    };

    pin_mut!(socket_listen, stream);
    future::select(socket_listen, stream).await;
}
