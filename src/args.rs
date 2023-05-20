use clap::Parser;

#[derive(Debug, Default, Parser)]
#[command(author="Joe Adamson", version, about)]
/// Create cool ASCII images from jpg, png and gif files
pub struct AsciiArgs {
    /// file(s) to be converted into ascii art
    #[arg(short, long)]
    pub files: String,
}