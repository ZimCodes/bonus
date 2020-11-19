use std::path::PathBuf;
use structopt::StructOpt;
#[derive(Debug,StructOpt)]
#[structopt()]
pub struct Opt{
    /// Save Location
    ///
    /// The save location for the extracted links found in the ibd file.
    #[structopt(short)]
    pub output:Option<PathBuf>,
    /// ibd directory path
    ///
    /// The directory where the ibd files are located.
    #[structopt(long = "input",short = "i")]
    pub dir_path:Option<PathBuf>,
    /// ibd file path(s)
    ///
    /// The file path(s) of the .ibd file(s)
    #[structopt(name = "Path(s)",required_unless("dir-path"))]
    pub paths:Vec<PathBuf>,
}
impl Opt{
    pub fn new() ->Opt{
        Opt::from_args()
    }
}