use std::io;
use std::path::PathBuf;

mod file;
mod opts;
pub fn extract_ibd() ->Result<(),io::Error>{
    let opts = opts::Opt::new();
    let save_loc = match opts.output{
        Some(path) => path,
        None => PathBuf::from("./ibd_urls")
    };

    let paths = match opts.dir_path{
        Some(path) => file::FileHandler::get_ibd_files(path),
        None => opts.paths
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
