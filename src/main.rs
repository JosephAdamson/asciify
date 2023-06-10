mod convert_img;
mod img_out;
mod utils;
use utils::{AsciiArgs, is_supported_format};
use clap::Parser;
use img_out::{output_to_console, save};

fn main() {
    let args: AsciiArgs = AsciiArgs::parse();

    if !args.save {
        for path_arg in args.files { 
            output_to_console(
                path_arg, 
                args.pixel_scale, 
                args.detailed, 
                args.color,
                args.mapping.clone()).expect("Could not output to console");
        }
    } else {
        // check format 
        for path_arg in args.files {
            if is_supported_format(&path_arg) {
                save(path_arg, 
                    args.detailed, 
                    args.color, 
                    args.mapping.clone(), 
                    args.pixel_scale).expect("Could not save to file");
            } else {
                panic!("Error: format not supported");
            }
        }
    }    
}
