use super::*;
use crate::errors::{GraderError, GraderResult};
use crate::instance;
use crate::utils::tests::{compile_cpp, get_example_dir, get_tmp_path, TempDir};
use tokio::test;

use dotenv::dotenv;

#[test]
async fn should_complete_initialize_instance() -> GraderResult<()> {
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

    instance.init().await?;
    Ok(())
}

#[test]
async fn should_error_if_input_path_is_wrong() -> GraderResult<()> {
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

    let _init_result = instance.init().await;
    assert_eq!(
        _init_result,
        Err(GraderError::InvalidIo {
            msg: String::from("No such file or directory (os error 2)")
        })
    );

    Ok(())
}

#[test]
async fn should_error_if_output_path_is_wrong() -> GraderResult<()> {
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

    let _init_result = instance.init().await;
    assert_eq!(
        _init_result,
        Err(GraderError::InvalidIo {
            msg: String::from("No such file or directory (os error 2)")
        })
    );

    Ok(())
}

#[test]
async fn should_error_if_runner_path_is_wrong() -> GraderResult<()> {
    dotenv().ok();

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

    let _init_result = instance.init().await;
    assert_eq!(
        _init_result,
        Err(GraderError::InvalidIo {
            msg: String::from("No such file or directory (os error 2)")
        })
    );

    Ok(())
}

#[test]
async fn should_read_log_correctly_when_ok() -> GraderResult<()> {
    dotenv().ok();

    let test_log = get_example_dir().join("etc").join("log_ok.txt");
    let tmp_log = get_tmp_path().join("test_log_ok.txt");
    fs::copy(&test_log, &tmp_log).await.unwrap();

    let instance = instance! {
        log_file: tmp_log,
        memory_limit: 4000
    };

    let result = instance.get_result().await?;

    assert_eq!(
        result,
        InstanceResult {
            status: RunVerdict::VerdictOK,
            time_usage: 0.002,
            memory_usage: 480,
        }
    );
    Ok(())
}

#[test]
async fn should_trigger_when_read_log_with_re() -> GraderResult<()> {
    dotenv().ok();

    let test_log = get_example_dir().join("etc").join("log_re.txt");
    let tmp_log = get_tmp_path().join("test_log_re.txt");
    fs::copy(&test_log, &tmp_log).await.unwrap();

    let instance = instance! {
        log_file: tmp_log,
        memory_limit: 4000
    };

    let result = instance.get_result().await?;

    assert_eq!(
        result,
        InstanceResult {
            status: RunVerdict::VerdictRE,
            time_usage: 0.002,
            memory_usage: 460,
        }
    );
    Ok(())
}

#[test]
async fn should_trigger_when_read_log_with_to() -> GraderResult<()> {
    dotenv().ok();

    let test_log = get_example_dir().join("etc").join("log_to.txt");
    let tmp_log = get_tmp_path().join("test_log_to.txt");
    fs::copy(&test_log, &tmp_log).await.unwrap();

    let instance = instance! {
        log_file: tmp_log,
        memory_limit: 4000
    };

    let result = instance.get_result().await?;

    assert_eq!(
        result,
        InstanceResult {
            status: RunVerdict::VerdictTLE,
            time_usage: 2.099,
            memory_usage: 448,
        }
    );
    Ok(())
}

#[test]
async fn should_trigger_when_read_log_with_sg() -> GraderResult<()> {
    dotenv().ok();

    let test_log = get_example_dir().join("etc").join("log_sg.txt");
    let tmp_log = get_tmp_path().join("test_log_sg.txt");
    fs::copy(&test_log, &tmp_log).await.unwrap();

    let instance = instance! {
        log_file: tmp_log,
        memory_limit: 4000
    };

    let result = instance.get_result().await?;

    assert_eq!(
        result,
        InstanceResult {
            status: RunVerdict::VerdictSG,
            time_usage: 0.006,
            memory_usage: 448,
        }
    );
    Ok(())
}

#[test]
async fn should_trigger_when_read_log_with_xx() -> GraderResult<()> {
    dotenv().ok();

    let test_log = get_example_dir().join("etc").join("log_xx.txt");
    let tmp_log = get_tmp_path().join("test_log_xx.txt");
    fs::copy(&test_log, &tmp_log).await.unwrap();

    let instance = instance! {
        log_file: tmp_log,
        memory_limit: 4000
    };

    let result = instance.get_result().await?;

    assert_eq!(
        result,
        InstanceResult {
            status: RunVerdict::VerdictXX,
            ..Default::default()
        }
    );
    Ok(())
}

#[test]
async fn should_trigger_when_read_log_with_mle() -> GraderResult<()> {
    dotenv().ok();

    let test_log = get_example_dir().join("etc").join("log_mle.txt");
    let tmp_log = get_tmp_path().join("test_log_mle.txt");
    fs::copy(&test_log, &tmp_log).await.unwrap();

    let instance = instance! {
        log_file: tmp_log,
        memory_limit: 1000
    };

    let result = instance.get_result().await?;

    assert_eq!(
        result,
        InstanceResult {
            status: RunVerdict::VerdictMLE,
            time_usage: 0.090,
            memory_usage: 1000,
        }
    );
    Ok(())
}

#[test]
async fn should_get_ok() -> GraderResult<()> {
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

    instance.init().await?;
    let result = instance.run().await?;

    assert_eq!(result.status, RunVerdict::VerdictOK);
    Ok(())
}

#[test]
async fn should_get_tle() -> GraderResult<()> {
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

    instance.init().await?;
    let result = instance.run().await?;

    assert_eq!(result.status, RunVerdict::VerdictTLE);
    Ok(())
}

#[test]
async fn should_get_re() -> GraderResult<()> {
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

    instance.init().await?;
    let result = instance.run().await?;

    assert_eq!(result.status, RunVerdict::VerdictRE);
    Ok(())
}

#[test]
async fn should_get_mle() -> GraderResult<()> {
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

    instance.init().await?;
    let result = instance.run().await?;

    assert_eq!(result.status, RunVerdict::VerdictMLE);
    Ok(())
}
