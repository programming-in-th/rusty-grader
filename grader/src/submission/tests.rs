use super::*;

use crate::s;
use crate::utils::tests::get_example_dir;
use dotenv::dotenv;
use std::fs;

#[test]
fn should_complete_initialize_submission() {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b.cpp")).unwrap();

    let _submission = Submission::from(
        "a_plus_b".to_string(),
        "000000".to_string(),
        "cpp".to_string(),
        &[code],
        None,
    );
}

#[test]
fn should_compile_cpp_successfully() {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b.cpp")).unwrap();

    let mut submission =
        Submission::from(s!("a_plus_b"), s!("000001"), s!("cpp"), &vec![code], None);
    submission.compile();
}

#[test]
fn should_compile_python_successfully() {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b.py")).unwrap();

    let mut submission = Submission::from(
        s!("a_plus_b"),
        s!("000002"),
        s!("python"),
        &vec![code],
        None,
    );
    submission.compile();
}

#[test]
fn should_compile_rust_successfully() {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b.rs")).unwrap();

    let mut submission =
        Submission::from(s!("a_plus_b"), s!("000003"), s!("rust"), &vec![code], None);
    submission.compile();
}

#[test]
fn should_remove_tmp_dir_after_out_of_scope() {
    dotenv().ok();

    let tmp_path;
    {
        let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b.cpp")).unwrap();

        let mut submission =
            Submission::from(s!("a_plus_b"), s!("000004"), s!("cpp"), &vec![code], None);
        submission.compile();
        tmp_path = submission.tmp_path.clone();
    }

    assert!(!tmp_path.exists());
}

#[test]
fn should_run_cpp_successfully() {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b.cpp")).unwrap();

    let mut submission =
        Submission::from(s!("a_plus_b"), s!("000005"), s!("cpp"), &vec![code], None);
    submission.compile();

    let _result = submission.run();
    assert_eq!(_result.score, 100.0);
}

#[test]
fn should_run_cpp_tle_skipped() {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b_TLE.cpp")).unwrap();

    let mut submission =
        Submission::from(s!("a_plus_b"), s!("000006"), s!("cpp"), &vec![code], None);
    submission.compile();

    let _result = submission.run();

    assert_eq!(_result.score, 0.0);

    assert_eq!(_result.group_result[0].score, 0.0);
    assert_eq!(
        _result.group_result[0].run_result[0].status,
        s!("Time Limit Exceeded")
    );
    assert_eq!(_result.group_result[0].run_result[1].status, s!(""));

    assert_eq!(_result.group_result[1].score, 0.0);
    assert_eq!(
        _result.group_result[1].run_result[0].status,
        s!("Time Limit Exceeded")
    );
    assert_eq!(_result.group_result[1].run_result[1].status, s!(""));
}

#[test]
fn should_run_cpp_mle_skipped() {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b_MLE.cpp")).unwrap();

    let mut submission =
        Submission::from(s!("a_plus_b"), s!("000007"), s!("cpp"), &vec![code], None);
    submission.compile();

    let _result = submission.run();

    assert_eq!(_result.score, 0.0);

    assert_eq!(_result.group_result[0].score, 0.0);
    assert_eq!(
        _result.group_result[0].run_result[0].status,
        s!("Memory Limit Exceeded")
    );
    assert_eq!(_result.group_result[0].run_result[1].status, s!(""));

    assert_eq!(_result.group_result[1].score, 0.0);
    assert_eq!(
        _result.group_result[1].run_result[0].status,
        s!("Memory Limit Exceeded")
    );
    assert_eq!(_result.group_result[1].run_result[1].status, s!(""));
}

#[test]
fn should_run_cpp_re_skipped() {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b_RE.cpp")).unwrap();

    let mut submission =
        Submission::from(s!("a_plus_b"), s!("000008"), s!("cpp"), &vec![code], None);
    submission.compile();

    let _result = submission.run();

    assert_eq!(_result.score, 0.0);

    assert_eq!(_result.group_result[0].score, 0.0);
    assert_eq!(
        _result.group_result[0].run_result[0].status,
        s!("Runtime Error")
    );
    assert_eq!(_result.group_result[0].run_result[1].status, s!(""));

    assert_eq!(_result.group_result[1].score, 0.0);
    assert_eq!(
        _result.group_result[1].run_result[0].status,
        s!("Runtime Error")
    );
    assert_eq!(_result.group_result[1].run_result[1].status, s!(""));
}

