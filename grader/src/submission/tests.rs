use super::*;

use crate::errors::GraderResult;
use crate::utils::tests::get_example_dir;
use dotenv::dotenv;
use std::fs;

#[test]
fn should_complete_initialize_submission() {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b.cpp")).unwrap();

    let _submission = Submission::from("a_plus_b", "000000", "cpp", &[code], None);
}

#[test]
fn should_compile_cpp_successfully() -> GraderResult<()> {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b.cpp")).unwrap();

    let mut submission = Submission::from("a_plus_b", "000001", "cpp", &vec![code], None)?;
    assert!(submission.compile()?);

    Ok(())
}

#[test]
fn should_compile_python_successfully() -> GraderResult<()> {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b.py")).unwrap();

    let mut submission = Submission::from("a_plus_b", "000002", "python", &vec![code], None)?;
    assert!(submission.compile()?);

    Ok(())
}

#[test]
fn should_compile_rust_successfully() -> GraderResult<()> {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b.rs")).unwrap();

    let mut submission = Submission::from("a_plus_b", "000003", "rust", &vec![code], None)?;
    assert!(submission.compile()?);

    Ok(())
}

#[test]
fn should_remove_tmp_dir_after_out_of_scope() -> GraderResult<()> {
    dotenv().ok();

    let tmp_path;
    {
        let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b.cpp")).unwrap();

        let mut submission = Submission::from("a_plus_b", "000004", "cpp", &vec![code], None)?;
        assert!(submission.compile()?);
        tmp_path = submission.tmp_path.clone();
    }

    assert!(!tmp_path.exists());

    Ok(())
}

#[test]
fn should_run_cpp_successfully() -> GraderResult<()> {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b.cpp")).unwrap();

    let mut submission = Submission::from("a_plus_b", "000005", "cpp", &vec![code], None)?;
    assert!(submission.compile()?);

    let _result = submission.run()?;
    assert_eq!(_result.score, 100.0);

    Ok(())
}

