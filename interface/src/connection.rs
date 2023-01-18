use futures::channel::mpsc::UnboundedSender;
use openssl::ssl::{SslConnector, SslMethod};
use postgres_openssl::MakeTlsConnector;
use realtime_rs::connection::Socket;
use tokio_postgres::{Client};

use crate::constants::PULL_MSG;

#[allow(dead_code)]
pub async fn connect_socket(url: &str, tx: UnboundedSender<String>) {
    let mut socket = Socket::new(url);
    socket.connect().await.unwrap();
    let channel = socket.set_channel("realtime:public:Submission:status=eq.In Queue");
    channel.join().on(
        "*",
        Box::new(|data| {
            if data.contains_key("record") {
                let table = data["record"].as_object().unwrap();
                let status = table["status"].as_str().unwrap();
                if status == PULL_MSG {
                    let id = table["id"].as_str().unwrap();
                    tx.unbounded_send(id.to_string()).unwrap();
                }
            }
        }),
    );

    socket.listen().await;
}

pub async fn connect_db(db_string: &str) -> Client {
    let builder = SslConnector::builder(SslMethod::tls()).unwrap();
    let mut connector = MakeTlsConnector::new(builder.build());
    connector.set_callback(|config, _| {
        config.set_verify_hostname(false);
        Ok(())
    });

    let (client, connection) = tokio_postgres::connect(db_string, connector)
        .await
        .unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    client
}
