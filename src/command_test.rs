use super::*;
use log;
use std::io::ErrorKind;
use types::Task;

#[test]
#[should_panic]
fn validate_exit_code_error() {
    let logger = log::create("error");
    validate_exit_code(Err(Error::new(ErrorKind::Other, "test")), &logger);
}

#[test]
fn run_no_command() {
    let logger = log::create("error");
    let task = Task::new();

    let step = Step { name: "test".to_string(), config: task };

    run(&logger, &step);
}

#[test]
fn run_command() {
    let logger = log::create("error");
    let mut task = Task::new();
    task.command = Some("echo".to_string());

    let step = Step { name: "test".to_string(), config: task };

    run(&logger, &step);
}

#[test]
#[should_panic]
fn run_command_error() {
    let logger = log::create("error");
    let mut task = Task::new();
    task.command = Some("badbadbad".to_string());

    let step = Step { name: "test".to_string(), config: task };

    run(&logger, &step);
}

#[test]
fn run_command_error_force() {
    let logger = log::create("error");
    let mut task = Task::new();
    task.force = Some(true);
    task.command = Some("badbadbad".to_string());

    let step = Step { name: "test".to_string(), config: task };

    run(&logger, &step);
}

#[test]
fn run_script() {
    let logger = log::create("error");
    let mut task = Task::new();
    task.script = Some(vec!["echo 1".to_string()]);

    let step = Step { name: "test".to_string(), config: task };

    run(&logger, &step);
}

#[test]
#[should_panic]
fn run_script_error() {
    let logger = log::create("error");
    let mut task = Task::new();
    task.script = Some(vec!["exit 1".to_string()]);

    let step = Step { name: "test".to_string(), config: task };

    run(&logger, &step);
}

#[test]
fn run_script_error_force() {
    let logger = log::create("error");
    let mut task = Task::new();
    task.force = Some(true);
    task.script = Some(vec!["exit 1".to_string()]);

    let step = Step { name: "test".to_string(), config: task };

    run(&logger, &step);
}
