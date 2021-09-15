use super::*;
use dotenv::dotenv;

#[test]
fn initialize_instance() -> Result<(), InstanceError> {
    dotenv().ok();
    // get base directory
    let base_dir = PathBuf::from(env::current_dir().unwrap())
        .join("tests")
        .join("instance");

    let tmp_dir = PathBuf::from(get_env("TEMPORARY_PATH")?).join("init_instance");

    if !tmp_dir.is_dir() {
        fs::create_dir(&tmp_dir)
            .map_err(|_| InstanceError::PermissionError("Unable to create tmp directory"))?;
    }

    Command::new(&base_dir.join("compile_cpp"))
        .arg(&tmp_dir)
        .arg(&base_dir.join("a_plus_b.cpp"))
        .output()
        .map_err(|_| InstanceError::PermissionError("Unable to compile file"))?;

    let mut instance = Instance {
        bin_path: tmp_dir.join("bin"),
        time_limit: 1.0,
        memory_limit: 512000,
        input_path: base_dir.join("input.txt"),
        output_path: base_dir.join("output.txt"),
        runner_path: base_dir.join("run_cpp"),
        ..Default::default()
    };

    let result = || -> Result<(), InstanceError> {
        instance.init()?;
        instance.run()?;
        Ok(())
    }();

    // clean up
    fs::remove_dir_all(&tmp_dir)
        .map_err(|_| InstanceError::PermissionError("Unable to remove tmp directory"))?;

    instance.cleanup()?;
    result
}

#[test]
fn should_error_if_input_path_is_wrong() -> Result<(), InstanceError> {
    dotenv().ok();
    // get base directory
    let base_dir = PathBuf::from(env::current_dir().unwrap())
        .join("tests")
        .join("instance");

    let tmp_dir = PathBuf::from(get_env("TEMPORARY_PATH")?).join("test_input_path_is_wrong");

    if !tmp_dir.is_dir() {
        fs::create_dir(&tmp_dir)
            .map_err(|_| InstanceError::PermissionError("Unable to create tmp directory"))?;
    }

    // compile cpp first
    Command::new(&base_dir.join("compile_cpp"))
        .arg(&tmp_dir)
        .arg(&base_dir.join("a_plus_b.cpp"))
        .output()
        .map_err(|_| InstanceError::PermissionError("Unable to compile files"))?;

    let mut instance = Instance {
        bin_path: tmp_dir.join("bin"),
        time_limit: 1.0,
        memory_limit: 512000,
        input_path: base_dir.join("input.txt_wrong_path"),
        output_path: base_dir.join("output.txt"),
        runner_path: base_dir.join("run_cpp"),
        ..Default::default()
    };

    let init_result = instance.init();

    fs::remove_dir_all(&tmp_dir)
        .map_err(|_| InstanceError::PermissionError("Unable to remove tmp directory"))?;

    instance.cleanup()?;

    assert_eq!(
        init_result,
        Err(InstanceError::PermissionError(
            "Unable to copy input file into box directory"
        ))
    );
    Ok(())
}

#[test]
fn should_error_if_output_path_is_wrong() -> Result<(), InstanceError> {
    dotenv().ok();
    // get base directory
    let base_dir = PathBuf::from(env::current_dir().unwrap())
        .join("tests")
        .join("instance");

    let tmp_dir = PathBuf::from(get_env("TEMPORARY_PATH")?).join("test_output_path_is_wrong");

    if !tmp_dir.is_dir() {
        fs::create_dir(&tmp_dir)
            .map_err(|_| InstanceError::PermissionError("Unable to create tmp directory"))?;
    }

    // compile cpp first
    Command::new(&base_dir.join("compile_cpp"))
        .arg(&tmp_dir)
        .arg(&base_dir.join("a_plus_b.cpp"))
        .output()
        .map_err(|_| InstanceError::PermissionError("Unable to compile files"))?;

    let mut instance = Instance {
        bin_path: tmp_dir.join("bin_wrong_path"),
        time_limit: 1.0,
        memory_limit: 512000,
        input_path: base_dir.join("input.txt"),
        output_path: base_dir.join("output.txt"),
        runner_path: base_dir.join("run_cpp"),
        ..Default::default()
    };


    let init_result = instance.init();

    fs::remove_dir_all(&tmp_dir)
        .map_err(|_| InstanceError::PermissionError("Unable to remove tmp directory"))?;

    instance.cleanup()?;

    assert_eq!(
        init_result,
        Err(InstanceError::PermissionError(
            "Unable to copy user exec file into box directory"
        ))
    );
    Ok(())
}

