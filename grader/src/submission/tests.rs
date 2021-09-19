use super::*;

use crate::utils::tests::get_example_dir;
use crate::{s, submission};
use dotenv::dotenv;
use std::fs;

#[test]
fn should_complete_initialize_submission() {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b.cpp"))
        .expect("Unable to read code from example dir");

    let mut submission = submission! {
        task_id: s!("a_plus_b"),
        submission_id: s!("000000"),
        language: s!("cpp"),
        code: vec![code.clone()]
    };

    submission.init().expect("Unable to init submission");
}

#[test]
fn should_parse_manifest_successfully() {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b.cpp"))
        .expect("Unable to read code from example dir");

    let mut submission = submission! {
        task_id: s!("a_plus_b"),
        submission_id: s!("000001"),
        language: s!("cpp"),
        code: vec![code.clone()]
    };

    submission.init().expect("Unable to init submission");

    assert_eq!(
        submission.task_manifest["task_id"].as_str().unwrap(),
        "a_plus_b"
    )
}

#[test]
fn should_compile_cpp_successfully() {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b.cpp"))
        .expect("Unable to read code from example dir");

    let mut submission = submission! {
        task_id: s!("a_plus_b"),
        submission_id: s!("000002"),
        language: s!("cpp"),
        code: vec![code.clone()]
    };

    submission.init().expect("Unable to init submission");
    submission.compile().expect("Unable to compile submission");
}

#[test]
fn should_compile_python_successfully() {
    dotenv().ok();

    let code = fs::read_to_string(get_example_dir().join("etc").join("a_plus_b.py"))
        .expect("Unable to read code from example dir");

    let mut submission = submission! {
        task_id: s!("a_plus_b"),
        submission_id: s!("000003"),
        language: s!("python"),
        code: vec![code.clone()]
    };

    submission.init().expect("Unable to init submission");
    submission.compile().expect("Unable to compile submission");
}
