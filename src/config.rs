use std::fs;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct ConfigFromRead {
  pub port: Option<u16>,
  pub ip: Option<String>,
  pub worker_count: Option<usize>,
  pub shutdown_timeout: Option<u64>
}

#[derive(Deserialize)]
pub struct Config {
  pub port: u16,
  pub ip: String,
  pub worker_count: usize,
  pub shutdown_timeout: u64
}

impl std::fmt::Display for Config {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "\n port: {} \n ip: {} \n worker_count: {} \n shutdown_timeout: {}", self.port, self.ip, self.worker_count, self.shutdown_timeout)
  }
}

static DEFAULT_PORT: u16 = 8080;
static DEFAULT_WORKER_COUNT: usize = 2;
static DEFAULT_SHUTDOWN_TIMEOUT: u64 = 30;
static DEFAULT_IP: &str = "localhost";

pub fn read_config() -> Config {
  let contents = fs::read_to_string("config.toml").expect("Couldn't read config file");

  let config_read: ConfigFromRead = toml::from_str(&contents).expect("Fields missing in config file");
  
  let config = 
    Config { 
      port: if config_read.port == None { DEFAULT_PORT } else { config_read.port.unwrap() },
      ip: if config_read.ip == None { String::from(DEFAULT_IP) } else { config_read.ip.unwrap() },
      worker_count: if config_read.worker_count == None { DEFAULT_WORKER_COUNT } else { config_read.worker_count.unwrap() }, 
      shutdown_timeout: if config_read.shutdown_timeout == None { DEFAULT_SHUTDOWN_TIMEOUT } else { config_read.shutdown_timeout.unwrap() }, 
    };

  println!("Running with settings: {}", config);

  config
}