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


const ASCII_DETAILED: [char; 70] = [
    ' ', '.', '\'', '`', '^', '"', ',', ':', ';', 'I', 'l', '!', 
    'i', '>', '<', '~', '+', '_', '-', '?', ']', '[', '}', '{', 
    '1', ')', '(', '|', '\\', '/', 't', 'f', 'j', 'r', 'x', 'n', 
    'u', 'v', 'c', 'z', 'X', 'Y', 'U', 'J', 'C', 'L', 'Q', '0', 
    'O', 'Z', 'm', 'w', 'q', 'p', 'd', 'b', 'k', 'h', 'a', 'o', 
    '*', '#', 'M', 'W', '&', '8', '%', 'B', '@', '$'
];


const ASCII_SIMPLE: [char; 10] = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];


const MAX_VALUE: f64 = 255.0;


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
/// *   'intensity'     - pixel intensity
/// *   'detail_flag'   - dictate the amount of ascii characters use 
fn asciify_intensity(intensity: i32, ascii_table: &Vec<char>) -> char {
    if intensity < 0 {
        return ASCII_SIMPLE[0];
    } else {
        let index: f64 = (intensity as f64 / MAX_VALUE) * ((ascii_table.len() - 1) as f64); 
        return ascii_table[index as usize];
    }
} 

/// Returns a String of pixel intensities mapped to char values.
/// 
/// # Arguments
/// *   'path'  - file path to the text file
/// *   'scale' - maximum bound used for width
/// *   'detail_flag'   - dictate the amount of ascii characters use 
pub fn generate_img(path: PathBuf, scale: u32, detail_flag: bool, mapping: Option<String>) -> Vec<AsciiToken> {
    let mut img: DynamicImage = image::open(path).expect("File not Found...");
    
    img = normalize_img(img, scale);
    let (width, height) = img.dimensions();
    let mut img_tokens: Vec<AsciiToken> = Vec::new();

    let ascii_table: Vec<char>;
    if mapping.is_none() {
        if detail_flag {
            ascii_table = ASCII_DETAILED.to_vec();
        } else {
            ascii_table = ASCII_SIMPLE.to_vec();
        }
    } else {
        ascii_table = mapping.unwrap().chars().collect();
    }

    for y in 0..height {
        for x in 0..width {
            if y % 2 == 0 && x % 1 == 0 {
                let pixel: Rgba<u8> = img.get_pixel(x, y);
                let mut intensity: i32 = (pixel[0] / 3 + pixel[1] / 3 + pixel[2] / 3 ) as i32;
                if pixel[3] == 0 {
                    intensity = 0;
                }
                let token: char = asciify_intensity(intensity, &ascii_table);
                img_tokens.push(AsciiToken { token, rbg: (pixel[0], pixel[1], pixel[2]) });
        }
        }
        if y % 2 == 0 {
            img_tokens.push(AsciiToken {token: '\n', rbg: (0, 0, 0)});
        }
    }
    return img_tokens;
}


