use clap::{ Parser };
use std::env;

#[derive(Debug, Default, Parser)]
#[command(author="Joe Adamson")] 
#[command(version = "1.0")]
/// Create cool ASCII images from jpg, png and gif files!
 
pub struct AsciiArgs {
    /// File(s) to be converted into ascii art
    pub files: Vec<String>,

    /// Save ascii output to a either a png or gif format depending on the format
    /// of ther original file. Files are saved individually.
    #[arg(long, short)]
    pub save: bool,

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

// encode the dimensions of the original image the pixel belongs to
// for ease of processing file output
#[derive(Debug)]
pub struct AsciiToken {
    pub token: char,
    pub rgb: (u8, u8, u8),
    pub parent_img_width: u32,
    pub parent_img_height: u32
}

#[derive(Debug)]
pub struct AsciiFrame {
    pub frame_tokens: Vec<AsciiToken>,
    pub delay: (u64, u64)
}


/// Returns file extension for a given file
/// 
/// # Arguments
/// 
/// * 'path_arg'  - file path
pub fn get_file_extension(file_path: &String) -> Option<&str> {
    let tokens: Vec<&str> = file_path.split(".").collect();
    if tokens.len() == 1 {
        return None;
    }
    return Some(tokens[tokens.len() - 1]);
}

/// Returns file name for saveed data
/// 
/// * 'path_arg'  - file path
pub fn build_output_file_name(file_path: &String) -> Result<String, &'static str> {
    let dirs: Vec<&str> = file_path.split("/").collect();
    let tokens: Vec<&str> = dirs[dirs.len() -1].split(".").collect();
    if tokens.len() < 2 {
        return Err("Not a parsable file!");
    } else {
        return Ok(format!("asciify-{}.{}", tokens[0], tokens[1]));
    }
}

/// Check the format of a given file is parsable
/// 
/// # Arguments
/// 
/// * 'parth_arg' - file path   
pub fn is_supported_format(file_path: &String) -> bool {
    let last: &str = get_file_extension(file_path).expect("Could not parse file extension");
    if last == "jpg" || last == "png" || last == "gif" {
        return true;
    } 
    return false;
}

/// Returns true if terminal application caller supports truecolor (16 million colors)
pub fn supports_truecolor() -> bool {
    let key: &str = "COLORTERM";
    let val: String = match env::var(key) {
        Ok(colorterm) => colorterm,
        Err(_) => String::from("none")
    };
    if val != "truecolor" {
        return false;
    }
    return true;
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

    #[test]
    fn get_file_extension_test() {
        let dummy: String = String::from("dummy_file22.gif");
        let expected: &str = "gif";
        let actual: &str = get_file_extension(&dummy).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn get_file_extension_test_2() {
        let dummy: String = String::from("dummy_file22.pong.gif");
        let expected: &str = "gif";
        let actual: &str = get_file_extension(&dummy).unwrap();
        assert_eq!(actual, expected);
    }

    // #[test]
    // fn supports_truecolor_test() {
    //     let res: bool = supports_truecolor();
    //     assert!(res);
    // }

    #[test]
    fn build_file_name_test() {
        let dummy: String = String::from("ferris.png");
        let expected: String = String::from("asciify-ferris.png");
        let actual: String = build_output_file_name(&dummy).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn build_file_name_test_2() {
        let dummy: String = String::from("../../assets/mario.png");
        let expected: String = String::from("asciify-mario.png");
        let actual: String = build_output_file_name(&dummy).unwrap();
        assert_eq!(expected, actual);
    }
}