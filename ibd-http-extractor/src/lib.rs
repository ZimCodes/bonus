use std::io;
use std::path::PathBuf;

mod file;
mod opts;
pub fn extract_ibd() ->Result<(),io::Error>{
    let opts = opts::Opt::new();
    let save_loc = opts.output.unwrap_or_else(|| PathBuf::from("./ibd_urls"));
    let paths = match opts.paths[0].as_path().is_dir() {
        true => file::FileHandler::get_ibd_files(opts.paths[0].clone()),
        false => opts.paths
    };

    multiple_ibd_files(paths,save_loc)?;

    Ok(())
}
/// Starts extracting multiple ibd files
fn multiple_ibd_files(paths:Vec<PathBuf>,save_loc:PathBuf)->io::Result<()>{
    for opt in paths{
        let file_handler = file::FileHandler::new(opt,&save_loc);
        file_handler.start()?;
    }
    Ok(())
}
