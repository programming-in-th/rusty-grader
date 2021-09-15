use super::*;
use dotenv::dotenv;

struct TempDir(PathBuf);

impl Drop for TempDir {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0)
            .expect("Unable to remove tmp directory");
    }
}

impl TempDir {
    fn new(tmp_name: &'static str) -> Self {
        let tmp_path = PathBuf::from(get_env("TEMPORARY_PATH").unwrap()).join(tmp_name);
        fs::create_dir(&tmp_path).expect("Unable to remove tmp directory");
        Self(tmp_path)
    }
}

fn get_base_dir() -> PathBuf {
    PathBuf::from(env::current_dir().unwrap())
        .join("tests")
        .join("instance")
}

fn compile_cpp(tmp_dir: &PathBuf, prog_file: &PathBuf) {
    Command::new(&get_base_dir().join("compile_cpp"))
        .arg(&tmp_dir)
        .arg(&prog_file)
        .status()
        .expect("Unable to compile file"); 
}


#[test]
fn initialize_instance() -> Result<(), InstanceError> {
    dotenv().ok();
     
    let base_dir = get_base_dir();
    let tmp_dir = TempDir::new("initialize_instance");
    compile_cpp(&tmp_dir.0, &base_dir.join("a_plus_b.cpp"));

    let mut instance = Instance::new(
        1.0,
        512000,
        tmp_dir.join("bin"),
        base_dir.join("input.txt"),
        base_dir.join("run_cpp"),
    );

    instance.init()?;
    instance.run()?;
    
    Ok(())
}

#[test]
fn should_error_if_input_path_is_wrong() -> Result<(), InstanceError> {
    dotenv().ok();
    
    let base_dir = get_base_dir();
    let tmp_dir = TempDir::new("test_input_path_is_wrong");
    compile_cpp(&tmp_dir.0, &base_dir.join("a_plus_b.cpp"));

    let mut instance = Instance::new(
        1.0,
        512000,
        tmp_dir.join("bin"),
        base_dir.join("input_wrong_path"),
        base_dir.join("run_cpp"),
    );

    let init_result = instance.init()?;

    fs::remove_dir_all(&tmp_dir)
        .map_err(|_| InstanceError::PermissionError("Unable to remove tmp directory"));

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
    
    let base_dir = get_base_dir();
    let tmp_dir = TempDir::new("test_output_path_is_wrong");

    if !tmp_dir.is_dir() {
        fs::create_dir(&tmp_dir)
            .map_err(|_| InstanceError::PermissionError("Unable to create tmp directory"));
    }

    compile_cpp(&tmp_dir.0, &base_dir.join("a_plus_b.cpp"));

    let mut instance = Instance::new(
        1.0,
        512000,
        tmp_dir.join("bin_wrong_path"),
        base_dir.join("input.txt"),
        base_dir.join("run_cpp"),
    );

    let init_result = instance.init()?;

    fs::remove_dir_all(&tmp_dir)
        .map_err(|_| InstanceError::PermissionError("Unable to remove tmp directory"));

    assert_eq!(
        init_result,
        Err(InstanceError::PermissionError(
            "Unable to copy user exec file into box directory"
        ))
    );
    
    Ok(())
}

