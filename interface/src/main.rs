use dotenv::dotenv;
use futures::{channel::mpsc::UnboundedSender, StreamExt};
use futures_util::{future, pin_mut};
use grader::submission::Submission;

use std::env;
use tokio_postgres::Client;

mod connection;
mod utils;

use connection::Data;

fn judge(
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

async fn clear_in_queue(client: &Client, tx: UnboundedSender<Data>) {
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

#[tokio::main]

async fn main() {
    dotenv().ok();

    let cert_path = env::var("CERTIFICATE").unwrap();
    let url = env::var("SOCKET").unwrap();
    let db_string = env::var("DB_STRING").unwrap();

    let (tx, rx) = futures::channel::mpsc::unbounded::<Data>();

    let client = connection::connect_db(cert_path, db_string).await;

    clear_in_queue(&client, tx.clone()).await;

    let socket_listen = connection::connect_socket(&url, tx.clone());

    let stream = {
        rx.for_each(|data| async {
            let (task_id, id, language, code) = data;
            judge(task_id, id, language, &code);
        })
    };

    pin_mut!(socket_listen, stream);
    future::select(socket_listen, stream).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_code() {
        let code = "{\"hello , world\",quote,\"\\\",\\\"q\"}";
        let ans = utils::parse_code(code);
        assert_eq!(ans, vec!["hello , world", "quote", "\",\"q"]);
    }
}
