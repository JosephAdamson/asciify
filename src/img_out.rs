use std::fs::*;
use std::io::Write;
use crate::convert_img::generate_img;
use std::path::PathBuf;


/// Save asciified img to file
pub fn save(imgs: Vec<String>, file_name: String) {
    println!("{:?}", imgs);
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_name)
        .expect("Could not write to file");

        for file_path in imgs {
            // I'll just hard code a scale for now
            let ascii_str: String = generate_img(PathBuf::from(file_path), 70);
            file.write_all(ascii_str.as_bytes()).expect("Could  not write to file")
        }
}


#[cfg(test)]
mod test {
    use std::io::Read;
    use std::fs::remove_file;
    use super::*;

    #[test]
    fn write() {
        let dummy_str: Vec<String> = vec![String::from("***************%*************(:->)")];
        let file_path: String = String::from("dummy.txt");
        save(dummy_str.clone(), file_path.clone());

        let mut dummy_file: File = OpenOptions::new()
            .read(true)
            .open(&file_path)
            .expect("File not found");

        let mut temp_buf = String::new();
        dummy_file.read_to_string(&mut temp_buf).unwrap();
        assert_eq!(temp_buf, dummy_str[0]);
        remove_file(file_path).expect("Could not delete temp file");
    }
}