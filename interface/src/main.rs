use dotenv::dotenv;
use grader::submission::Submission;
use openssl::ssl::{SslConnector, SslMethod};
use postgres_openssl::MakeTlsConnector;
use realtime_rs::connection::Socket;
use std::env;

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

async fn connect_socket(url: &str) {
    let mut socket = Socket::new(url);
    socket.connect().await.unwrap();
    let channel = socket.set_channel("realtime:public");

    channel.join().on(
        "UPDATE",
        Box::new(|data| {
            for x in data.keys().into_iter() {
                println!("{}", x);
            }
            let table = data["record"].as_object().unwrap();
            let submission_id = table["id"].as_str().unwrap();
            let task_id = table["taskId"].as_str().unwrap();
            let language = table["language"].as_str().unwrap();
            let code = table["code"].as_str().unwrap();
            println!(
                "input: {:?}, {:?}, {:?}, {:?}",
                submission_id, task_id, language, code
            );
            judge(
                task_id,
                submission_id,
                language,
                &[String::from(
                    "a, b = map(int, input().split())\nprint(a + b)",
                )],
            );
        }),
    );

    socket.listen().await;
}

#[tokio::main]

async fn main() {
    dotenv().ok();
    let cert_path = env::var("CERTIFICATE").unwrap();
    let url = env::var("SOCKET").unwrap();
    let db_string = env::var("DB_STRING").unwrap();

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
    //
    let rows = client
        .query(
            "SELECT id, \"taskId\", language, code  FROM \"Submission\" WHERE status = 'in_queue'",
            &[],
        )
        .await
        .unwrap();

    // let rows = client.query(&statement, &[]).await.unwrap();
    for row in rows.iter() {
        let id: i32 = row.get(0);
        let task_id: String = row.get(1);
        let language: String = row.get(2);
        let code: Vec<String> = row.get(3);

        println!("{} {} {} {:?}", id, task_id, language, code);
    }

    connect_socket(&url).await;
}