/// Prints asciified image to the console
/// 
/// # Arguments
/// *   'path_arg' - string representation of file path to the text file
/// *   'color_flag'    - option to print terminal output in color
/// *   'detail_flag'   - dictate the amount of ascii characters use 
pub fn print_img_to_console(path_arg: String, color_flag: bool, detail_flag: bool, mapping: Option<String>, pixel_scale: Option<u32>) {
    let path: PathBuf = PathBuf::from(path_arg);
    let scale: u32 = match pixel_scale {
        Some(scale) => scale,
        None => 72
    };
    let img: Vec<AsciiToken> = generate_img(path, scale, detail_flag, mapping);
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
        let expected: char = '.';
        let actual: char = asciify_intensity(38, &ASCII_SIMPLE.to_vec());
        assert_eq!(expected, actual);

        let expected_2: char = '@';
        let actual_2: char = asciify_intensity(255, &ASCII_SIMPLE.to_vec());
        assert_eq!(expected_2, actual_2);

        let expected_3: char = '%';
        let actual_3: char = asciify_intensity(245, &ASCII_DETAILED.to_vec());
        assert_eq!(expected_3, actual_3);
    }

    #[test]
    fn generate_img_test() {
        let expected: String = String::from("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@%%%@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@%%%@%*+#%%+=*%%*+#%@%%%@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@%*=+##=--==---==--=##+=#%@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@%++##=---------------------+%#+*%@@@@@@@@@@@@@@@@@@@
@@@@@%%@@@@@@@@@%%%%%=-----------------------------+%%%%@@@@@@@%%*#%@%%@
@%%%%#=+#%@@@@@@%====-------------------------------==-+%@@@@%*=-=#%%*+%
%*=+%%=--+%%@%%%%+-------------------------------------*%%%@%+---=%%*==#
#=--+%+---+%%#===---------------------------------------===%%=---=*=--+%
%+---==---+%@%*------------------------------------------=*%%*-------+%@
@%#+-----=##++=-------------------------------------------=++#*=--=+%%@@
@@@%%#=-=#%#=-----------------*#- .-----=#*..---------------+#+-=#%%@@@@
@@@@@@%*=-=++----------------:+#-  .---::**. .-------------==-=*%@@@@@@@
@@@@@@@@%#+------------------:     .----     .----------------+#%@@@@@@@
@@@@@@@%#=---------------------:.:-------:::--------------------=%@@@@@@
@@@@@@@%#=--=#+:=+=-----------------.   ---------------+#*-+#=-=#%@@@@@@
@@@@@@@@%%*=-=##=+%%%%#*+=----------------------=+*#%%%%#=*%+-+%%@@@@@@@
@@@@@@@@@@%%*==#%*+%@@@@@@@%%%%%%%%%%%%%%%%%%%%@@@@@@@%#+#%*=*%@@@@@@@@@
@@@@@@@@@@@@%%*=*%%%%@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@%#%%*=#%@@@@@@@@@@
@@@@@@@@@@@@@@%%**%@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@%#*%%@@@@@@@@@@@
@@@@@@@@@@@@@@@@%%%%@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@%%%@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
");
        let path: PathBuf = PathBuf::from("assets/ferris.jpg");
        let res: Vec<AsciiToken> = generate_img(path, 72, false, None);
        let actual: String = res.iter().map(|ascii_token| {ascii_token.token}).collect();
        assert_eq!(expected, actual);
    }

    // use a custom mapping to generate a custom image
    #[test]
    fn mapping_test() {
        let expected: String = String::from("}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}
}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}
}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}---}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}
}}}}}}}}}}}}}}}}}}}}}}}}}---}---------------}---}}}}}}}}}}}}}}}}}}}}}}}}
}}}}}}}}}}}}}}}}}}}}}}}}-------------------------}}}}}}}}}}}}}}}}}}}}}}}
}}}}}}}}}}}}}}}}}}}}---------------------------------}}}}}}}}}}}}}}}}}}}
}}}}}--}}}}}}}}}----------------------------------------}}}}}}}-----}--}
}---------}}}}}}-----------------------------------------}}}}-----------
------------}----------------------------------------------}------------
------------------------------------------------------------------------
------------}----------------------------------------------------------}
}---------------------------------------------------------------------}}
}}}-----------------------------------------------------------------}}}}
}}}}}}-----------------------------------------------------------}}}}}}}
}}}}}}}}---------------------------------------------------------}}}}}}}
}}}}}}}-----------------------------------------------------------}}}}}}
}}}}}}}-----------------------------------------------------------}}}}}}
}}}}}}}}---------------------------------------------------------}}}}}}}
}}}}}}}}}}----------}}}}}}}--------------------}}}}}}}---------}}}}}}}}}
}}}}}}}}}}}}---------}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}--------}}}}}}}}}}
}}}}}}}}}}}}}}-----}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}-----}}}}}}}}}}}
}}}}}}}}}}}}}}}}----}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}---}}}}}}}}}}}}}
}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}
}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}
");
        let path: PathBuf = PathBuf::from("assets/ferris.jpg");
        let mapping: Option<String> = Some(String::from("-}"));
        let res: Vec<AsciiToken> = generate_img(path, 72, false, mapping);
        let actual: String = res.iter().map(|ascii_token| {ascii_token.token}).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn scale_test() {
        let expected: String = String::from("@@@@@@@@%%%%@@@@@@@@
@%@@%%#+====+#%%@%%%
***%*=--------=*#+**
%*++----=:-=----=+*%
@%*===---:::--===+%@
@@%%#%%%%%%%%%%%#%@@
@@@@@@@@@@@@@@@@@@@@
");
        let path: PathBuf = PathBuf::from("assets/ferris.jpg");
        let res: Vec<AsciiToken> = generate_img(path, 20, false, None);
        let actual: String = res.iter().map(|ascii_token| {ascii_token.token}).collect();
        assert_eq!(expected, actual);
    }
}