use super::*;

use log;
use std::{thread, time};
use std::collections::HashMap;
use std::env;

#[test]
fn setup_env_empty() {
    let logger = log::create("error");
    let config = Config { env: HashMap::new(), tasks: HashMap::new() };

    setup_env(&logger, &config, "setup_env_empty1");

    let mut value = env::var("CARGO_MAKE_TASK");
    assert_eq!(value.unwrap(), "setup_env_empty1");

    setup_env(&logger, &config, "setup_env_empty2");

    let delay = time::Duration::from_millis(10);
    thread::sleep(delay);

    value = env::var("CARGO_MAKE_TASK");
    assert_eq!(value.unwrap(), "setup_env_empty2");
}

#[test]
fn setup_env_values() {
    let logger = log::create("error");
    let mut config = Config { env: HashMap::new(), tasks: HashMap::new() };
    config.env.insert("MY_ENV_KEY".to_string(), "MY_ENV_VALUE".to_string());
    config.env.insert("MY_ENV_KEY2".to_string(), "MY_ENV_VALUE2".to_string());

    assert_eq!(env::var("MY_ENV_KEY").unwrap_or("NONE".to_string()), "NONE".to_string());
    assert_eq!(env::var("MY_ENV_KEY2").unwrap_or("NONE".to_string()), "NONE".to_string());

    setup_env(&logger, &config, "set_env_values");

    assert_eq!(env::var("MY_ENV_KEY").unwrap(), "MY_ENV_VALUE");
    assert_eq!(env::var("MY_ENV_KEY2").unwrap(), "MY_ENV_VALUE2");
}

#[test]
fn setup_env_for_crate_load_toml_found() {
    let logger = log::create("error");

    env::set_var("CARGO_MAKE_CRATE_NAME", "EMPTY");
    env::set_var("CARGO_MAKE_CRATE_VERSION", "EMPTY");
    env::set_var("CARGO_MAKE_CRATE_DESCRIPTION", "EMPTY");
    env::set_var("CARGO_MAKE_CRATE_LICENSE", "EMPTY");
    env::set_var("CARGO_MAKE_CRATE_DOCUMENTATION", "EMPTY");
    env::set_var("CARGO_MAKE_CRATE_HOMEPAGE", "EMPTY");
    env::set_var("CARGO_MAKE_CRATE_REPOSITORY", "EMPTY");

    setup_env_for_crate(&logger);

    assert_eq!(env::var("CARGO_MAKE_CRATE_NAME").unwrap(), "cargo-make");
    assert_eq!(env::var("CARGO_MAKE_CRATE_VERSION").unwrap(), env!("CARGO_PKG_VERSION"));
    assert_eq!(env::var("CARGO_MAKE_CRATE_DESCRIPTION").unwrap(), env!("CARGO_PKG_DESCRIPTION"));
    assert_eq!(env::var("CARGO_MAKE_CRATE_LICENSE").unwrap(), "Apache-2.0");
    assert_eq!(env::var("CARGO_MAKE_CRATE_DOCUMENTATION").unwrap(), "https://sagiegurari.github.io/cargo-make");
    assert_eq!(env::var("CARGO_MAKE_CRATE_HOMEPAGE").unwrap(), "https://sagiegurari.github.io/cargo-make");
    assert_eq!(env::var("CARGO_MAKE_CRATE_REPOSITORY").unwrap(), "https://github.com/sagiegurari/cargo-make.git");
}

#[test]
fn setup_env_for_crate_load_toml_not_found_and_cwd() {
    let logger = log::create("error");

    env::set_var("CARGO_MAKE_CRATE_NAME", "EMPTY");
    env::set_var("CARGO_MAKE_CRATE_VERSION", "EMPTY");
    env::set_var("CARGO_MAKE_CRATE_DESCRIPTION", "EMPTY");
    env::set_var("CARGO_MAKE_CRATE_LICENSE", "EMPTY");
    env::set_var("CARGO_MAKE_CRATE_DOCUMENTATION", "EMPTY");
    env::set_var("CARGO_MAKE_CRATE_HOMEPAGE", "EMPTY");
    env::set_var("CARGO_MAKE_CRATE_REPOSITORY", "EMPTY");

    setup_cwd(&logger, Some("examples"));
    setup_env_for_crate(&logger);
    setup_cwd(&logger, Some(".."));

    assert_eq!(env::var("CARGO_MAKE_CRATE_NAME").unwrap(), "EMPTY");
    assert_eq!(env::var("CARGO_MAKE_CRATE_VERSION").unwrap(), "EMPTY");
    assert_eq!(env::var("CARGO_MAKE_CRATE_DESCRIPTION").unwrap(), "EMPTY");
    assert_eq!(env::var("CARGO_MAKE_CRATE_LICENSE").unwrap(), "EMPTY");
    assert_eq!(env::var("CARGO_MAKE_CRATE_DOCUMENTATION").unwrap(), "EMPTY");
    assert_eq!(env::var("CARGO_MAKE_CRATE_HOMEPAGE").unwrap(), "EMPTY");
    assert_eq!(env::var("CARGO_MAKE_CRATE_REPOSITORY").unwrap(), "EMPTY");

    setup_env_for_crate(&logger);

    assert_eq!(env::var("CARGO_MAKE_CRATE_NAME").unwrap(), "cargo-make");
    assert_eq!(env::var("CARGO_MAKE_CRATE_VERSION").unwrap(), env!("CARGO_PKG_VERSION"));
    assert_eq!(env::var("CARGO_MAKE_CRATE_DESCRIPTION").unwrap(), env!("CARGO_PKG_DESCRIPTION"));
    assert_eq!(env::var("CARGO_MAKE_CRATE_LICENSE").unwrap(), "Apache-2.0");
    assert_eq!(env::var("CARGO_MAKE_CRATE_DOCUMENTATION").unwrap(), "https://sagiegurari.github.io/cargo-make");
    assert_eq!(env::var("CARGO_MAKE_CRATE_HOMEPAGE").unwrap(), "https://sagiegurari.github.io/cargo-make");
    assert_eq!(env::var("CARGO_MAKE_CRATE_REPOSITORY").unwrap(), "https://github.com/sagiegurari/cargo-make.git");
}
