mod convert_img;
mod img_out;
mod utils;

use utils::AsciiArgs;
use clap::Parser;
use convert_img::convert_and_output;
//use img_out::save;


fn main() {
    let args: AsciiArgs = AsciiArgs::parse();

    if args.save_txt.is_none() {
        for file_path in args.files { 
            convert_and_output(
                file_path, 
                args.pixel_scale, 
                args.detailed,
                args.color, 
                args.mapping.clone()).unwrap();
        }
    } else {
        // save(args.files, 
        //     args.save_txt.expect("Could not write to file"),
        //     args.detailed,
        //     &None
        // );
    }    
}
