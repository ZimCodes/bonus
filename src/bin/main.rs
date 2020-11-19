use ibd_http_extractor::extract_ibd;
fn main() {
    match extract_ibd(){
      Ok(()) => println!("Operation Completed!"),
        Err(e) => println!("Operation failed! {}",e.to_string())
    };
}
