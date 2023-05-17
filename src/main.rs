mod convert_img;
mod img_out;
mod args;

use args::AsciiArgs;
use clap::Parser;

fn main() {
    let args: AsciiArgs = AsciiArgs::parse();
    println!("{:?}", args);
}
