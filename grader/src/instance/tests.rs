use super::*;
use crate::instance;
use crate::utils::{compile_cpp, get_example_dir, get_tmp_path, TempDir};

use dotenv::dotenv;

#[test]
fn should_complete_initialize_instance() {
    dotenv().ok();

    let base_dir = get_example_dir().join("etc");
    let tmp_dir = TempDir::new("initialize_instance");

    compile_cpp(&tmp_dir.0, &base_dir.join("a_plus_b.cpp"));

    let mut instance = instance! {
        time_limit: 1.0,
        memory_limit: 512000,
        bin_path: tmp_dir.0.clone().join("bin"),
        input_path: get_example_dir().join("tasks").join("a_plus_b").join("testcases").join("1.in"),
        output_path: tmp_dir.0.clone().join("output.txt"),
        runner_path: get_example_dir().join("scripts").join("runner_scripts").join("cpp")
    };

    instance.init().expect("Should run init without error");
}

#[test]
fn should_error_if_input_path_is_wrong() {
    dotenv().ok();

    let base_dir = get_example_dir().join("etc");
    let tmp_dir = TempDir::new("test_input_path_is_wrong");

    compile_cpp(&tmp_dir.0, &base_dir.join("a_plus_b.cpp"));

    let mut instance = instance! {
        time_limit: 1.0,
        memory_limit: 512000,
        bin_path: tmp_dir.0.clone().join("bin"),
        input_path: base_dir.join("input_wrong_path"),
        runner_path: base_dir.join("run_cpp")
    };
    let init_result = instance.init();

    assert_eq!(init_result.unwrap_err().kind(), io::ErrorKind::NotFound);
}

#[test]
fn should_error_if_output_path_is_wrong() {
    dotenv().ok();

    let base_dir = get_example_dir().join("etc");
    let tmp_dir = TempDir::new("test_output_path_is_wrong");

    compile_cpp(&tmp_dir.0, &base_dir.join("a_plus_b.cpp"));

    let mut instance = instance! {
        time_limit: 1.0,
        memory_limit: 512000,
        bin_path: tmp_dir.0.clone().join("bin_wrong_path"),
        input_path: get_example_dir().join("tasks").join("a_plus_b").join("testcases").join("1.in"),
        runner_path: base_dir.join("run_cpp")
    };

    let init_result = instance.init();

    assert_eq!(init_result.unwrap_err().kind(), io::ErrorKind::NotFound);
}

#[test]
fn should_error_if_runner_path_is_wrong() {
    dotenv().ok();
    // get base directory
    let base_dir = get_example_dir().join("etc");
    let tmp_dir = TempDir::new("test_runner_path_is_wrong");

    compile_cpp(&tmp_dir.0, &base_dir.join("a_plus_b.cpp"));

    let mut instance = instance! {
        time_limit: 1.0,
        memory_limit: 512000,
        bin_path: tmp_dir.0.clone().join("bin"),
        input_path: get_example_dir().join("tasks").join("a_plus_b").join("testcases").join("1.in"),
        runner_path: base_dir.join("run_cpp_wrong_path")
    };

    let init_result = instance.init();

    assert_eq!(init_result.unwrap_err().kind(), io::ErrorKind::NotFound);
}

#[test]
fn should_read_log_correctly_when_ok() {
    dotenv().ok();

    let test_log = get_example_dir().join("etc").join("log_ok.txt");
    let tmp_log = get_tmp_path().join("test_log_ok.txt");
    fs::copy(&test_log, &tmp_log).expect("Unable to copy log_ok to tmp");

    let instance = instance! {
        log_file: tmp_log,
        memory_limit: 4000
    };

    let result = instance
        .get_result()
        .expect("Should read log without error");

    assert_eq!(
        result,
        InstanceResult {
            status: RunVerdict::VerdictOK,
            time_usage: 0.004,
            memory_usage: 3196,
        }
    );
}

#[test]
fn should_trigger_when_read_log_with_re() {
    dotenv().ok();

    let test_log = get_example_dir().join("etc").join("log_re.txt");
    let tmp_log = get_tmp_path().join("test_log_re.txt");
    fs::copy(&test_log, &tmp_log).expect("Unable to copy log_re to tmp");

    let instance = instance! {
        log_file: tmp_log,
        memory_limit: 4000
    };

    let result = instance
        .get_result()
        .expect("Should read log without error");

    assert_eq!(
        result,
        InstanceResult {
            status: RunVerdict::VerdictRE,
            time_usage: 0.002,
            memory_usage: 3056,
        }
    );
}

#[test]
fn should_trigger_when_read_log_with_to() {
    dotenv().ok();

    let test_log = get_example_dir().join("etc").join("log_to.txt");
    let tmp_log = get_tmp_path().join("test_log_to.txt");
    fs::copy(&test_log, &tmp_log).expect("Unable to copy log_to to tmp");

    let instance = instance! {
        log_file: tmp_log,
        memory_limit: 4000
    };

    let result = instance
        .get_result()
        .expect("Should read log without error");

    assert_eq!(
        result,
        InstanceResult {
            status: RunVerdict::VerdictTLE,
            time_usage: 2.095,
            memory_usage: 3076,
        }
    );
}

