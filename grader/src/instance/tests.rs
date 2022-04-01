use super::*;
use crate::instance;
use crate::utils::tests::{compile_cpp, get_example_dir, get_tmp_path, TempDir};

use dotenv::dotenv;

#[test]
fn should_complete_initialize_instance() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let base_dir = get_example_dir().join("etc");
    let tmp_dir = TempDir::new("initialize_instance");

    compile_cpp(&tmp_dir.0, &base_dir.join("a_plus_b.cpp"));

    let mut instance = instance! {
        time_limit: 1.0,
        memory_limit: 512000,
        bin_path: tmp_dir.0.join("bin"),
        input_path: get_example_dir().join("tasks").join("a_plus_b").join("testcases").join("1.in"),
        output_path: tmp_dir.0.join("output.txt"),
        runner_path: get_example_dir().join("scripts").join("runner_scripts").join("cpp")
    };

    instance.init()?;
    Ok(())
}

#[test]
fn should_error_if_input_path_is_wrong() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let base_dir = get_example_dir().join("etc");
    let tmp_dir = TempDir::new("test_input_path_is_wrong");

    compile_cpp(&tmp_dir.0, &base_dir.join("a_plus_b.cpp"));

    let mut instance = instance! {
        time_limit: 1.0,
        memory_limit: 512000,
        bin_path: tmp_dir.0.join("bin"),
        input_path: base_dir.join("input_wrong_path"),
        runner_path: base_dir.join("run_cpp")
    };

    let _init_result = instance.init()?;
    Ok(())
}

#[test]
#[should_panic]
fn should_error_if_output_path_is_wrong() {
    dotenv().ok();

    let base_dir = get_example_dir().join("etc");
    let tmp_dir = TempDir::new("test_output_path_is_wrong");

    compile_cpp(&tmp_dir.0, &base_dir.join("a_plus_b.cpp"));

    let mut instance = instance! {
        time_limit: 1.0,
        memory_limit: 512000,
        bin_path: tmp_dir.0.join("bin_wrong_path"),
        input_path: get_example_dir().join("tasks").join("a_plus_b").join("testcases").join("1.in"),
        runner_path: base_dir.join("run_cpp")
    };

    let _init_result = instance.init();
}

#[test]
#[should_panic]
fn should_error_if_runner_path_is_wrong() {
    dotenv().ok();
    // get base directory
    let base_dir = get_example_dir().join("etc");
    let tmp_dir = TempDir::new("test_runner_path_is_wrong");

    compile_cpp(&tmp_dir.0, &base_dir.join("a_plus_b.cpp"));

    let mut instance = instance! {
        time_limit: 1.0,
        memory_limit: 512000,
        bin_path: tmp_dir.0.join("bin"),
        input_path: get_example_dir().join("tasks").join("a_plus_b").join("testcases").join("1.in"),
        runner_path: base_dir.join("run_cpp_wrong_path")
    };

    let _init_result = instance.init();
}

#[test]
fn should_read_log_correctly_when_ok() {
    dotenv().ok();

    let test_log = get_example_dir().join("etc").join("log_ok.txt");
    let tmp_log = get_tmp_path().join("test_log_ok.txt");
    fs::copy(&test_log, &tmp_log).unwrap();

    let instance = instance! {
        log_file: tmp_log,
        memory_limit: 4000
    };

    let result = instance.get_result();

    assert_eq!(
        result,
        InstanceResult {
            status: RunVerdict::VerdictOK,
            time_usage: 0.002,
            memory_usage: 480,
        }
    );
}

#[test]
fn should_trigger_when_read_log_with_re() {
    dotenv().ok();

    let test_log = get_example_dir().join("etc").join("log_re.txt");
    let tmp_log = get_tmp_path().join("test_log_re.txt");
    fs::copy(&test_log, &tmp_log).unwrap();

    let instance = instance! {
        log_file: tmp_log,
        memory_limit: 4000
    };

    let result = instance.get_result();

    assert_eq!(
        result,
        InstanceResult {
            status: RunVerdict::VerdictRE,
            time_usage: 0.002,
            memory_usage: 460,
        }
    );
}

