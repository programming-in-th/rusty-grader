use std::sync::Arc;

use grader::submission::result::GroupResult;
use openssl::ssl::{SslConnector, SslMethod};
use postgres_openssl::{MakeTlsConnector, TlsStream};
use tokio::sync::Mutex;
use tokio_postgres::{Client, Connection, Socket};

use crate::{error::Error, runner::JudgeState};
use log::*;

const EPS: f64 = 1e-6;

#[derive(Clone)]
pub struct SharedClient {
    pub db_client: Arc<Client>,
}

impl SharedClient {
    pub fn new(db_client: Arc<Client>) -> Self {
        Self { db_client }
    }

    pub async fn update_result(
        &self,
        submission_id: &str,
        state: &Mutex<JudgeState>,
        group: GroupResult,
    ) -> Result<(), Error> {
        debug!("received new group result for {submission_id}");
        let new_score = group.score;
        let new_time = group
            .run_result
            .iter()
            .map(|r| (r.time_usage * 1000.0) as i32)
            .max()
            .unwrap_or(0);
        let new_memory = group
            .run_result
            .iter()
            .map(|r| r.memory_usage)
            .max()
            .unwrap_or(0) as i32;

        let mut lock = state.lock().await;

        lock.score += new_score;
        lock.time = std::cmp::max(lock.time, new_time);
        lock.memory = std::cmp::max(lock.memory, new_memory);
        lock.result.push(group);

        let score = lock.score + EPS;
        let time = lock.time;
        let memory = lock.memory;

        let data = serde_json::to_value(&lock.result).unwrap();
        drop(lock);

        debug!("update {submission_id} to (score: {score}, time: {time}, memory: {memory})");
        self.db_client
            .execute(
                "UPDATE submission SET \
                            groups = $1, score = $2, time = $3, \
                            memory = $4 WHERE id = $5",
                &[
                    &data,
                    &(score as i32),
                    &time,
                    &memory,
                    &submission_id.parse::<i32>().unwrap(),
                ],
            )
            .await?;

        Ok(())
    }

    pub async fn update_status(&self, submission_id: &str, msg: String) -> Result<(), Error> {
        debug!("change {submission_id}'s status to {msg}");
        self.db_client
            .execute(
                "UPDATE submission SET status = $1 WHERE id = $2",
                &[&msg, &submission_id.parse::<i32>().unwrap()],
            )
            .await?;

        Ok(())
    }
}

pub async fn connect_db(db_string: &str) -> (Client, Connection<Socket, TlsStream<Socket>>) {
    let builder = SslConnector::builder(SslMethod::tls()).unwrap();
    let mut connector = MakeTlsConnector::new(builder.build());
    connector.set_callback(|config, _| {
        config.set_verify_hostname(false);
        Ok(())
    });

    let (client, connection) = tokio_postgres::connect(db_string, connector).await.unwrap();

    (client, connection)
}