#[test]
fn should_trigger_when_read_log_with_sg() {
    dotenv().ok();

    let test_log = get_example_dir().join("etc").join("log_sg.txt");
    let tmp_log = get_tmp_path().join("test_log_sg.txt");
    fs::copy(&test_log, &tmp_log).expect("Unable to copy log_sg to tmp");

    let instance = instance! {
        log_file: tmp_log,
        memory_limit: 4000
    };

    let result = instance
        .get_result()
        .expect("Should read log without error");

    assert_eq!(
        result,
        InstanceResult {
            status: RunVerdict::VerdictSG,
            time_usage: 0.002,
            memory_usage: 3004,
        }
    );
}

#[test]
fn should_trigger_when_read_log_with_xx() {
    dotenv().ok();

    let test_log = get_example_dir().join("etc").join("log_xx.txt");
    let tmp_log = get_tmp_path().join("test_log_xx.txt");
    fs::copy(&test_log, &tmp_log).expect("Unable to copy log_xx to tmp");

    let instance = instance! {
        log_file: tmp_log,
        memory_limit: 4000
    };

    let result = instance
        .get_result()
        .expect("Should read log without error");

    assert_eq!(
        result,
        InstanceResult {
            status: RunVerdict::VerdictXX,
            ..Default::default()
        }
    );
}

#[test]
fn should_trigger_when_read_log_with_mle() {
    dotenv().ok();

    let test_log = get_example_dir().join("etc").join("log_ok.txt");
    let tmp_log = get_tmp_path().join("test_log_mle.txt");
    fs::copy(&test_log, &tmp_log).expect("Unable to copy log_ok to tmp (MLE)");

    let instance = instance! {
        log_file: tmp_log,
        memory_limit: 1000
    };

    let result = instance
        .get_result()
        .expect("Should read log without error");

    assert_eq!(
        result,
        InstanceResult {
            status: RunVerdict::VerdictMLE,
            time_usage: 0.004,
            memory_usage: 3196,
        }
    );
}

#[test]
fn should_get_ok() {
    dotenv().ok();

    let base_dir = get_example_dir().join("etc");
    let tmp_dir = TempDir::new("should_get_ok");
    compile_cpp(&tmp_dir.0, &base_dir.join("a_plus_b.cpp"));

    let mut instance = instance! {
        time_limit: 1.0,
        memory_limit: 512000,
        bin_path: tmp_dir.0.clone().join("bin"),
        input_path: get_example_dir().join("tasks").join("a_plus_b").join("testcases").join("1.in"),
        output_path: tmp_dir.0.clone().join("output.txt"),
        runner_path: get_example_dir().join("scripts").join("runner_scripts").join("cpp")
    };

    instance.init().expect("Should init without error");
    let result = instance.run().expect("Should run without error");

    assert_eq!(result.status, RunVerdict::VerdictOK);
}

#[test]
fn should_get_tle() {
    dotenv().ok();

    let base_dir = get_example_dir().join("etc");
    let tmp_dir = TempDir::new("should_get_tle");

    compile_cpp(&tmp_dir.0, &base_dir.join("a_plus_b_TLE.cpp"));

    let mut instance = instance! {
        time_limit: 0.1,
        memory_limit: 512000,
        bin_path: tmp_dir.0.clone().join("bin"),
        input_path: get_example_dir().join("tasks").join("a_plus_b").join("testcases").join("1.in"),
        runner_path: get_example_dir().join("scripts").join("runner_scripts").join("cpp")
    };

    instance.init().expect("Should init without error");
    let result = instance.run().expect("Should run without error");

    assert_eq!(result.status, RunVerdict::VerdictTLE);
}

#[test]
fn should_get_re() {
    dotenv().ok();

    let base_dir = get_example_dir().join("etc");
    let tmp_dir = TempDir::new("should_get_re");

    compile_cpp(&tmp_dir.0, &base_dir.join("a_plus_b_RE.cpp"));

    let mut instance = instance! {
        time_limit: 1.0,
        memory_limit: 512000,
        bin_path: tmp_dir.0.clone().join("bin"),
        input_path: get_example_dir().join("tasks").join("a_plus_b").join("testcases").join("1.in"),
        runner_path: get_example_dir().join("scripts").join("runner_scripts").join("cpp")
    };

    instance.init().expect("Should init without error");
    let result = instance.run().expect("Should run without error");

    assert_eq!(result.status, RunVerdict::VerdictRE);
}

#[test]
fn should_get_mle() {
    dotenv().ok();

    let base_dir = get_example_dir().join("etc");
    let tmp_dir = TempDir::new("should_get_mle");

    compile_cpp(&tmp_dir.0, &base_dir.join("a_plus_b.cpp"));

    let mut instance = instance! {
        time_limit: 1.0,
        memory_limit: 1,
        bin_path: tmp_dir.0.clone().join("bin"),
        input_path: get_example_dir().join("tasks").join("a_plus_b").join("testcases").join("1.in"),
        runner_path: get_example_dir().join("scripts").join("runner_scripts").join("cpp")
    };

    instance.init().expect("Should init without error");
    let result = instance.run().expect("Should run without error");

    assert_eq!(result.status, RunVerdict::VerdictMLE);
}
