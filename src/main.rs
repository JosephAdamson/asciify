use std::path::PathBuf;
mod convert_img;

fn main() {
    let p: String = String::from("ferris.jpg");
    convert_img::print_img_to_console(p);
}
