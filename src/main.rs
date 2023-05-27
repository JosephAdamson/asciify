mod convert_img;
mod img_out;
mod args;

use args::AsciiArgs;
use clap::Parser;
use convert_img::{ print_img_to_console };
use img_out::{ save };


fn main() {
    let args: AsciiArgs = AsciiArgs::parse();

    if args.output_path.is_none() {
        for file_path in args.files {
            print_img_to_console(file_path);
        }
    } else {
        save(args.files, args.output_path.expect("Could not write to file"));
    }
    
}
