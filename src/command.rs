//! # command
//!
//! Runs task commands/scripts.
//!

#[cfg(test)]
#[path = "./command_test.rs"]
mod command_test;

use log::Logger;
use rand::{Rng, thread_rng};
use std::env;
use std::env::current_dir;
use std::fs::{File, create_dir_all, remove_file};
use std::io::Error;
use std::io::prelude::*;
use std::process::{Command, ExitStatus, Stdio};
use types::Step;

/// Validates the exit code code and if not 0 or unable to validate it, panic.
pub fn validate_exit_code(
    exit_status: Result<ExitStatus, Error>,
    logger: &Logger,
) {
    match exit_status {
        Ok(code) => {
            if !code.success() {
                match code.code() {
                    Some(value) => {
                        if value != 0 {
                            logger.error("Error while executing command, exit code: ", &[], Some(value));
                        }
                    }
                    None => logger.error::<()>("Error while executing command, unable to extract exit code.", &[], None),
                }
            }
        }
        Err(error) => logger.error("Error while executing command, error: ", &[], Some(error)),
    }
}

/// Runs the requested script text and panics in case of any script error.
pub fn run_script(
    logger: &Logger,
    script_lines: &Vec<String>,
    validate: bool,
) {
    let name = env!("CARGO_PKG_NAME");
    let file_name: String = thread_rng().gen_ascii_chars().take(10).collect();

    let mut file_path = env::temp_dir();
    file_path.push(name);

    // create parent directory
    match create_dir_all(&file_path) {
        Ok(_) => logger.verbose::<()>("Created temporary directory.", &[], None),
        Err(error) => logger.verbose("Unable to create temporary directory: ", &[&file_path.to_str().unwrap_or("???")], Some(error)),
    };

    file_path.push(file_name);
    if cfg!(windows) {
        file_path.set_extension("bat");
    } else {
        file_path.set_extension("sh");
    };

    let file_path_str = &file_path.to_str().unwrap_or("???");

    logger.verbose::<()>("Creating temporary script file: ", &[&file_path_str], None);

    let mut file = match File::create(&file_path) {
        Err(error) => {
            logger.error("Unable to create script file: ", &[&file_path_str], Some(&error));
            panic!("Unable to create script file, error: {}", error);
        }
        Ok(file) => file,
    };

    let mut text = script_lines.join("\n");

    let cwd_holder = match current_dir() {
        Ok(value) => value,
        Err(error) => {
            logger.error("Unable to get current working directory.", &[], Some(&error));
            panic!("Unable to get current working directory, error: {}", error);
        }
    };

    let cwd = match cwd_holder.to_str() {
        Some(cwd_str) => cwd_str.clone(),
        None => {
            logger.error::<()>("Unable to get current working directory.", &[], None);
            panic!("Unable to get current working directory");
        }
    };

    text.insert_str(0, "\n");
    text.insert_str(0, cwd);
    text.insert_str(0, "cd ");

    match file.write_all(text.as_bytes()) {
        Err(error) => {
            logger.error("Unable to write to script file: ", &[&file_path_str], Some(&error));
            panic!("Unable to write to script file, error: {}", error);
        }
        Ok(_) => logger.verbose::<()>("Written script file text:\n", &[&text], None),
    }

    let command = if cfg!(windows) {
        "cmd.exe"
    } else {
        "sh"
    };

    let args_vector = if cfg!(windows) {
        vec!["/C".to_string(), file_path_str.to_string()]
    } else {
        vec![file_path_str.to_string()]
    };

    let args = Some(args_vector);

    run_command(&logger, &command, &args, validate);

    match remove_file(&file_path_str) {
        Ok(_) => logger.verbose::<()>("Temporary file deleted: ", &[&file_path_str], None),
        Err(error) => logger.verbose("Unable to delete temporary file: ", &[&file_path_str], Some(error)),
    };
}

/// Runs the requested command and panics in case of any error.
pub fn run_command(
    logger: &Logger,
    command_string: &str,
    args: &Option<Vec<String>>,
    validate: bool,
) {
    logger.verbose::<()>("Execute Command: ", &[&command_string], None);
    let mut command = Command::new(&command_string);

    match *args {
        Some(ref args_vec) => {
            for arg in args_vec.iter() {
                command.arg(arg);
            }
        }
        None => logger.verbose::<()>("No command args defined.", &[], None),
    };

    command.stdin(Stdio::inherit()).stdout(Stdio::inherit()).stderr(Stdio::inherit());
    logger.info("Execute Command: ", &[], Some(&command));

    let exit_status = command.status();
    if validate {
        validate_exit_code(exit_status, logger);
    }
}

/// Runs the given task command and if not defined, the task script.
pub fn run(
    logger: &Logger,
    step: &Step,
) {
    let validate = !step.config.is_force();

    match step.config.command {
        Some(ref command_string) => {
            run_command(&logger, &command_string, &step.config.args, validate);
        }
        None => {
            match step.config.script {
                Some(ref script) => run_script(&logger, script, validate),
                None => logger.verbose::<()>("No script defined.", &[], None),
            }
        }
    };
}
