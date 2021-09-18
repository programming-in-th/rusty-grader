use super::*;

use crate::{s, submission};
use dotenv::dotenv;

#[test]
fn should_complete_initialize_submission() {
    dotenv().ok();

    let mut submission = submission! {
        task_id: s!("a_plus_b"),
        submission_id: s!("000000"),
        language: s!("cpp"),
        code: vec![s!(
            "#include <cstdio> int main() { int a, b; cin >> a >> b; cout << a+b;}"
        )]
    };
    
    submission.init().expect("Unable to init submission");
}

#[test]
fn should_parse_manifest_successfully() {
    dotenv().ok();

    let mut submission = submission! {
        task_id: s!("a_plus_b"),
        submission_id: s!("000001"),
        language: s!("cpp"),
        code: vec![s!(
            "#include <cstdio> int main() { int a, b; cin >> a >> b; cout << a+b;}"
        )]
    };

    submission.init().expect("Unable to init submission");

    assert_eq!(
        submission.task_manifest["task_id"].as_str().unwrap(),
        "a_plus_b"
    )
}
