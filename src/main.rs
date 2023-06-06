mod convert_img;
mod img_out;
mod utils;

use utils::{AsciiArgs, is_supported_format, get_file_extension};
use clap::Parser;
use img_out::{output_to_console};
//use img_out::save;


fn main() {
    let args: AsciiArgs = AsciiArgs::parse();

    if args.save_txt.is_none() {
        for path_arg in args.files { 
            output_to_console(
                path_arg, 
                args.pixel_scale, 
                args.detailed, 
                args.color,
                args.mapping.clone()).unwrap();
        }
    } else {
        // check format 
        for file_path in args.files {
            if is_supported_format(&file_path) {
                let ext: &str = get_file_extension(&file_path).expect("Could not read file");
                if ext == "gif" {
                    // save as gif
                } else {
                    // save as png
                }
            } else {
                // throw error
            }
        }
    }    
}
