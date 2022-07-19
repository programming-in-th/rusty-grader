use futures::channel::mpsc::UnboundedSender;
use openssl::ssl::{SslConnector, SslMethod};
use postgres_openssl::MakeTlsConnector;
use realtime_rs::connection::Socket;
use tokio_postgres::Client;

use crate::constants::PULL_MSG;
use crate::utils;

pub type Data = (String, String, String, Vec<String>);

pub async fn connect_socket(url: &str, tx: UnboundedSender<Data>) {
    let mut socket = Socket::new(url);
    socket.connect().await.unwrap();
    let channel = socket.set_channel("realtime:public");
    // let tx = Rc::new(tx);
    channel.join().on(
        "UPDATE",
        Box::new(|data| {
            let table = data["record"].as_object().unwrap();
            let status = table["status"].as_str().unwrap();
            let id = table["id"].as_str().unwrap();
            let task_id = table["taskId"].as_str().unwrap();
            let language = table["language"].as_str().unwrap();
            let code = utils::parse_code(table["code"].as_str().unwrap());

            if status == PULL_MSG {
                tx.unbounded_send((
                    task_id.to_string(),
                    id.to_string(),
                    language.to_string(),
                    code,
                ))
                .unwrap();
            }
        }),
    );

    socket.listen().await;
}

pub async fn connect_db(cert_path: String, db_string: String) -> Client {
    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    builder.set_ca_file(cert_path).unwrap();
    let connector = MakeTlsConnector::new(builder.build());

    let (client, connection) = tokio_postgres::connect(&db_string, connector)
        .await
        .unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    client
}
