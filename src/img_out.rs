use std::fs::*;
use std::io::Write;


/// Save asciified img to file
fn save_img(img_str: String, file_name: String) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_name)
        .expect("Could not write to file");

    file.write_all(img_str.as_bytes()).expect("Could not write to file");
}


#[cfg(test)]
mod test {
    use std::io::Read;
    use std::fs::remove_file;
    use super::*;

    #[test]
    fn write() {
        let dummy_str: String = String::from("***************%*************(:->)");
        let file_path: String = String::from("dummy.txt");
        save_img(dummy_str.clone(), file_path.clone());

        let mut dummy_file: File = OpenOptions::new()
            .read(true)
            .open(&file_path)
            .expect("File not found");

        let mut temp_buf = String::new();
        dummy_file.read_to_string(&mut temp_buf).unwrap();
        assert_eq!(temp_buf, dummy_str);
        remove_file(file_path).expect("Could not delete temp file");
    }
}