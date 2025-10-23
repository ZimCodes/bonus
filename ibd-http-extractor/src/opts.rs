use clap::Parser;
use std::path::PathBuf;
#[derive(Debug, Parser)]
#[command()]
pub struct Opt {
    /// Save Location
    ///
    /// The save location for the extracted links found in the ibd file.
    #[arg(short)]
    pub output: Option<PathBuf>,
    /// ibd file path(s) or directory path
    ///
    /// The file path(s) or a directory path of the .ibd file(s)
    #[arg(value_name = "Path(s)")]
    pub paths: Vec<PathBuf>,
}
impl Opt {
    pub fn new() -> Opt {
        Opt::parse()
    }
}