#[test]
fn should_run_cpp_sg_skipped() {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b_SG.cpp")).unwrap();

    let mut submission =
        Submission::from(s!("a_plus_b"), s!("000009"), s!("cpp"), &vec![code], None);
    submission.compile();

    let _result = submission.run();

    assert_eq!(_result.score, 0.0);

    assert_eq!(_result.group_result[0].score, 0.0);
    assert_eq!(
        _result.group_result[0].run_result[0].status,
        s!("Signal Error")
    );
    assert_eq!(_result.group_result[0].run_result[1].status, s!(""));

    assert_eq!(_result.group_result[1].score, 0.0);
    assert_eq!(
        _result.group_result[1].run_result[0].status,
        s!("Signal Error")
    );
    assert_eq!(_result.group_result[1].run_result[1].status, s!(""));
}

#[test]
fn should_run_cpp_with_header_successfully() {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b_h.cpp")).unwrap();

    let mut submission =
        Submission::from(s!("a_plus_b_h"), s!("000010"), s!("cpp"), &vec![code], None);
    submission.compile();

    let _result = submission.run();
    assert_eq!(_result.score, 100.0);
}

#[test]
fn should_run_python_successfully() {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b.py")).unwrap();

    let mut submission = Submission::from(
        s!("a_plus_b"),
        s!("000011"),
        s!("python"),
        &vec![code],
        None,
    );
    submission.compile();

    let _result = submission.run();
    assert_eq!(_result.score, 100.0);
}

#[test]
fn should_run_python_tle_skipped() {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b_TLE.py")).unwrap();

    let mut submission = Submission::from(
        s!("a_plus_b"),
        s!("000012"),
        s!("python"),
        &vec![code],
        None,
    );
    submission.compile();

    let _result = submission.run();

    assert_eq!(_result.score, 0.0);

    assert_eq!(_result.group_result[0].score, 0.0);
    assert_eq!(
        _result.group_result[0].run_result[0].status,
        s!("Time Limit Exceeded")
    );
    assert_eq!(_result.group_result[0].run_result[1].status, s!(""));

    assert_eq!(_result.group_result[1].score, 0.0);
    assert_eq!(
        _result.group_result[1].run_result[0].status,
        s!("Time Limit Exceeded")
    );
    assert_eq!(_result.group_result[1].run_result[1].status, s!(""));
}

#[test]
fn should_run_python_mle_skipped() {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b_MLE.py")).unwrap();

    let mut submission = Submission::from(
        s!("a_plus_b"),
        s!("000013"),
        s!("python"),
        &vec![code],
        None,
    );
    submission.compile();

    let _result = submission.run();

    assert_eq!(_result.score, 0.0);

    assert_eq!(_result.group_result[0].score, 0.0);
    assert_eq!(
        _result.group_result[0].run_result[0].status,
        s!("Memory Limit Exceeded")
    );
    assert_eq!(_result.group_result[0].run_result[1].status, s!(""));

    assert_eq!(_result.group_result[1].score, 0.0);
    assert_eq!(
        _result.group_result[1].run_result[0].status,
        s!("Memory Limit Exceeded")
    );
    assert_eq!(_result.group_result[1].run_result[1].status, s!(""));
}

#[test]
fn should_run_python_re_skipped() {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b_RE.py")).unwrap();

    let mut submission = Submission::from(
        s!("a_plus_b"),
        s!("000014"),
        s!("python"),
        &vec![code],
        None,
    );
    submission.compile();

    let _result = submission.run();

    assert_eq!(_result.score, 0.0);

    assert_eq!(_result.group_result[0].score, 0.0);
    assert_eq!(
        _result.group_result[0].run_result[0].status,
        s!("Runtime Error")
    );
    assert_eq!(_result.group_result[0].run_result[1].status, s!(""));

    assert_eq!(_result.group_result[1].score, 0.0);
    assert_eq!(
        _result.group_result[1].run_result[0].status,
        s!("Runtime Error")
    );
    assert_eq!(_result.group_result[1].run_result[1].status, s!(""));
}

#[test]
fn should_run_rust_successfully() {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b.rs")).unwrap();

    let mut submission =
        Submission::from(s!("a_plus_b"), s!("000015"), s!("rust"), &vec![code], None);
    submission.compile();

    let _result = submission.run();
    assert_eq!(_result.score, 100.0);
}

