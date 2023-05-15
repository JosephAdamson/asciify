use image::{ DynamicImage, GenericImageView, Rgba };
use image::imageops::FilterType;
use std::path::PathBuf;

static ASCII: [char; 32] = ['N', '@', '#', 'W', '$', '9', '8', '7', '6', 
'5', '4', '3', '2', '1', '0', '?', '!', 'a', 'b', 'c', ';', ':', '+', '=', '-', ',',
 ' ', ' ', ' ', ' ', ' ', ' '];


fn normalize_img(img: DynamicImage, pixel_dims: u32,) -> DynamicImage {
    return img.resize(pixel_dims, pixel_dims, FilterType::Gaussian);
}


fn asciify_intensity(intensity: u8) -> char {
    let index = intensity / 8;
    return ASCII[index as usize];
} 


fn generate_img(path: PathBuf, scale: u32) -> String {
    let mut img: DynamicImage = image::open(path).expect("File not Found...");
    
    // normalize img 
    img = normalize_img(img, scale);
    let (width, height) = img.dimensions();
    let mut img_str: String = String::new();

    for y in 0..height {
        for x in 0..width {
            if y % 2 == 0 && x % 1 == 0 {
                let pixel: Rgba<u8> = img.get_pixel(x, y);
                let mut intensity = pixel[0] / 3 + pixel[1] / 3 + pixel[2] / 3;
                if pixel[3] == 0 {
                    intensity = 0;
                }
                let token = asciify_intensity(intensity);
                img_str.push(token);
        }
        }
        if y % 2 == 0 {
            img_str.push('\n');
        }
    }
    return img_str;
}



pub fn print_img_to_console(path_arg: String) {
    let path: PathBuf = PathBuf::from(path_arg);
    let img: String = generate_img(path, 72);
    println!("{}", img);
}


//-----------
// Unit tests
//-----------
#[cfg(test)] 
mod test {

    use image::DynamicImage;
    use std::path::PathBuf;
    use super::*;

    #[test]
    fn normalize() {
        // resize preserves the aspect ratio
        let path = PathBuf::from("ferris.jpg");
        match image::open(path) {
            Err(_) => panic!("Test file could be found."),
            Ok(mut img) => {
                let actual_dims: (u32, u32) = (48, 32);
                img = normalize_img(img, 48);
                let (width, height) = img.dimensions();
                assert!(actual_dims.0 == width && actual_dims.1 == height);
            }
        };
    }

    #[test]
    fn asciify_img() {
        let expected: char = '$';
        let actual: char = asciify_intensity(38);
        assert_eq!(expected, actual);

        let expected: char = ' ';
        let actual: char = asciify_intensity(255);
        assert_eq!(expected, actual);
    }
}