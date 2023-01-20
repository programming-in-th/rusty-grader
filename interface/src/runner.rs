use crate::{
    constants::{parse_submission_status, PULL_MSG},
    error::Error,
    SubmissionId,
};
use futures::TryStreamExt;
use futures::{channel::mpsc::UnboundedSender, StreamExt};
use futures::{Sink, Stream};
use grader::{
    errors::GraderResult,
    submission::{
        result::{GroupResult, SubmissionResult},
        Submission, SubmissionMessage, SubmissionStatus,
    },
};
use postgres_openssl::TlsStream;
use serde_json;
use tokio::sync::Mutex;
use tokio_postgres::{Connection, Socket};

use super::SharedClient;

use log::{debug, error, warn};

pub struct JudgeState {
    result: Vec<GroupResult>,
    score: f64,
    time: i32,
    memory: i32,
}

pub async fn update_status(
    client: SharedClient,
    submission_id: &str,
    msg: String,
) -> Result<(), Error> {
    debug!("change {submission_id}'s status to {msg}");
    client
        .execute(
            "UPDATE submission SET status = $1 WHERE id = $2",
            &[&msg, &submission_id.parse::<i32>().unwrap()],
        )
        .await?;

    Ok(())
}

pub async fn update_result(
    client: SharedClient,
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

    let score = lock.score;
    let time = lock.time;
    let memory = lock.memory;

    let data = serde_json::to_value(&lock.result).unwrap();
    drop(lock);

    debug!("update {submission_id} to (score: {score}, time: {time}, memory: {memory})");
    client
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

pub async fn judge(
    task_id: impl ToString,
    submission_id: impl ToString,
    language: impl ToString,
    code: &[String],
    client: SharedClient,
) -> GraderResult<SubmissionResult> {
    let task_id = task_id.to_string();
    let submission_id = submission_id.to_string();
    let language = language.to_string();

    let result = vec![];
    let score = 0.0;
    let time = 0;
    let memory = 0;

    let state = Mutex::new(JudgeState {
        result,
        score,
        time,
        memory,
    });

    let (tx, rx) = futures::channel::mpsc::unbounded::<SubmissionMessage>();

    let mut submission =
        Submission::try_from(task_id, submission_id.clone(), language, code, tx).await?;

    tokio::spawn(handle_update_message(
        client.clone(),
        rx,
        submission_id.clone(),
        state,
    ));

    debug!("compiling {submission_id}");
    submission.compile().await?;
    debug!("running {submission_id}");
    let result = submission.run().await?;

    Ok(result)
}

pub async fn clear_in_queue(client: SharedClient, tx: UnboundedSender<SubmissionId>) {
    let rows = client
        .query("SELECT id FROM submission WHERE status = $1", &[&PULL_MSG])
        .await
        .unwrap();

    for row in rows.iter() {
        let id: i32 = row.get(0);
        let id = id.to_string();
        tx.unbounded_send(id).unwrap();
    }
}

pub async fn listen_new_submission<U>(
    client: SharedClient,
    mut connection: Connection<Socket, TlsStream<Socket>>,
    writer: U,
) where
    U: Sink<SubmissionId> + Sync + Send + 'static,
    <U as Sink<SubmissionId>>::Error: std::fmt::Debug + Send + Sync + 'static,
{
    debug!("start listen_new_submission");
    let stream =
        futures::stream::poll_fn(move |cx| connection.poll_message(cx)).map_err(|x| panic!("{x}"));

    let stream = stream.and_then(|msg| async {
        match msg {
            tokio_postgres::AsyncMessage::Notification(msg) => Ok(msg.payload().to_string()),
            _ => panic!(),
        }
    });

    let stream = stream.forward(writer);

    let handle = tokio::spawn(stream);

    match client.batch_execute("LISTEN submit;").await {
        Ok(_) => {}
        Err(_) => {
            error!("Unable to listen to database");
            panic!("Unable to listen to database");
        }
    }

    match handle.await {
        Err(e) => {
            if e.is_cancelled() {
                warn!("Listen new submission got cancelled");
            } else if e.is_panic() {
                warn!("Listen new submisison panic");
            }
        }
        Ok(_) => {}
    }
}

async fn handle_update_message<T>(
    client: SharedClient,
    mut rx: T,
    submission_id: SubmissionId,
    state: Mutex<JudgeState>,
) where
    T: Stream<Item = SubmissionMessage> + std::marker::Unpin,
{
    debug!("start handle_update_message for {submission_id}");
    while let Some(message) = rx.next().await {
        match message {
            SubmissionMessage::Status(status @ SubmissionStatus::Done(..)) => {
                if let Err(e) = update_status(
                    client.clone(),
                    &submission_id,
                    parse_submission_status(status),
                )
                .await
                {
                    warn!("unable to update status to database: {e}");
                }
                break;
            }
            SubmissionMessage::Status(status) => {
                if let Err(e) = update_status(
                    client.clone(),
                    &submission_id,
                    parse_submission_status(status),
                )
                .await
                {
                    warn!("unable to update status to database: {e}");
                }
            }
            SubmissionMessage::GroupResult(group_result) => {
                if let Err(e) =
                    update_result(client.clone(), &submission_id, &state, group_result).await
                {
                    warn!("ubable to update status to database: {e}");
                }
            }
            _ => {}
        }
    }
}
