use std::fs::*;
use std::io::Write;
use crate::convert_img::generate_img;
use crate::utils::AsciiToken;
use std::path::PathBuf;


/// Save asciified img to file
pub fn save(imgs: Vec<String>, file_name: String, detail_flag: bool, mapping: Option<Vec<char>>) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_name)
        .expect("Could not write to file");

    for file_path in imgs {
        // I'll just hard code a scale for now
        let ascii_str: Vec<AsciiToken> = generate_img(PathBuf::from(file_path), 70, detail_flag, None);

        // for now save non color images
        let img_str: String = ascii_str.iter()
            .map(|ascii_token|{ascii_token.token})
            .collect();
        file.write_all(img_str.as_bytes()).expect("Could not write to file");
    }
}