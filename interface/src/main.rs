use dotenv::dotenv;
use futures::{channel::mpsc::UnboundedSender, StreamExt};
use futures_util::{future, pin_mut};
use grader::submission::Submission;
use openssl::ssl::{SslConnector, SslMethod};
use postgres_openssl::MakeTlsConnector;
use realtime_rs::connection::Socket;
use serde_json::Value;
use std::env;
use tokio_postgres::Client;

type Data = (String, String, String, Vec<String>);

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

fn parse_code(code: &str) -> Vec<String> {
    let code = code.trim_end_matches("}").trim_start_matches("{");
    let mut ans = Vec::new();
    let mut start_string = false;
    let mut extra = false;
    let mut pre = '\0';
    for ch in code.chars() {
        if ch == '"' && pre != '\\' {
            start_string = !start_string;
        }
        if pre == ',' && start_string == false {
            start_string = true;
            extra = true;
            ans.push('"');
        }
        if ch == ',' && start_string == true && extra == true {
            start_string = false;
            extra = false;
            ans.push('"');
        }
        ans.push(ch);
        pre = ch;
    }
    if extra {
        ans.push('"');
    }
    let ans: String = ans.iter().collect();
    let code = "[".to_string() + &ans + "]";
    serde_json::from_str::<Value>(&code)
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .map(|x| x.as_str().unwrap().to_string())
        .collect()
}

async fn connect_socket(url: &str, tx: UnboundedSender<Data>) {
    let mut socket = Socket::new(url);
    socket.connect().await.unwrap();
    let channel = socket.set_channel("realtime:public");
    // let tx = Rc::new(tx);
    channel.join().on(
        "UPDATE",
        Box::new(|data| {
            for x in data.keys().into_iter() {
                println!("{}", x);
            }
            let table = data["record"].as_object().unwrap();
            let id = table["id"].as_str().unwrap();
            let task_id = table["taskId"].as_str().unwrap();
            let language = table["language"].as_str().unwrap();
            let code = parse_code(table["code"].as_str().unwrap());

            tx.unbounded_send((
                task_id.to_string(),
                id.to_string(),
                language.to_string(),
                code,
            ))
            .unwrap();
        }),
    );

    socket.listen().await;
}

async fn connect_db(cert_path: String, db_string: String) -> Client {
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

    let client = connect_db(cert_path, db_string).await;

    clear_in_queue(&client, tx.clone()).await;

    let socket_listen = connect_socket(&url, tx.clone());

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
        let ans = parse_code(code);
        assert_eq!(ans, vec!["hello , world", "quote", "\",\"q"]);
    }
}
