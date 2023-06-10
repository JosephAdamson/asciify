mod convert_img;
mod img_out;
mod utils;

use utils::{AsciiArgs, is_supported_format, get_file_extension};
use clap::Parser;
use img_out::{output_to_console, save};
//use img_out::save;


fn main() {
    let args: AsciiArgs = AsciiArgs::parse();

    if !args.save {
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
        for path_arg in args.files {
            if is_supported_format(&path_arg) {
                save(path_arg, 
                    args.detailed, 
                    args.color, 
                    args.mapping.clone(), 
                    args.pixel_scale)
            } else {
                // throw error
            }
        }
    }    
}
