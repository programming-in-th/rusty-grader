use super::*;

use crate::s;
use dotenv::dotenv;

#[test]
fn should_complete_initialize_submission() {
    dotenv().ok();

    let _submission = Submission::new(
        s!("0000"),
        s!("000000"),
        s!("cpp"),
        vec![s!(
            "#include <cstdio> int main() { int a, b; cin >> a >> b; cout << a+b;}"
        )],
    );
}

#[test]
fn should_parse_manifest_successfully() {
    dotenv().ok();

    let submission = Submission::new(
        s!("0000"),
        s!("000001"),
        s!("cpp"),
        vec![s!(
            "#include <cstdio> int main() { int a, b; cin >> a >> b; cout << a+b;}"
        )],
    );

    assert_eq!(
        submission.get_manifest()["task_id"].as_str().unwrap(),
        "0000"
    )
}
