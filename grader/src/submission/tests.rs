use super::*;

use crate::errors::GraderResult;
use crate::utils::tests::get_example_dir;
use dotenv::dotenv;
use std::convert::Infallible;
use tokio::fs;
use tokio::test;

struct MessageSink;

const _: () = {
    use std::pin::Pin;
    use std::task::{Context, Poll};

    impl<T> futures::Sink<T> for MessageSink {
        type Error = Infallible;

        fn poll_ready(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn poll_close(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn start_send(self: Pin<&mut Self>, _: T) -> Result<(), Self::Error> {
            Ok(())
        }
    }
};

#[test]
async fn should_complete_initialize_submission() {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b.cpp"))
        .await
        .unwrap();

    let _submission = Submission::try_from("a_plus_b", "000000", "cpp", &[code], MessageSink).await;
}

#[test]
async fn should_compile_cpp_successfully() -> GraderResult<()> {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b.cpp"))
        .await
        .unwrap();

    let mut submission =
        Submission::try_from("a_plus_b", "000001", "cpp", &vec![code], MessageSink).await?;
    assert!(submission.compile().await?);

    Ok(())
}

#[test]
async fn should_compile_python_successfully() -> GraderResult<()> {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b.py"))
        .await
        .unwrap();

    let mut submission =
        Submission::try_from("a_plus_b", "000002", "python", &vec![code], MessageSink).await?;
    assert!(submission.compile().await?);

    Ok(())
}

#[test]
async fn should_compile_rust_successfully() -> GraderResult<()> {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b.rs"))
        .await
        .unwrap();

    let mut submission =
        Submission::try_from("a_plus_b", "000003", "rust", &vec![code], MessageSink).await?;
    assert!(submission.compile().await?);

    Ok(())
}

#[test]
async fn should_remove_tmp_dir_after_out_of_scope() -> GraderResult<()> {
    dotenv().ok();

    let tmp_path;
    {
        let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b.cpp"))
            .await
            .unwrap();

        let mut submission =
            Submission::try_from("a_plus_b", "000004", "cpp", &vec![code], MessageSink).await?;
        assert!(submission.compile().await?);
        tmp_path = submission.tmp_path.clone();
    }

    assert!(!tmp_path.exists());

    Ok(())
}

#[test]
async fn should_run_cpp_successfully() -> GraderResult<()> {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b.cpp"))
        .await
        .unwrap();

    let mut submission =
        Submission::try_from("a_plus_b", "000005", "cpp", &vec![code], MessageSink).await?;
    assert!(submission.compile().await?);

    let _result = submission.run().await?;
    assert_eq!(_result.score, 100.0);

    Ok(())
}

#[test]
async fn should_run_cpp_tle_skipped() -> GraderResult<()> {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b_TLE.cpp"))
        .await
        .unwrap();

    let mut submission =
        Submission::try_from("a_plus_b", "000006", "cpp", &vec![code], MessageSink).await?;
    assert!(submission.compile().await?);

    let _result = submission.run().await?;

    assert_eq!(_result.score, 0.0);

    assert_eq!(_result.group_result[0].score, 0.0);
    assert_eq!(
        _result.group_result[0].run_result[0].status,
        "Time Limit Exceeded"
    );
    assert_eq!(_result.group_result[0].run_result[1].status, "");

    assert_eq!(_result.group_result[1].score, 0.0);
    assert_eq!(
        _result.group_result[1].run_result[0].status,
        "Time Limit Exceeded"
    );
    assert_eq!(_result.group_result[1].run_result[1].status, "");

    Ok(())
}

#[test]
async fn should_run_cpp_mle_skipped() -> GraderResult<()> {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b_MLE.cpp"))
        .await
        .unwrap();

    let mut submission =
        Submission::try_from("a_plus_b", "000007", "cpp", &vec![code], MessageSink).await?;
    assert!(submission.compile().await?);

    let _result = submission.run().await?;

    assert_eq!(_result.score, 0.0);

    assert_eq!(_result.group_result[0].score, 0.0);
    assert_eq!(
        _result.group_result[0].run_result[0].status,
        "Memory Limit Exceeded"
    );
    assert_eq!(_result.group_result[0].run_result[1].status, "");

    assert_eq!(_result.group_result[1].score, 0.0);
    assert_eq!(
        _result.group_result[1].run_result[0].status,
        "Memory Limit Exceeded"
    );
    assert_eq!(_result.group_result[1].run_result[1].status, "");

    Ok(())
}

#[test]
async fn should_run_cpp_re_skipped() -> GraderResult<()> {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b_RE.cpp"))
        .await
        .unwrap();

    let mut submission =
        Submission::try_from("a_plus_b", "000008", "cpp", &vec![code], MessageSink).await?;
    assert!(submission.compile().await?);

    let _result = submission.run().await?;

    assert_eq!(_result.score, 0.0);

    assert_eq!(_result.group_result[0].score, 0.0);
    assert_eq!(
        _result.group_result[0].run_result[0].status,
        "Runtime Error"
    );
    assert_eq!(_result.group_result[0].run_result[1].status, "");

    assert_eq!(_result.group_result[1].score, 0.0);
    assert_eq!(
        _result.group_result[1].run_result[0].status,
        "Runtime Error"
    );
    assert_eq!(_result.group_result[1].run_result[1].status, "");

    Ok(())
}

#[test]
async fn should_run_cpp_sg_skipped() -> GraderResult<()> {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b_SG.cpp"))
        .await
        .unwrap();

    let mut submission =
        Submission::try_from("a_plus_b", "000009", "cpp", &vec![code], MessageSink).await?;
    assert!(submission.compile().await?);

    let _result = submission.run().await?;

    assert_eq!(_result.score, 0.0);

    assert_eq!(_result.group_result[0].score, 0.0);
    assert_eq!(_result.group_result[0].run_result[0].status, "Signal Error");
    assert_eq!(_result.group_result[0].run_result[1].status, "");

    assert_eq!(_result.group_result[1].score, 0.0);
    assert_eq!(_result.group_result[1].run_result[0].status, "Signal Error");
    assert_eq!(_result.group_result[1].run_result[1].status, "");

    Ok(())
}

#[test]
async fn should_run_cpp_with_header_successfully() -> GraderResult<()> {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b_h.cpp"))
        .await
        .unwrap();

    let mut submission =
        Submission::try_from("a_plus_b_h", "000010", "cpp", &vec![code], MessageSink).await?;
    assert!(submission.compile().await?);

    let _result = submission.run().await?;
    assert_eq!(_result.score, 100.0);

    Ok(())
}

#[test]
async fn should_run_python_successfully() -> GraderResult<()> {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b.py"))
        .await
        .unwrap();

    let mut submission =
        Submission::try_from("a_plus_b", "000011", "python", &vec![code], MessageSink).await?;
    assert!(submission.compile().await?);

    let _result = submission.run().await?;
    assert_eq!(_result.score, 100.0);

    Ok(())
}

#[test]
async fn should_run_python_tle_skipped() -> GraderResult<()> {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b_TLE.py"))
        .await
        .unwrap();

    let mut submission =
        Submission::try_from("a_plus_b", "000012", "python", &vec![code], MessageSink).await?;
    assert!(submission.compile().await?);

    let _result = submission.run().await?;

    assert_eq!(_result.score, 0.0);

    assert_eq!(_result.group_result[0].score, 0.0);
    assert_eq!(
        _result.group_result[0].run_result[0].status,
        "Time Limit Exceeded"
    );
    assert_eq!(_result.group_result[0].run_result[1].status, "");

    assert_eq!(_result.group_result[1].score, 0.0);
    assert_eq!(
        _result.group_result[1].run_result[0].status,
        "Time Limit Exceeded"
    );
    assert_eq!(_result.group_result[1].run_result[1].status, "");

    Ok(())
}

#[test]
async fn should_run_python_mle_skipped() -> GraderResult<()> {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b_MLE.py"))
        .await
        .unwrap();

    let mut submission =
        Submission::try_from("a_plus_b", "000013", "python", &vec![code], MessageSink).await?;
    assert!(submission.compile().await?);

    let _result = submission.run().await?;

    assert_eq!(_result.score, 0.0);

    assert_eq!(_result.group_result[0].score, 0.0);
    assert_eq!(
        _result.group_result[0].run_result[0].status,
        "Memory Limit Exceeded"
    );
    assert_eq!(_result.group_result[0].run_result[1].status, "");

    assert_eq!(_result.group_result[1].score, 0.0);
    assert_eq!(
        _result.group_result[1].run_result[0].status,
        "Memory Limit Exceeded"
    );
    assert_eq!(_result.group_result[1].run_result[1].status, "");

    Ok(())
}

#[test]
async fn should_run_python_re_skipped() -> GraderResult<()> {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b_RE.py"))
        .await
        .unwrap();

    let mut submission =
        Submission::try_from("a_plus_b", "000014", "python", &vec![code], MessageSink).await?;
    assert!(submission.compile().await?);

    let _result = submission.run().await?;

    assert_eq!(_result.score, 0.0);

    assert_eq!(_result.group_result[0].score, 0.0);
    assert_eq!(
        _result.group_result[0].run_result[0].status,
        "Runtime Error"
    );
    assert_eq!(_result.group_result[0].run_result[1].status, "");

    assert_eq!(_result.group_result[1].score, 0.0);
    assert_eq!(
        _result.group_result[1].run_result[0].status,
        "Runtime Error"
    );
    assert_eq!(_result.group_result[1].run_result[1].status, "");

    Ok(())
}

#[test]
async fn should_run_rust_successfully() -> GraderResult<()> {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b.rs"))
        .await
        .unwrap();

    let mut submission =
        Submission::try_from("a_plus_b", "000015", "rust", &vec![code], MessageSink).await?;
    assert!(submission.compile().await?);

    let _result = submission.run().await?;
    assert_eq!(_result.score, 100.0);

    Ok(())
}

#[test]
async fn should_run_rust_tle_skipped() -> GraderResult<()> {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b_TLE.rs"))
        .await
        .unwrap();

    let mut submission =
        Submission::try_from("a_plus_b", "000016", "rust", &vec![code], MessageSink).await?;
    assert!(submission.compile().await?);

    let _result = submission.run().await?;

    assert_eq!(_result.score, 0.0);

    assert_eq!(_result.group_result[0].score, 0.0);
    assert_eq!(
        _result.group_result[0].run_result[0].status,
        "Time Limit Exceeded"
    );
    assert_eq!(_result.group_result[0].run_result[1].status, "");

    assert_eq!(_result.group_result[1].score, 0.0);
    assert_eq!(
        _result.group_result[1].run_result[0].status,
        "Time Limit Exceeded"
    );
    assert_eq!(_result.group_result[1].run_result[1].status, "");

    Ok(())
}

#[test]
async fn should_run_rust_mle_skipped() -> GraderResult<()> {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b_MLE.rs"))
        .await
        .unwrap();

    let mut submission =
        Submission::try_from("a_plus_b", "000017", "rust", &vec![code], MessageSink).await?;
    assert!(submission.compile().await?);

    let _result = submission.run().await?;

    assert_eq!(_result.score, 0.0);

    assert_eq!(_result.group_result[0].score, 0.0);
    assert_eq!(
        _result.group_result[0].run_result[0].status,
        "Memory Limit Exceeded"
    );
    assert_eq!(_result.group_result[0].run_result[1].status, "");

    assert_eq!(_result.group_result[1].score, 0.0);
    assert_eq!(
        _result.group_result[1].run_result[0].status,
        "Memory Limit Exceeded"
    );
    assert_eq!(_result.group_result[1].run_result[1].status, "");

    Ok(())
}

#[test]
async fn should_run_rust_re_skipped() -> GraderResult<()> {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b_RE.rs"))
        .await
        .unwrap();

    let mut submission =
        Submission::try_from("a_plus_b", "000018", "rust", &vec![code], MessageSink).await?;
    assert!(submission.compile().await?);

    let _result = submission.run().await?;

    assert_eq!(_result.score, 0.0);

    assert_eq!(_result.group_result[0].score, 0.0);
    assert_eq!(
        _result.group_result[0].run_result[0].status,
        "Runtime Error"
    );
    assert_eq!(_result.group_result[0].run_result[1].status, "");

    assert_eq!(_result.group_result[1].score, 0.0);
    assert_eq!(
        _result.group_result[1].run_result[0].status,
        "Runtime Error"
    );
    assert_eq!(_result.group_result[1].run_result[1].status, "");

    Ok(())
}

#[test]
async fn should_run_rust_sg_skipped() -> GraderResult<()> {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b_SG.rs"))
        .await
        .unwrap();

    let mut submission =
        Submission::try_from("a_plus_b", "000019", "rust", &vec![code], MessageSink).await?;
    assert!(submission.compile().await?);

    let _result = submission.run().await?;

    assert_eq!(_result.score, 0.0);

    assert_eq!(_result.group_result[0].score, 0.0);
    assert_eq!(_result.group_result[0].run_result[0].status, "Signal Error");
    assert_eq!(_result.group_result[0].run_result[1].status, "");

    assert_eq!(_result.group_result[1].score, 0.0);
    assert_eq!(_result.group_result[1].run_result[0].status, "Signal Error");
    assert_eq!(_result.group_result[1].run_result[1].status, "");

    Ok(())
}

#[test]
async fn should_compile_go_successfully() -> GraderResult<()> {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b.go"))
        .await
        .unwrap();

    let mut submission =
        Submission::try_from("a_plus_b", "000020", "go", &vec![code], MessageSink).await?;
    assert!(submission.compile().await?);

    Ok(())
}

#[test]
async fn should_run_go_successfully() -> GraderResult<()> {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b.go"))
        .await
        .unwrap();

    let mut submission =
        Submission::try_from("a_plus_b", "000021", "go", &vec![code], MessageSink).await?;
    assert!(submission.compile().await?);

    let _result = submission.run().await?;
    assert_eq!(_result.score, 100.0);

    Ok(())
}

#[test]
async fn should_compile_java_successfully() -> GraderResult<()> {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b.java"))
        .await
        .unwrap();

    let mut submission =
        Submission::try_from("a_plus_b", "000022", "java", &vec![code], MessageSink).await?;
    assert!(submission.compile().await?);

    Ok(())
}

#[test]
async fn should_run_java_successfully() -> GraderResult<()> {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b.java"))
        .await
        .unwrap();

    let mut submission =
        Submission::try_from("a_plus_b", "000023", "java", &vec![code], MessageSink).await?;
    assert!(submission.compile().await?);

    let _result = submission.run().await?;
    assert_eq!(_result.score, 100.0);

    Ok(())
}

#[test]
async fn should_handle_messaging() -> GraderResult<()> {
    use futures::StreamExt;

    dotenv().ok();

    let (tx, rx) = futures::channel::mpsc::unbounded();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b.cpp"))
        .await
        .unwrap();
    {
        let mut submission =
            Submission::try_from("a_plus_b", "000024", "cpp", &vec![code], tx).await?;
        assert!(submission.compile().await?);

        let _result = submission.run().await?;
        assert_eq!(_result.score, 100.0);
    }

    let msg: Vec<_> = rx.collect().await;

    assert!(matches!(
        msg[0],
        SubmissionMessage::Status(SubmissionStatus::Compiling)
    ));
    assert!(matches!(
        msg[1],
        SubmissionMessage::Status(SubmissionStatus::Compiled)
    ));

    Ok(())
}

#[test]
async fn should_compile_error_cpp() -> GraderResult<()> {
    dotenv().ok();

    let code = "hello(".to_string();

    let mut submission =
        Submission::try_from("a_plus_b", "000025", "cpp", &vec![code], MessageSink).await?;
    let result = submission.compile().await?;

    assert!(result == false);

    Ok(())
}

#[test]
async fn should_compile_error_python() -> GraderResult<()> {
    dotenv().ok();

    let code = "hello(".to_string();

    let mut submission =
        Submission::try_from("a_plus_b", "000026", "python", &vec![code], MessageSink).await?;
    let result = submission.compile().await?;

    assert!(result == false);

    Ok(())
}

#[test]
async fn should_error_when_task_not_found() -> GraderResult<()> {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b.cpp"))
        .await
        .unwrap();

    let submission = Submission::try_from("hello", "000027", "cpp", &vec![code], MessageSink).await;

    let error_msg = submission.unwrap_err();

    assert_eq!(error_msg, GraderError::TaskNotFound {});

    Ok(())
}
