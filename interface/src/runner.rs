use crate::constants::{parse_submission_status, PULL_MSG};
use futures::{channel::mpsc::UnboundedSender, executor::block_on};
use grader::{
    errors::GraderResult,
    submission::{result::GroupResult, Submission, SubmissionMessage},
};
use serde_json;
use tokio_postgres::Client;

pub fn update_status(msg: String, client: &Client, submission_id: &str) {
    block_on(async {
        client
            .execute(
                "UPDATE \"Submission\" SET status = $1 WHERE id = $2",
                &[&msg, &submission_id.parse::<i32>().unwrap()],
            )
            .await
            .unwrap();
    });
}

pub fn judge(
    task_id: impl Into<String>,
    submission_id: impl Into<String>,
    language: impl Into<String>,
    code: &[String],
    client: &Client,
) -> GraderResult<()> {
    let task_id = task_id.into();
    let submission_id: String = submission_id.into();
    let language = language.into();

    let callback_sub = submission_id.to_owned();
    let callback_result = submission_id.to_owned();

    let mut result = vec![];
    let mut score = 0.0;
    let mut time = 0;
    let mut memory = 0;

    let mut update_result = move |group: GroupResult| {
        let data = serde_json::to_value(&result).unwrap();

        score += group.score;
        time = std::cmp::max(
            time,
            group
                .run_result
                .iter()
                .map(|r| (r.time_usage * 1000.0) as i32)
                .max()
                .unwrap_or(0),
        );
        memory = std::cmp::max(
            memory,
            group
                .run_result
                .iter()
                .map(|r| r.memory_usage)
                .max()
                .unwrap_or(0) as i32,
        );

        result.push(group);
        block_on(async {
            client
                .execute(
                    "UPDATE \"Submission\" SET \
                    groups = $1, score = $2, time = $3, \
                    memory = $4 WHERE id = $5",
                    &[
                        &data,
                        &(score as i32),
                        &time,
                        &memory,
                        &callback_result.parse::<i32>().unwrap(),
                    ],
                )
                .await
                .unwrap();
        });
    };

    let mut submission = Submission::from(
        task_id,
        submission_id,
        language,
        code,
        Some(Box::new(|input| match input {
            SubmissionMessage::Status(status) => {
                update_status(parse_submission_status(status), client, &callback_sub);
            }
            SubmissionMessage::GroupResult(result) => {
                update_result(result);
            }
            _ => {}
        })),
    )?;

    submission.compile()?;
    let _result = submission.run()?;
    Ok(())
}

pub async fn clear_in_queue(client: &Client, tx: UnboundedSender<String>) {
    let rows = client
        .query(
            "SELECT id FROM \"Submission\" WHERE status = $1",
            &[&PULL_MSG],
        )
        .await
        .unwrap();

    for row in rows.iter() {
        let id: i32 = row.get(0);
        let id = id.to_string();
        tx.unbounded_send(id).unwrap();
    }
}
