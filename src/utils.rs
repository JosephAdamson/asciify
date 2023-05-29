use clap::{ Parser };

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

    #[arg(long, short)]
    pub mapping: Option<String>
}


pub struct AsciiToken {
    pub token: char,
    pub rbg: (u8, u8, u8)
}