#[test]
fn should_run_cpp_tle_skipped() -> GraderResult<()> {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b_TLE.cpp")).unwrap();

    let mut submission = Submission::from("a_plus_b", "000006", "cpp", &vec![code], None)?;
    assert!(submission.compile()?);

    let _result = submission.run()?;

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
fn should_run_cpp_mle_skipped() -> GraderResult<()> {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b_MLE.cpp")).unwrap();

    let mut submission = Submission::from("a_plus_b", "000007", "cpp", &vec![code], None)?;
    assert!(submission.compile()?);

    let _result = submission.run()?;

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
fn should_run_cpp_re_skipped() -> GraderResult<()> {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b_RE.cpp")).unwrap();

    let mut submission = Submission::from("a_plus_b", "000008", "cpp", &vec![code], None)?;
    assert!(submission.compile()?);

    let _result = submission.run()?;

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
fn should_run_cpp_sg_skipped() -> GraderResult<()> {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b_SG.cpp")).unwrap();

    let mut submission = Submission::from("a_plus_b", "000009", "cpp", &vec![code], None)?;
    assert!(submission.compile()?);

    let _result = submission.run()?;

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
fn should_run_cpp_with_header_successfully() -> GraderResult<()> {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b_h.cpp")).unwrap();

    let mut submission = Submission::from("a_plus_b_h", "000010", "cpp", &vec![code], None)?;
    assert!(submission.compile()?);

    let _result = submission.run()?;
    assert_eq!(_result.score, 100.0);

    Ok(())
}

#[test]
fn should_run_python_successfully() -> GraderResult<()> {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b.py")).unwrap();

    let mut submission = Submission::from("a_plus_b", "000011", "python", &vec![code], None)?;
    assert!(submission.compile()?);

    let _result = submission.run()?;
    assert_eq!(_result.score, 100.0);

    Ok(())
}

#[test]
fn should_run_python_tle_skipped() -> GraderResult<()> {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b_TLE.py")).unwrap();

    let mut submission = Submission::from("a_plus_b", "000012", "python", &vec![code], None)?;
    assert!(submission.compile()?);

    let _result = submission.run()?;

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
fn should_run_python_mle_skipped() -> GraderResult<()> {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b_MLE.py")).unwrap();

    let mut submission = Submission::from("a_plus_b", "000013", "python", &vec![code], None)?;
    assert!(submission.compile()?);

    let _result = submission.run()?;

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
fn should_run_python_re_skipped() -> GraderResult<()> {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b_RE.py")).unwrap();

    let mut submission = Submission::from("a_plus_b", "000014", "python", &vec![code], None)?;
    assert!(submission.compile()?);

    let _result = submission.run()?;

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
fn should_run_rust_successfully() -> GraderResult<()> {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b.rs")).unwrap();

    let mut submission = Submission::from("a_plus_b", "000015", "rust", &vec![code], None)?;
    assert!(submission.compile()?);

    let _result = submission.run()?;
    assert_eq!(_result.score, 100.0);

    Ok(())
}

#[test]
fn should_run_rust_tle_skipped() -> GraderResult<()> {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b_TLE.rs")).unwrap();

    let mut submission = Submission::from("a_plus_b", "000016", "rust", &vec![code], None)?;
    assert!(submission.compile()?);

    let _result = submission.run()?;

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
fn should_run_rust_mle_skipped() -> GraderResult<()> {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b_MLE.rs")).unwrap();

    let mut submission = Submission::from("a_plus_b", "000017", "rust", &vec![code], None)?;
    assert!(submission.compile()?);

    let _result = submission.run()?;

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
fn should_run_rust_re_skipped() -> GraderResult<()> {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b_RE.rs")).unwrap();

    let mut submission = Submission::from("a_plus_b", "000018", "rust", &vec![code], None)?;
    assert!(submission.compile()?);

    let _result = submission.run()?;

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
fn should_run_rust_sg_skipped() -> GraderResult<()> {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b_SG.rs")).unwrap();

    let mut submission = Submission::from("a_plus_b", "000019", "rust", &vec![code], None)?;
    assert!(submission.compile()?);

    let _result = submission.run()?;

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
fn should_compile_go_successfully() -> GraderResult<()> {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b.go")).unwrap();

    let mut submission = Submission::from("a_plus_b", "000020", "go", &vec![code], None)?;
    assert!(submission.compile()?);

    Ok(())
}

#[test]
fn should_run_go_successfully() -> GraderResult<()> {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b.go")).unwrap();

    let mut submission = Submission::from("a_plus_b", "000021", "go", &vec![code], None)?;
    assert!(submission.compile()?);

    let _result = submission.run()?;
    assert_eq!(_result.score, 100.0);

    Ok(())
}

#[test]
fn should_compile_java_successfully() -> GraderResult<()> {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b.java")).unwrap();

    let mut submission = Submission::from("a_plus_b", "000022", "java", &vec![code], None)?;
    assert!(submission.compile()?);

    Ok(())
}

#[test]
fn should_run_java_successfully() -> GraderResult<()> {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b.java")).unwrap();

    let mut submission = Submission::from("a_plus_b", "000023", "java", &vec![code], None)?;
    assert!(submission.compile()?);

    let _result = submission.run()?;
    assert_eq!(_result.score, 100.0);

    Ok(())
}

#[test]
fn should_handle_messaging() -> GraderResult<()> {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b.cpp")).unwrap();
    let mut v: Vec<SubmissionMessage> = Vec::new();
    {
        let mut submission = Submission::from(
            "a_plus_b",
            "000024",
            "cpp",
            &vec![code],
            Some(Box::new(|msg| {
                v.push(msg);
            })),
        )?;
        assert!(submission.compile()?);

        let _result = submission.run()?;
        assert_eq!(_result.score, 100.0);
    }

    Ok(())
}

#[test]
fn should_compile_error_cpp() -> GraderResult<()> {
    dotenv().ok();

    let code = "hello(".to_string();

    let mut submission = Submission::from("a_plus_b", "000025", "cpp", &vec![code], None)?;
    let result = submission.compile()?;

    assert!(result == false);

    Ok(())
}

#[test]
fn should_compile_error_python() -> GraderResult<()> {
    dotenv().ok();

    let code = "hello(".to_string();

    let mut submission = Submission::from("a_plus_b", "000026", "python", &vec![code], None)?;
    let result = submission.compile()?;

    assert!(result == false);

    Ok(())
}

#[test]
fn should_error_when_task_not_found() -> GraderResult<()> {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b.cpp")).unwrap();

    let submission = Submission::from("hello", "000027", "cpp", &vec![code], None);

    let error_msg = submission.unwrap_err();

    assert_eq!(error_msg, GraderError::TaskNotFound {});

    Ok(())
}
