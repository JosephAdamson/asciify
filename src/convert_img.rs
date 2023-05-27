use image::{ 
    DynamicImage, 
    GenericImageView, 
    Rgba 
};
use image::imageops::FilterType;
use std::path::PathBuf;
use std::io::Write;
use termcolor::{ Color, ColorChoice, ColorSpec, StandardStream, WriteColor };
use crate::utils::AsciiToken;


static ASCII: [char; 32] = ['N', '@', '#', 'W', '$', '9', '8', '7', '6', 
'5', '4', '3', '2', '1', '0', '?', '!', 'a', 'b', 'c', ';', ':', '+', '=', '-', ',',
 ' ', ' ', ' ', ' ', ' ', ' '];



/// Returns an descaled Dynamic image
fn normalize_img(img: DynamicImage, pixel_scale: u32,) -> DynamicImage {
    let (width, height) = img.dimensions();
    // if image is smaller than the provided scale we use the original width
    if width < pixel_scale {
        return img.resize(width, height, FilterType::Gaussian);
    } else {
        return img.resize(pixel_scale, pixel_scale, FilterType::Gaussian);
    }
}

/// Returns a char mapping
/// 
/// # Arguments
/// *   'intensity' - pixel intensity
fn asciify_intensity(intensity: i32) -> char {
    if intensity < 0 {
        return ASCII[31];
    } else {
        let index: i32 = intensity / 8;
        return ASCII[index as usize];
    }
} 

/// Returns a String of pixel intensities mapped to char values.
/// 
/// # Arguments
/// *   'path'  - file path to the text file
/// *   'scale' - maximum bound used for width
pub fn generate_img(path: PathBuf, scale: u32) -> Vec<AsciiToken> {
    let mut img: DynamicImage = image::open(path).expect("File not Found...");
    
    img = normalize_img(img, scale);
    let (width, height) = img.dimensions();
    //let mut img_str: String = String::new();
    let mut img_tokens: Vec<AsciiToken> = Vec::new();

    for y in 0..height {
        for x in 0..width {
            if y % 2 == 0 && x % 1 == 0 {
                let pixel: Rgba<u8> = img.get_pixel(x, y);
                let mut intensity: i32 = (pixel[0] / 3 + pixel[1] / 3 + pixel[2] / 3 ) as i32;
                if pixel[3] == 0 {
                    intensity = -1;
                }
                let token: char = asciify_intensity(intensity);
                //img_str.push(token);
                img_tokens.push(AsciiToken { token, rbg: (pixel[0], pixel[1], pixel[2]) });
        }
        }
        if y % 2 == 0 {
            // img_str.push('\n');
            img_tokens.push(AsciiToken {token: '\n', rbg: (0, 0, 0)});
        }
    }
    //return img_str;
    return img_tokens;
}


/// Prints asciified image to the console
/// 
/// # Arguments
/// *   'path_arg' - string representation of file path to the text file
/// *   'color_flag'    - option to print terminal output in color
pub fn print_img_to_console(path_arg: String, color_flag: bool) {
    let path: PathBuf = PathBuf::from(path_arg);
    //let img: String = generate_img(path, 72, color_flag);
    //println!("{}", img);
    let img: Vec<AsciiToken> = generate_img(path, 72);
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    if color_flag {
        for token in img {
            stdout.set_color(ColorSpec::new().set_fg(
                Some(Color::Rgb(token.rbg.0, token.rbg.1, token.rbg.2)))
            ).expect("Failed to set color");
            write!(&mut stdout, "{}", token.token).expect("failed to write");
        }
    } else {
        let img_str: String = img.iter()
                                    .map(|ascii_token|{ascii_token.token})
                                    .collect();
        println!("{}", img_str);
    }
}


//-----------
// Unit tests
//-----------
#[cfg(test)] 
mod test {

    use std::path::PathBuf;
    use super::*;

    #[test]
    fn normalize_test() {
        // resize preserves the aspect ratio
        let path: PathBuf = PathBuf::from("assets/ferris.jpg");
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
    fn asciify_img_test() {
        let expected: char = '$';
        let actual: char = asciify_intensity(38);
        assert_eq!(expected, actual);

        let expected: char = ' ';
        let actual: char = asciify_intensity(255);
        assert_eq!(expected, actual);
    }

    #[test]
    fn generate_img_test() {
        let expected: String = String::from("                                                                        
                                                                        
                                                                        
                              =b   ;0+  -a,                             
                         +0c ,011!?111!?11! ,a?,                        
                     c;  a111111111111111111111c ,c:                    
                     !11111111111111111111111111111b             =,     
      !;         a0??1111111111111111111111111111110??0c      -?1?   =; 
 +0;  !11c       :1111111111111111111111111111111111110=     ;111!  -0? 
,011: c111;   ?0011111111111111111111111111111111111111100!  !111?+a11c 
 c111?!111c   +1111111111111111111111111111111111111111110-  +0111110;  
   a11111! ,ca?1111111111111111111111111111111111111111111?b; =0110c    
      a10,  !11111111111111110= 2@$311110 =9$311111111111111a,b1!,      
       -?10bc01111111111111115; 1NN921147-+$N$21111111111110?10+        
          a1111111111111111116@NNNN81113#NNNN92111111111111111c         
        ,?111133211111111111112697311111257531111111111223331111a       
        ,?11! c4!;?33221111111111114$#@W411111111123441c -3b,?1?,       
          =?1a  0c     +b?1333332222233223333331?c=      ?= b1b         
            =??  =;                                     c  :0+          
              =!:                                         +a            
                -:                                        :             
                                                                        
                                                                        
                                                                        
");
        let path: PathBuf = PathBuf::from("assets/ferris.jpg");
        let res: Vec<AsciiToken> = generate_img(path, 72);
        let actual: String = res.iter().map(|ascii_token| {ascii_token.token}).collect();
        assert_eq!(expected, actual);
    }
}