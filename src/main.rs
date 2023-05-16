mod convert_img;

fn main() {
    let p: String = String::from("big_rust.png");
    convert_img::print_img_to_console(p);
}
