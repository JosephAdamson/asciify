mod convert_img;
mod img_out;
mod utils;

use utils::AsciiArgs;
use clap::Parser;
use convert_img::print_img_to_console;
use img_out::save;


fn main() {
    let args: AsciiArgs = AsciiArgs::parse();

    if args.save_txt.is_none() {
        for file_path in args.files { 
            print_img_to_console(
                file_path,
                args.color, 
                args.detailed, 
                args.mapping.clone(),
                args.pixel_scale
            );
        }
    } else {
        save(args.files, 
            args.save_txt.expect("Could not write to file"),
            args.detailed,
            &None
        );
    }    
}
