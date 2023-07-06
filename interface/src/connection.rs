use std::sync::Arc;

use grader::submission::result::GroupResult;
use lapin::{options::BasicPublishOptions, BasicProperties, Channel};
use openssl::ssl::{SslConnector, SslMethod};
use postgres_openssl::{MakeTlsConnector, TlsStream};
use tokio::sync::Mutex;
use tokio_postgres::{Client, Connection, Socket};

use crate::{cfg::RabbitMqConfig, error::Error, runner::JudgeState};
use log::*;

const EPS: f64 = 1e-6;

#[derive(Clone)]
pub struct SharedClient {
    pub db_client: Arc<Client>,
    pub rmq_channel: Arc<Channel>,
    update_routing_key: String,
}

impl SharedClient {
    pub fn new(
        db_client: Arc<Client>,
        rmq_channel: Arc<Channel>,
        rmq_config: &RabbitMqConfig,
    ) -> Self {
        let update_routing_key = format!("submission.update.{}", rmq_config.env);

        Self {
            db_client,
            rmq_channel,
            update_routing_key,
        }
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
        let row = self
            .db_client
            .query_one(
                "UPDATE submission SET \
                            groups = $1, score = $2, time = $3, \
                            memory = $4 WHERE id = $5 \
                            RETURNING id, groups, status, score",
                &[
                    &data,
                    &(score as i32),
                    &time,
                    &memory,
                    &submission_id.parse::<i32>().unwrap(),
                ],
            )
            .await?;

        let id: i32 = row.get(0);
        let groups: String = row.get(1);
        let status: String = row.get(2);
        let score: i32 = row.get(3);

        let payload = serde_json::json!({
            "id": id,
            "groups": groups,
            "status": status,
            "score": score,
        })
        .to_string();

        let payload = payload.as_bytes();

        if let Err(e) = self
            .rmq_channel
            .basic_publish(
                "",
                &self.update_routing_key,
                BasicPublishOptions::default(),
                payload,
                BasicProperties::default(),
            )
            .await
        {
            log::error!("Unable to publish message: {e}");
        }

        Ok(())
    }

    pub async fn update_status(&self, submission_id: &str, msg: String) -> Result<(), Error> {
        debug!("change {submission_id}'s status to {msg}");
        let row = self.db_client
            .query_one(
                "UPDATE submission SET status = $1 WHERE id = $2 RETURNING id, groups, status, score",
                &[&msg, &submission_id.parse::<i32>().unwrap()],
            )
            .await?;

        let id: i32 = row.get(0);
        let groups: String = row.get(1);
        let status: String = row.get(2);
        let score: i32 = row.get(3);

        let payload = serde_json::json!({
            "id": id,
            "groups": groups,
            "status": status,
            "score": score,
        })
        .to_string();

        let payload = payload.as_bytes();

        if let Err(e) = self
            .rmq_channel
            .basic_publish(
                "",
                &self.update_routing_key,
                BasicPublishOptions::default(),
                payload,
                BasicProperties::default(),
            )
            .await
        {
            log::error!("Unable to publish message: {e}");
        }

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
