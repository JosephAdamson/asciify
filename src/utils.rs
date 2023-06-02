use clap::{ Parser };
use std::{
    error::Error,
    fmt
};

#[derive(Debug, Default, Parser)]
#[command(author="Joe Adamson")] 
#[command(version = "1.0")]
/// Create cool ASCII images from jpg, png and gif files
pub struct AsciiArgs {
    /// File(s) to be converted into ascii art
    #[arg(long, short, num_args = 1.., value_delimiter = ' ')]
    pub files: Vec<String>,

    /// Save ascii output to a .txt file at the given file path
    #[arg(long, short)]
    pub save_txt: Option<String>,

    /// Print color ascii image(s) to the terminal
    #[arg(long, short)]
    pub color: bool,

    /// Choose to print image composed of a wider array of ascii characters
    /// (70 as opposed to default of 10)
    #[arg(long, short)]
    pub detailed: bool,

    /// Provide a custom mapping of ascii characters used to compose the image
    #[arg(long, short)]
    pub mapping: Option<String>,

    /// Scale factor for output image
    #[arg(long, short)]
    pub pixel_scale: Option<u32>
}


pub struct AsciiToken {
    pub token: char,
    pub rbg: (u8, u8, u8)
}


pub fn is_supported_format(path_arg: &String) -> bool {
    let tokens: Vec<&str> = path_arg.split(".").collect();
    if tokens.len() == 1 {
        return false;
    }
    let last = tokens[tokens.len() - 1];
    if last == "jpg" || last == "png" || last == "gif" {
        return true;
    } 
    return false;
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn supported_format_test() {
        let dummy: String = String::from("dummy_file.png");
        assert!(is_supported_format(&dummy));
    }

    #[test]
    fn supported_format_test_2() {
        let dummy: String = String::from("dummy_file.wav");
        assert_ne!(true, is_supported_format(&dummy));
    }
}