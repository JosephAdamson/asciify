use crate::utils::{get_file_extension, is_supported_format, AsciiToken};
use image::{
    codecs::gif::GifDecoder, imageops::FilterType, AnimationDecoder, DynamicImage, Frame,
    GenericImageView, Rgba,
};
use std::{fs::{File, OpenOptions}, io::Write, path::PathBuf, thread, time};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

const ASCII_DETAILED: [char; 70] = [
    ' ', '.', '\'', '`', '^', '"', ',', ':', ';', 'I', 'l', '!', 'i', '>', '<', '~', '+', '_', '-',
    '?', ']', '[', '}', '{', '1', ')', '(', '|', '\\', '/', 't', 'f', 'j', 'r', 'x', 'n', 'u', 'v',
    'c', 'z', 'X', 'Y', 'U', 'J', 'C', 'L', 'Q', '0', 'O', 'Z', 'm', 'w', 'q', 'p', 'd', 'b', 'k',
    'h', 'a', 'o', '*', '#', 'M', 'W', '&', '8', '%', 'B', '@', '$',
];

const ASCII_SIMPLE: [char; 10] = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];

const MAX_VALUE: f64 = 255.0;

/// Returns an descaled Dynamic image
///
/// # Arguments
///
/// *   'img'           - a pixel matrix
/// *   'pixel_scale'   - pixel scale factor used to resize the image
fn normalize_img(img: DynamicImage, pixel_scale: u32) -> DynamicImage {
    let (width, height) = img.dimensions();
    // if image is smaller than the provided scale we use the original width
    if width < pixel_scale {
        return img.resize(width, height, FilterType::Gaussian);
    } else {
        return img.resize(pixel_scale, pixel_scale, FilterType::Gaussian);
    }
}

/// Returns a char mapping for a unsigned int value repesenting a pixel instensity
///
/// # Arguments
///
/// *   'intensity'     - pixel intensity
/// *   'detail_flag'   - dictate the amount of ascii characters use
fn asciify_intensity(intensity: u32, ascii_table: &Vec<char>) -> char {
    let index: f64 = (intensity as f64 / MAX_VALUE) * ((ascii_table.len() - 1) as f64);
    return ascii_table[index as usize];
}

/// Convert a DynamicImage's pixel values into a AsciiTokens
///
/// # Arguments
///
/// *   'img'           - Rgba pixel matrix
/// *   'ascii_table'   - char vector of mappable ascii characters
fn convert_img_to_ascii_tokens(img: DynamicImage, ascii_table: &Vec<char>) -> Vec<AsciiToken> {
    let (width, height) = img.dimensions();
    let mut img_tokens: Vec<AsciiToken> = Vec::new();
    for y in 0..height {
        for x in 0..width {
            if y % 2 == 0 && x % 1 == 0 {
                let pixel: Rgba<u8> = img.get_pixel(x, y);
                let mut intensity: u32 = (pixel[0] / 3 + pixel[1] / 3 + pixel[2] / 3) as u32;
                if pixel[3] == 0 {
                    intensity = 0;
                }
                let token: char = asciify_intensity(intensity, &ascii_table);
                img_tokens.push(AsciiToken {
                    token,
                    rbg: (pixel[0], pixel[1], pixel[2]),
                });
            }
        }
        if y % 2 == 0 {
            img_tokens.push(AsciiToken {
                token: '\n',
                rbg: (0, 0, 0),
            });
        }
    }
    return img_tokens;
}

pub fn convert_gif_to_ascii_tokens(
    gif: GifDecoder<File>,
    ascii_table: &Vec<char>,
    pixel_scale: u32
) -> Vec<AsciiToken> {
    let frames: Vec<Frame> = gif
        .into_frames()
        .collect_frames()
        .expect("Error decoding gif");
    let mut tokenized_gif:Vec<AsciiToken> = Vec::new();
    
    for frame in frames {
        let mut img: DynamicImage = DynamicImage::ImageRgba8(frame.into_buffer());
        img = normalize_img(img, pixel_scale);
        let mut ascii_tokens: Vec<AsciiToken> = convert_img_to_ascii_tokens(img, ascii_table);
        tokenized_gif.append(&mut ascii_tokens);

        // might need to add whitespace between frames
    }
    return tokenized_gif;
}