#[test]
fn should_run_rust_tle_skipped() {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b_TLE.rs")).unwrap();

    let mut submission =
        Submission::from(s!("a_plus_b"), s!("000016"), s!("rust"), &vec![code], None);
    submission.compile();

    let _result = submission.run();

    assert_eq!(_result.score, 0.0);

    assert_eq!(_result.group_result[0].score, 0.0);
    assert_eq!(
        _result.group_result[0].run_result[0].status,
        s!("Time Limit Exceeded")
    );
    assert_eq!(_result.group_result[0].run_result[1].status, s!(""));

    assert_eq!(_result.group_result[1].score, 0.0);
    assert_eq!(
        _result.group_result[1].run_result[0].status,
        s!("Time Limit Exceeded")
    );
    assert_eq!(_result.group_result[1].run_result[1].status, s!(""));
}

#[test]
fn should_run_rust_mle_skipped() {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b_MLE.rs")).unwrap();

    let mut submission =
        Submission::from(s!("a_plus_b"), s!("000017"), s!("rust"), &vec![code], None);
    submission.compile();

    let _result = submission.run();

    assert_eq!(_result.score, 0.0);

    assert_eq!(_result.group_result[0].score, 0.0);
    assert_eq!(
        _result.group_result[0].run_result[0].status,
        s!("Memory Limit Exceeded")
    );
    assert_eq!(_result.group_result[0].run_result[1].status, s!(""));

    assert_eq!(_result.group_result[1].score, 0.0);
    assert_eq!(
        _result.group_result[1].run_result[0].status,
        s!("Memory Limit Exceeded")
    );
    assert_eq!(_result.group_result[1].run_result[1].status, s!(""));
}

#[test]
fn should_run_rust_re_skipped() {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b_RE.rs")).unwrap();

    let mut submission =
        Submission::from(s!("a_plus_b"), s!("000018"), s!("rust"), &vec![code], None);
    submission.compile();

    let _result = submission.run();

    assert_eq!(_result.score, 0.0);

    assert_eq!(_result.group_result[0].score, 0.0);
    assert_eq!(
        _result.group_result[0].run_result[0].status,
        s!("Runtime Error")
    );
    assert_eq!(_result.group_result[0].run_result[1].status, s!(""));

    assert_eq!(_result.group_result[1].score, 0.0);
    assert_eq!(
        _result.group_result[1].run_result[0].status,
        s!("Runtime Error")
    );
    assert_eq!(_result.group_result[1].run_result[1].status, s!(""));
}

#[test]
fn should_run_rust_sg_skipped() {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b_SG.rs")).unwrap();

    let mut submission =
        Submission::from(s!("a_plus_b"), s!("000019"), s!("rust"), &vec![code], None);
    submission.compile();

    let _result = submission.run();

    assert_eq!(_result.score, 0.0);

    assert_eq!(_result.group_result[0].score, 0.0);
    assert_eq!(
        _result.group_result[0].run_result[0].status,
        s!("Signal Error")
    );
    assert_eq!(_result.group_result[0].run_result[1].status, s!(""));

    assert_eq!(_result.group_result[1].score, 0.0);
    assert_eq!(
        _result.group_result[1].run_result[0].status,
        s!("Signal Error")
    );
    assert_eq!(_result.group_result[1].run_result[1].status, s!(""));
}

#[test]
fn should_compile_go_successfully() {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b.go")).unwrap();

    let mut submission =
        Submission::from(s!("a_plus_b"), s!("000020"), s!("go"), &vec![code], None);
    submission.compile();
}

#[test]
fn should_run_go_successfully() {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b.go")).unwrap();

    let mut submission =
        Submission::from(s!("a_plus_b"), s!("000021"), s!("go"), &vec![code], None);
    submission.compile();

    let _result = submission.run();
    assert_eq!(_result.score, 100.0);
}

#[test]
fn should_compile_java_successfully() {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b.java")).unwrap();

    let mut submission =
        Submission::from(s!("a_plus_b"), s!("000022"), s!("java"), &vec![code], None);
    submission.compile();
}

#[test]
fn should_run_java_successfully() {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b.java")).unwrap();

    let mut submission =
        Submission::from(s!("a_plus_b"), s!("000023"), s!("java"), &vec![code], None);
    submission.compile();

    let _result = submission.run();
    assert_eq!(_result.score, 100.0);
}

#[test]
fn should_handle_messaging() {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b.cpp")).unwrap();
    let mut v: Vec<SubmissionMessage> = Vec::new();
    {
        let mut submission = Submission::from(
            s!("a_plus_b"),
            s!("000024"),
            s!("cpp"),
            &vec![code],
            Some(Box::new(|msg| {
                v.push(msg);
            })),
        );
        submission.compile();

        let _result = submission.run();
        assert_eq!(_result.score, 100.0);
    }
}