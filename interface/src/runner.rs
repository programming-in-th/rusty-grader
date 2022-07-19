use crate::connection::Data;
use dotenv::dotenv;
use futures::channel::mpsc::UnboundedSender;
use grader::submission::Submission;
use tokio_postgres::Client;

pub fn judge(
    task_id: impl Into<String>,
    submission_id: impl Into<String>,
    language: impl Into<String>,
    code: &[String],
) {
    dotenv().ok();
    let task_id = task_id.into();
    let submission_id = submission_id.into();
    let language = language.into();
    println!(
        "Get Submission:\n{} {} {} {:?}",
        task_id, submission_id, language, code
    );
    let mut submission = Submission::from(
        task_id,
        submission_id,
        language,
        code,
        Some(Box::new(|input| {
            println!("{:?}", input);
            println!("------------------------------");
        })),
    )
    .unwrap();
    submission.compile().unwrap();
    let _result = submission.run();
}

pub async fn clear_in_queue(client: &Client, tx: UnboundedSender<Data>) {
    let rows = client
        .query(
            "SELECT \"taskId\", id, language, code  FROM \"Submission\" WHERE status = 'in_queue'",
            &[],
        )
        .await
        .unwrap();

    for row in rows.iter() {
        let task_id: String = row.get(0);
        let id: i32 = row.get(1);
        let id = id.to_string();
        let language: String = row.get(2);
        let code: Vec<String> = row.get(3);
        tx.unbounded_send((task_id, id, language, code)).unwrap();
    }
}
