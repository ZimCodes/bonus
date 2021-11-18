use std::io::prelude::*;
use std::fs;
use std::io;
use std::path::PathBuf;
use regex::Regex;

mod clean;
pub struct FileHandler{
    ibd_path: String,
    save_dir:String
}

impl FileHandler{
    pub fn new(ibd_path:PathBuf,save_dir:&PathBuf) -> FileHandler{
        let ibd = ibd_path.to_str().expect("Cannot change path of ibd to a &str");
        let save = save_dir.to_str().expect("Cannot change save path to a &str");
        FileHandler{
            ibd_path:ibd.to_string(),
            save_dir:save.to_string()
        }
    }
    /// Gets the ibd files from a directory
    pub fn get_ibd_files(dir:PathBuf)->Vec<PathBuf>{
        let mut ibd_paths = Vec::new();
        let contents = fs::read_dir(dir).unwrap();
        //Look through directory
        for content in contents{
            let file = content.expect("Error while looking through directory for ibd files");
            let name_os = file.file_name();
            let name = name_os.to_str().unwrap();

            // retrieve all .ibd files from directory
            if name.ends_with(".ibd"){
                ibd_paths.push(file.path())
            }
        }
        ibd_paths
    }
    /// Starts retrieving urls from ibd file
    pub fn start(&self) ->Result<(),io::Error>{
        let file_bytes = fs::read(&self.ibd_path)?;
        let file_str_vec = self.split_zero_bytes(file_bytes);
        let url_links = self.get_urls(file_str_vec);


        Ok(self.write_to_file(url_links)?)
    }
    /// Create directories to save path if they do not already exist
    fn create_dirs(&self)->io::Result<()>{
        Ok(fs::create_dir_all(&self.save_dir)?)
    }
    /// Writes the URLs to a file
    fn write_to_file(&self,links:Vec<String>) ->Result<(),io::Error>{
        self.create_dirs()?;//create nonexistent dirs(if needed)

        let file_path = format!("{}/{}",self.save_dir,self.get_file_name());
        let mut f = fs::File::create(file_path)?;
        // Write to file
        for link in links{
            f.write(link.as_bytes())?;
        }
        Ok(())
    }
    /// Gets the name of the ibd file
    fn get_file_name(&self)->String{
        let reg = Regex::new(r"(\w+\.ibd)").unwrap();
        let end_reg = Regex::new(r"\.ibd").unwrap();
        let file_captures = reg.captures(&self.ibd_path).unwrap();
        let file = file_captures.get(0).unwrap().as_str();
        let file_name = end_reg.replace(file, ".txt");
        let file_name_ref = file_name.as_ref();
        file_name_ref.to_string()
    }
    /// Remove minor unicode code points 1-12
    fn get_urls(&self,file_str_vec:Vec<String>) -> Vec<String>{
        let mut http_vec = Vec::new();
        let cleaner = clean::Cleaner::new();
        for word in file_str_vec{
            if word.contains("http"){
                let mut new_word = cleaner.clean_word(word);
                new_word = format!("{}\n",new_word);
                http_vec.push(new_word);
            }
        }
        http_vec
    }

    /// Split the bytes
    fn split_zero_bytes(&self,file_bytes:Vec<u8>) -> Vec<String>{
        let filtered_bytes= file_bytes.split(|num| num == &0u8).into_iter();
        let mut bytes = Vec::new();
        for byte in filtered_bytes{
            let str = String::from_utf8_lossy(byte);
            let owned_str = String::from(str);
            bytes.push(owned_str);
        }

        bytes
    }
}
