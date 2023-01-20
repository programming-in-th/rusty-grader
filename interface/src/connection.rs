use std::sync::Arc;

use openssl::ssl::{SslConnector, SslMethod};
use postgres_openssl::{MakeTlsConnector, TlsStream};
use tokio_postgres::{Client, Connection, Socket};

#[derive(Clone)]
pub struct SharedClient(Arc<Client>);

impl std::ops::Deref for SharedClient {
    type Target = Arc<Client>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Client> for SharedClient {
    fn from(value: Client) -> Self {
        Self(Arc::new(value))
    }
}

#[cfg(development)]
pub async fn connect_db(db_string: &str) -> (SharedClient, Connection<Socket, TlsStream<Socket>>) {
    let builder = SslConnector::builder(SslMethod::tls()).unwrap();
    let mut connector = MakeTlsConnector::new(builder.build());
    connector.set_callback(|config, _| {
        config.set_verify_hostname(false);
        Ok(())
    });

    let (client, connection) = tokio_postgres::connect(db_string, connector).await.unwrap();

    (client.into(), connection)
}

#[cfg(not(development))]
pub async fn connect_db(db_string: &str) -> (SharedClient, Connection<Socket, TlsStream<Socket>>) {
    let cert_path = std::env::var("CERTIFICATE").unwrap();

    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    builder.set_ca_file(cert_path).unwrap();
    let connector = MakeTlsConnector::new(builder.build());

    let (client, connection) = tokio_postgres::connect(db_string, connector).await.unwrap();

    (client.into(), connection)
}