#[test]
fn should_error_if_runner_path_is_wrong() -> Result<(), InstanceError> {
    dotenv().ok();
    // get base directory
    let base_dir = PathBuf::from(env::current_dir().unwrap())
        .join("tests")
        .join("instance");

    let tmp_dir = PathBuf::from(get_env("TEMPORARY_PATH")?).join("test_runner_path_is_wrong");

    if !tmp_dir.is_dir() {
        fs::create_dir(&tmp_dir)
            .map_err(|_| InstanceError::PermissionError("Unable to create tmp directory"))?;
    }

    // compile cpp first
    Command::new(&base_dir.join("compile_cpp"))
        .arg(&tmp_dir)
        .arg(&base_dir.join("a_plus_b.cpp"))
        .output()
        .map_err(|_| InstanceError::PermissionError("Unable to compile files"))?;

    let mut instance = Instance {
        bin_path: tmp_dir.join("bin"),
        time_limit: 1.0,
        memory_limit: 512000,
        input_path: base_dir.join("input.txt"),
        output_path: base_dir.join("output.txt"),
        runner_path: base_dir.join("run_cpp_wrong_path"),
        ..Default::default()
    };

    let init_result = instance.init();

    fs::remove_dir_all(&tmp_dir)
        .map_err(|_| InstanceError::PermissionError("Unable to remove tmp directory"))?;

    instance.cleanup()?;

    assert_eq!(
        init_result,
        Err(InstanceError::PermissionError(
            "Unable to copy runner script into box directory"
        ))
    );
    Ok(())
}

#[test]
fn should_read_log_correctly_when_ok() -> Result<(), InstanceError> {
    dotenv().ok();
    // get base directory
    let base_dir = PathBuf::from(env::current_dir().unwrap())
        .join("tests")
        .join("instance");

    let instance = Instance {
        log_file: base_dir.join("log_ok.txt"),
        memory_limit: 4000,
        ..Default::default()
    };

    let result = instance.get_result()?;

    assert_eq!(
        result,
        InstanceResult {
            status: RunVerdict::VerdictOK,
            time_usage: 0.004,
            memory_usage: 3196,
        }
    );
    Ok(())
}



#[test]
fn should_trigger_when_read_log_with_re() -> Result<(), InstanceError>  {
    dotenv().ok();
    // get base directory
    let base_dir = PathBuf::from(env::current_dir().unwrap())
        .join("tests")
        .join("instance");

    let instance = Instance {
        log_file: base_dir.join("log_re.txt"),
        memory_limit: 1000,
        ..Default::default()
    };

    let result = instance.get_result()?;

    assert_eq!(
        result,
        InstanceResult {
            status: RunVerdict::VerdictRE,
            time_usage: 0.002,
            memory_usage: 3056,
        }
    );

    Ok(())
}

#[test]
fn should_trigger_when_read_log_with_to() -> Result<(), InstanceError> {
    dotenv().ok();
    // get base directory
    let base_dir = PathBuf::from(env::current_dir().unwrap())
        .join("tests")
        .join("instance");

    let instance = Instance {
        log_file: base_dir.join("log_to.txt"),
        memory_limit: 1000,
        ..Default::default()
    };

    let result = instance.get_result()?;

    assert_eq!(
        result,
        InstanceResult {
            status: RunVerdict::VerdictTLE,
            time_usage: 2.095,
            memory_usage: 3076,
        }
    );

    Ok(())
}

#[test]
fn should_trigger_when_read_log_with_sg() -> Result<(), InstanceError> {
    dotenv().ok();
    // get base directory
    let base_dir = PathBuf::from(env::current_dir().unwrap())
        .join("tests")
        .join("instance");

    let instance = Instance {
        log_file: base_dir.join("log_sg.txt"),
        memory_limit: 1000,
        ..Default::default()
    };

    let result = instance.get_result()?;

    assert_eq!(
        result,
        InstanceResult {
            status: RunVerdict::VerdictSG,
            time_usage: 0.002,
            memory_usage: 3004,
        }
    );

    Ok(())
}

#[test]
fn should_trigger_when_read_log_with_xx() -> Result<(), InstanceError> {
    dotenv().ok();
    // get base directory
    let base_dir = PathBuf::from(env::current_dir().unwrap())
        .join("tests")
        .join("instance");

    let instance = Instance {
        log_file: base_dir.join("log_xx.txt"),
        memory_limit: 1000,
        ..Default::default()
    };

    let result = instance.get_result()?;

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
fn should_trigger_when_read_log_with_mle() -> Result<(), InstanceError> {
    dotenv().ok();
    // get base directory
    let base_dir = PathBuf::from(env::current_dir().unwrap())
        .join("tests")
        .join("instance");

    let instance = Instance {
        log_file: base_dir.join("log_ok.txt"),
        memory_limit: 1000,
        ..Default::default()
    };

    let result = instance.get_result()?;

    assert_eq!(
        result,
        InstanceResult {
            status: RunVerdict::VerdictMLE,
            time_usage: 0.004,
            memory_usage: 3196,
        }
    );

    Ok(())
}