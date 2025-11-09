use config::{Config, ConfigError, Environment, File};
use getopts::{Matches, Options};
use log::info;
use log4rs;
use serde::Deserialize;
use std::ffi::OsString;

/// Struct for the config.
#[allow(unused)]
#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    // Top-level setting
    pub debug_mode: bool,
    pub log_config_path: String,

    // Nested struct for the server settings
    pub server: ServerSettings,
}
/// Struct for the server config.
#[derive(Debug, Deserialize, Clone)]
pub struct ServerSettings {
    pub port: u16,
    pub host: String,
}

impl Settings {
    /// Loads configuration settings from four sources:
    /// 1. Defaults (lowest precedence)
    /// 2. ~/.config/poker/poker.toml file if it exists, or poker.toml in current dir
    /// 3. Environment variables
    /// 4. args (highest precedence)
    #[allow(unused)]
    pub fn load(args: Vec<String>) -> Result<Self, ConfigError> {
        // configure the opts
        let program = args[0].clone();
        let mut opts = Options::new();
        opts.optopt("c", "config", "set the config file location", "PATH");
        opts.optopt("l", "log", "set the log file location", "PATH");
        opts.optopt("n", "host", "set the host name", "NAME");
        opts.optopt("p", "port", "set the port number", "NUMBER");
        opts.optflag("h", "help", "print this help menu");
        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m,
            Err(f) => {
                panic!("{}", f.to_string())
            }
        };
        // If h flag, print usage and exit.
        if matches.opt_present("h") {
            print_usage(&program, opts);
            std::process::exit(0);
        }

        let config_path = get_opt_or_path("c", &matches, ".config/poker/", "poker.toml");
        let log_path = get_opt_or_path("l", &matches, ".config/poker/", "logging_config.yaml");

        // get settings from config file.
        let mut settings: Settings = Config::builder()
            .set_default("debug_mode", false)?
            .set_default("log_config_path", "logging_config.yaml")?
            .set_default("server.port", 3000)?
            .set_default("server.host", "127.0.0.1")?
            .add_source(File::with_name(&config_path).required(false))
            .add_source(
                Environment::with_prefix("POKER")
                    .separator("__")
                    .try_parsing(true),
            )
            .build()?
            .try_deserialize()
            .unwrap();
        if matches.opt_present("l") {
            settings.log_config_path = matches.opt_str("l;").unwrap();
        }
        if matches.opt_present("n") {
            settings.server.host = matches.opt_str("n").unwrap();
        }
        if matches.opt_present("p") {
            settings.server.port = matches.opt_str("p").unwrap().parse().unwrap();
        }
        log4rs::init_file(settings.log_config_path.clone(), Default::default()).unwrap();
        info!("Config path: {}", config_path);
        info!("Log path: {}", log_path);
        Ok(settings)
    }
}
/// Print the usage message.
fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

/// Retrieves an option from matches, or constructs a file path in the home dir,
/// or returns the default if that fails.
fn get_opt_or_path<'a>(
    key: &'a str,
    matches: &Matches,
    dir: &'a str,
    file_name: &'a str,
) -> String {
    if matches.opt_present(key) {
        matches.opt_str(key).unwrap()
    } else {
        if let Some(mut path_buf) = dirs::home_dir() {
            path_buf.push(dir);
            path_buf.push(file_name);
            let path_str: OsString = path_buf.into_os_string();
            path_str.to_str().unwrap().to_string()
        } else {
            file_name.to_string()
        }
    }
}