#[test]
fn should_error_if_runner_path_is_wrong() {
    dotenv().ok();
    // get base directory
    let base_dir = get_base_dir();
    let tmp_dir = TempDir::new("test_runner_path_is_wrong");

    if !tmp_dir.is_dir() {
        fs::create_dir(&tmp_dir)
            .map_err(|_| InstanceError::PermissionError("Unable to create tmp directory"));
    }

    compile_cpp(&tmp_dir.0, &base_dir.join("a_plus_b.cpp"));

    let mut instance = Instance::new(
        1.0,
        512000,
        tmp_dir.join("bin"),
        base_dir.join("input.txt"),
        base_dir.join("run_cpp_wrong_path"),
    );

    let init_result = instance.init()?;

    fs::remove_dir_all(&tmp_dir)
        .map_err(|_| InstanceError::PermissionError("Unable to remove tmp directory"));

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
    
    let test_log = get_base_dir().join("log_ok.txt");
    let tmp_log = PathBuf::from(get_env("TEMPORARY_PATH")).join("test_log_ok.txt");
    fs::copy(&test_log, &tmp_log)
        .map_err(|_| InstanceError::PermissionError("Unable to copy log_ok to tmp"));

    let mut instance: Instance = Default::default();
    instance.log_file = tmp_log;
    instance.memory_limit = 4000;

    let result = instance.get_result();

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
fn should_trigger_when_read_log_with_re() -> Result<(), InstanceError> {
    dotenv().ok();
    
    let test_log = get_base_dir().join("log_re.txt");
    let tmp_log = PathBuf::from(get_env("TEMPORARY_PATH")).join("test_log_re.txt");
    fs::copy(&test_log, &tmp_log)
        .map_err(|_| InstanceError::PermissionError("Unable to copy log_re to tmp"));

    let mut instance: Instance = Default::default();
    instance.log_file = tmp_log;
    instance.memory_limit = 4000;

    let result = instance.get_result();

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
    
    let test_log = get_base_dir().join("log_to.txt");
    let tmp_log = PathBuf::from(get_env("TEMPORARY_PATH")).join("test_log_to.txt");
    fs::copy(&test_log, &tmp_log)
        .map_err(|_| InstanceError::PermissionError("Unable to copy log_to to tmp"));

    let mut instance: Instance = Default::default();
    instance.log_file = tmp_log;
    instance.memory_limit = 4000;

    let result = instance.get_result();

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
    
    let test_log = get_base_dir().join("log_sg.txt");
    let tmp_log = PathBuf::from(get_env("TEMPORARY_PATH")).join("test_log_sg.txt");
    fs::copy(&test_log, &tmp_log)
        .map_err(|_| InstanceError::PermissionError("Unable to copy log_sg to tmp"));

    let mut instance: Instance = Default::default();
    instance.log_file = tmp_log;
    instance.memory_limit = 4000;

    let result = instance.get_result();

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
    
    let test_log = get_base_dir().join("log_xx.txt");
    let tmp_log = PathBuf::from(get_env("TEMPORARY_PATH")).join("test_log_xx.txt");
    fs::copy(&test_log, &tmp_log)
        .map_err(|_| InstanceError::PermissionError("Unable to copy log_xx to tmp"));

    let mut instance: Instance = Default::default();
    instance.log_file = tmp_log;
    instance.memory_limit = 4000;

    let result = instance.get_result();

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
    
    let test_log = get_base_dir().join("log_ok.txt");
    let tmp_log = PathBuf::from(get_env("TEMPORARY_PATH")).join("test_log_mle.txt");
    fs::copy(&test_log, &tmp_log)
        .map_err(|_| InstanceError::PermissionError("Unable to copy log_ok to tmp (MLE)"));

    let mut instance: Instance = Default::default();
    instance.log_file = tmp_log;
    instance.memory_limit = 1000;

    let result = instance.get_result();

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

#[test]
fn should_get_tle() -> Result<(), InstanceError> {
    dotenv().ok();
    
    let base_dir = get_base_dir();
    let tmp_dir = TempDir::new("should_get_tle");

    if !tmp_dir.is_dir() {
        fs::create_dir(&tmp_dir)
            .map_err(|_| InstanceError::PermissionError("Unable to create tmp directory"));
    }
    
    compile_cpp(&tmp_dir.0, &base_dir.join("a_plus_b_TLE.cpp"));

    let mut instance = Instance::new(
        1.0,
        512000,
        tmp_dir.join("bin"),
        base_dir.join("input.txt"),
        base_dir.join("run_cpp"),
    );

    instance.init()?;
    instance.run()?;
    let result = instance.get_result()?;

    assert_eq!(result.status, RunVerdict::VerdictTLE);

    Ok(())
}

#[test]
fn should_get_re() -> Result<(), InstanceError> {
    dotenv().ok();
    
    let base_dir = get_base_dir();
    let tmp_dir = TempDir::new("should_get_re");

    if !tmp_dir.is_dir() {
        fs::create_dir(&tmp_dir)
            .map_err(|_| InstanceError::PermissionError("Unable to create tmp directory"));
    }
    
    compile_cpp(&tmp_dir.0, &base_dir.join("a_plus_b_RE.cpp"));

    let mut instance = Instance::new(
        1.0,
        512000,
        tmp_dir.join("bin"),
        base_dir.join("input.txt"),
        base_dir.join("run_cpp"),
    );

    instance.init()?;
    instance.run()?;
    
    let result = instance.get_result()?;

    assert_eq!(result.status, RunVerdict::VerdictRE);

    Ok(())
}

#[test]
fn should_get_mle() -> Result<(), InstanceError> {
    dotenv().ok();
    
    let base_dir = get_base_dir();
    let tmp_dir = TempDir::new("should_get_mle");

    if !tmp_dir.is_dir() {
        fs::create_dir(&tmp_dir)
            .map_err(|_| InstanceError::PermissionError("Unable to create tmp directory"));
    }
    
    compile_cpp(&tmp_dir.0, &base_dir.join("a_plus_b.cpp"));

    let mut instance = Instance::new(
        1.0,
        1,
        tmp_dir.join("bin"),
        base_dir.join("input.txt"),
        base_dir.join("run_cpp"),
    );

    instance.init()?;
    instance.run()?;
    
    let result = instance.get_result()?;

    assert_eq!(result.status, RunVerdict::VerdictMLE);

    Ok(())
}
