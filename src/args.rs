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
    pub output_path: Option<String>
}