/// Reads file and converts image data into a vector of AsciiToken data.
///
/// # Arguments
///
/// *   'path'  - file path to the text file
/// *   'scale' - maximum bound used for width
/// *   'detail_flag'   - dictate the amount of ascii characters use
pub fn convert_to_ascii_tokens(
    path_arg: String,
    pixel_scale: u32,
    detail_flag: bool,
    mapping: Option<String>,
) -> Result<Vec<AsciiToken>, &'static str> {
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
    if is_supported_format(&path_arg) {
        let ext: &str = get_file_extension(&path_arg).expect("Could not read file path");
        if ext == "gif" {
            let file: File = OpenOptions::new()
                .read(true)
                .open(path_arg)
                .expect("Could not read file");
            let decoder: GifDecoder<File> = 
                GifDecoder::new(file).expect("Decoder could not decode file");
            let img_tokens: Vec<AsciiToken> = convert_gif_to_ascii_tokens(decoder, &ascii_table, pixel_scale);
            return Ok(img_tokens);
        } else {
            let mut img: DynamicImage =
                image::open(PathBuf::from(path_arg)).expect("File not Found...");
            img = normalize_img(img, pixel_scale);
            let img_tokens: Vec<AsciiToken> = convert_img_to_ascii_tokens(img, &ascii_table);
            return Ok(img_tokens);
        }
    } else {
        return Err("File format not supported for file");
    }
}

/// Prints asciified image to the console
///
/// # Arguments
///
/// *   'path_arg' - string representation of file path to the text file
/// *   'color_flag'    - option to print terminal output in color
/// *   'detail_flag'   - dictate the amount of ascii characters use
pub fn print_img_to_console(
    path_arg: String,
    color_flag: bool,
    detail_flag: bool,
    mapping: Option<String>,
    pixel_scale: Option<u32>,
) {
    let scale: u32 = match pixel_scale {
        Some(scale) => scale,
        None => 72,
    };
    let img: Vec<AsciiToken> =
        convert_to_ascii_tokens(path_arg, scale, detail_flag, mapping).unwrap();
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    if color_flag {
        for token in img {
            stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Rgb(
                    token.rbg.0,
                    token.rbg.1,
                    token.rbg.2,
                ))))
                .expect("Failed to set color");
            write!(&mut stdout, "{}", token.token).expect("failed to write");
        }
    } else {
        let img_str: String = img.iter().map(|ascii_token| ascii_token.token).collect();
        println!("{}", img_str);
    }
}

//-----------
// Unit tests
//-----------
#[cfg(test)]
mod test {

    use super::*;
    use std::path::PathBuf;

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
        let expected: String = String::from(
            "@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
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
",
        );
        let path: String = String::from("assets/ferris.jpg");
        let res: Vec<AsciiToken> = convert_to_ascii_tokens(path, 72, false, None).unwrap();
        let actual: String = res.iter().map(|ascii_token| ascii_token.token).collect();
        assert_eq!(expected, actual);
    }

    // use a custom mapping to generate a custom image
    #[test]
    fn mapping_test() {
        let expected: String = String::from(
            "}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}
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
",
        );
        let path: String = String::from("assets/ferris.jpg");
        let mapping: Option<String> = Some(String::from("-}"));
        let res: Vec<AsciiToken> = convert_to_ascii_tokens(path, 72, false, mapping).unwrap();
        let actual: String = res.iter().map(|ascii_token| ascii_token.token).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn scale_test() {
        let expected: String = String::from(
            "@@@@@@@@%%%%@@@@@@@@
@%@@%%#+====+#%%@%%%
***%*=--------=*#+**
%*++----=:-=----=+*%
@%*===---:::--===+%@
@@%%#%%%%%%%%%%%#%@@
@@@@@@@@@@@@@@@@@@@@
",
        );
        let path: String = String::from("assets/ferris.jpg");
        let res: Vec<AsciiToken> = convert_to_ascii_tokens(path, 20, false, None).unwrap();
        let actual: String = res.iter().map(|ascii_token| ascii_token.token).collect();
        assert_eq!(expected, actual);
    }
}