#[test]
fn should_trigger_when_read_log_with_to() {
    dotenv().ok();

    let test_log = get_example_dir().join("etc").join("log_to.txt");
    let tmp_log = get_tmp_path().join("test_log_to.txt");
    fs::copy(&test_log, &tmp_log).unwrap();

    let instance = instance! {
        log_file: tmp_log,
        memory_limit: 4000
    };

    let result = instance.get_result();

    assert_eq!(
        result,
        InstanceResult {
            status: RunVerdict::VerdictTLE,
            time_usage: 2.099,
            memory_usage: 448,
        }
    );
}

#[test]
fn should_trigger_when_read_log_with_sg() {
    dotenv().ok();

    let test_log = get_example_dir().join("etc").join("log_sg.txt");
    let tmp_log = get_tmp_path().join("test_log_sg.txt");
    fs::copy(&test_log, &tmp_log).unwrap();

    let instance = instance! {
        log_file: tmp_log,
        memory_limit: 4000
    };

    let result = instance.get_result();

    assert_eq!(
        result,
        InstanceResult {
            status: RunVerdict::VerdictSG,
            time_usage: 0.006,
            memory_usage: 448,
        }
    );
}

#[test]
fn should_trigger_when_read_log_with_xx() {
    dotenv().ok();

    let test_log = get_example_dir().join("etc").join("log_xx.txt");
    let tmp_log = get_tmp_path().join("test_log_xx.txt");
    fs::copy(&test_log, &tmp_log).unwrap();

    let instance = instance! {
        log_file: tmp_log,
        memory_limit: 4000
    };

    let result = instance.get_result();

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

    let test_log = get_example_dir().join("etc").join("log_mle.txt");
    let tmp_log = get_tmp_path().join("test_log_mle.txt");
    fs::copy(&test_log, &tmp_log).unwrap();

    let instance = instance! {
        log_file: tmp_log,
        memory_limit: 1000
    };

    let result = instance.get_result();

    assert_eq!(
        result,
        InstanceResult {
            status: RunVerdict::VerdictMLE,
            time_usage: 0.090,
            memory_usage: 1000,
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
        bin_path: tmp_dir.0.join("bin"),
        input_path: get_example_dir().join("tasks").join("a_plus_b").join("testcases").join("1.in"),
        output_path: tmp_dir.0.join("output.txt"),
        runner_path: get_example_dir().join("scripts").join("runner_scripts").join("cpp")
    };

    instance.init();
    let result = instance.run();

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
        bin_path: tmp_dir.0.join("bin"),
        input_path: get_example_dir().join("tasks").join("a_plus_b").join("testcases").join("1.in"),
        runner_path: get_example_dir().join("scripts").join("runner_scripts").join("cpp")
    };

    instance.init();
    let result = instance.run();

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
        bin_path: tmp_dir.0.join("bin"),
        input_path: get_example_dir().join("tasks").join("a_plus_b").join("testcases").join("1.in"),
        runner_path: get_example_dir().join("scripts").join("runner_scripts").join("cpp")
    };

    instance.init();
    let result = instance.run();

    assert_eq!(result.status, RunVerdict::VerdictRE);
}

#[test]
fn should_get_mle() {
    dotenv().ok();

    let base_dir = get_example_dir().join("etc");
    let tmp_dir = TempDir::new("should_get_mle");

    compile_cpp(&tmp_dir.0, &base_dir.join("a_plus_b_MLE.cpp"));

    let mut instance = instance! {
        time_limit: 1.0,
        memory_limit: 32,
        bin_path: tmp_dir.0.join("bin"),
        input_path: get_example_dir().join("tasks").join("a_plus_b").join("testcases").join("1.in"),
        runner_path: get_example_dir().join("scripts").join("runner_scripts").join("cpp")
    };

    instance.init();
    let result = instance.run();

    assert_eq!(result.status, RunVerdict::VerdictMLE);
}
