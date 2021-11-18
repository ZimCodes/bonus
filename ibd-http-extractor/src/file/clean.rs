use regex;
pub struct Cleaner;

impl Cleaner{
    pub fn new()->Cleaner{
        Cleaner
    }
    ///Clean the text by trimming & removing unicodes
    pub fn clean_word(&self,word:String)->String{
        let trimmed_word = self.trim_empty_spaces(word);
        let unwanted_uni_word = self.remove_unwanted_unicodes(trimmed_word);
        let replace_char_word = self.remove_replacement_char(unwanted_uni_word);
        let word_end = self.end_char(replace_char_word);
        word_end
    }
    /// Reduce large empty space gaps
    fn trim_empty_spaces(&self,word:String)->String{
        let reg = regex::Regex::new(r" {2,}").unwrap();
        let trim_word = reg.replace(word.as_str()," ");
        String::from(trim_word)
    }
    /// Removes the replacement character U+FFFD (�)
    fn remove_replacement_char(&self,word:String) -> String{
        let reg =regex::bytes::Regex::new(r"\u{FFFD}").unwrap();
        let new_word = reg.replace_all(word.as_bytes(),&b""[..]);
        let new_word_str = String::from_utf8_lossy(new_word.as_ref());
        String::from(new_word_str)
    }
    /// Removes the unwanted unicodes U+0000 to U+0020
    fn remove_unwanted_unicodes(&self,word:String) -> String{
        let reg = regex::bytes::Regex::new(r"[\u{0}-\u{20}]").unwrap();
        let new_word = reg.replace_all(word.as_bytes(),&b""[..]);
        let new_word_str = String::from_utf8_lossy(new_word.as_ref());
        String::from(new_word_str)
    }
    /// Removes the 'd' character at the end of each text
    fn end_char(&self,word:String)->String{
        let reg = regex::Regex::new(r"d$").unwrap();
        let new_word = reg.replace(word.as_str(),"");
        let new_word_ref = new_word.as_ref();
        new_word_ref.to_string()
    }
}
