use anyhow::{Result, bail};
use core::Slicer;
use slicer::RegionSlicer;
use types::SpenserConfig;

pub fn open_slicer(config: &SpenserConfig) -> Result<Box<dyn Slicer>> {
    match config.slicer.as_str() {
        "builtin:region" => Ok(Box::new(RegionSlicer)),
        other => bail!("unsupported slicer: {other}"),
    }
}
