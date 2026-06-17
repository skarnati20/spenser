use anyhow::{Result, bail};
use core::Producer;
use git::GitStandardProducer;
use types::SpenserConfig;

pub fn open_producer(config: &SpenserConfig, path: &str) -> Result<Box<dyn Producer>> {
    match config.producer.as_str() {
        "builtin:git-standard" => Ok(Box::new(GitStandardProducer::new(path))),
        other => bail!("unsupported producer: {other}"),
    }
}
