use std::path::PathBuf;
mod convert_img;

fn main() {
    let p: String = String::from("dolphin.jpg");
    convert_img::print_img_to_console(p);
}
