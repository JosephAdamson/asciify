use crate::utils::{
    get_file_extension, is_supported_format, supports_truecolor, AsciiFrame, AsciiToken,
};
use image::{
    codecs::gif::GifDecoder, imageops::FilterType, AnimationDecoder, DynamicImage, Frame,
    GenericImageView, Rgba,
};
use rgb2ansi256::rgb_to_ansi256;
use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::PathBuf,
    process, thread,
    time::Duration,
};
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
    pixel_scale: u32,
) -> Vec<AsciiFrame> {
    let frames: Vec<Frame> = gif
        .into_frames()
        .collect_frames()
        .expect("Error decoding gif");
    let mut tokenized_gif: Vec<AsciiFrame> = Vec::new();

    for frame in frames {
        let frame_ratio: (u32, u32) = frame.delay().clone().numer_denom_ms();
        let mut img: DynamicImage = DynamicImage::ImageRgba8(frame.into_buffer());
        img = normalize_img(img, pixel_scale);
        let ascii_tokens: Vec<AsciiToken> = convert_img_to_ascii_tokens(img, ascii_table);
        let int_delay: u64 = (frame_ratio.0 / frame_ratio.1) as u64;
        let ascii_frame: AsciiFrame = AsciiFrame {
            frame_tokens: ascii_tokens,
            delay: int_delay,
        };
        tokenized_gif.push(ascii_frame);

        // might need to add whitespace between frames?
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
pub fn convert_and_output(
    path_arg: String,
    pixel_scale: Option<u32>,
    detail_flag: bool,
    color_flag: bool,
    mapping: Option<String>,
) -> Result<(), &'static str> {
    // get char mapping
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

    // check for scale
    let scale: u32 = match pixel_scale {
        Some(scale) => scale,
        None => 72,
    };
    if is_supported_format(&path_arg) {
        let ext: &str = get_file_extension(&path_arg).expect("Could not read file path");
        if ext == "gif" {
            let file: File = OpenOptions::new()
                .read(true)
                .open(path_arg)
                .expect("Could not read file");
            let decoder: GifDecoder<File> =
                GifDecoder::new(file).expect("Decoder could not decode file");
            let img_frames: Vec<AsciiFrame> =
                convert_gif_to_ascii_tokens(decoder, &ascii_table, scale);

            // print gif
            print_gif_to_console(img_frames, color_flag);
            return Ok(());
        } else {
            let mut img: DynamicImage =
                image::open(PathBuf::from(path_arg)).expect("File not Found...");
            img = normalize_img(img, scale);
            let img_tokens: Vec<AsciiToken> = convert_img_to_ascii_tokens(img, &ascii_table);

            // print img
            print_img_to_console(img_tokens, color_flag);
            return Ok(());
        }
    } else {
        return Err("File format not supported for file");
    }
}

fn write_color_output(tokens: Vec<AsciiToken>) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let truecolor_flag = supports_truecolor();
    for token in tokens {
        if truecolor_flag {
            stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Rgb(
                    token.rbg.0,
                    token.rbg.1,
                    token.rbg.2,
                ))))
                .expect("Failed to set color");
        } else {
            let ansci_val: u8 = rgb_to_ansi256(token.rbg.0, token.rbg.1, token.rbg.2);
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Ansi256(ansci_val))))
                .expect("Failed to set color");
        }
        write!(&mut stdout, "{}", token.token).expect("failed to write");
    }
}

/// Prints asciified image to the console
///
/// # Arguments
///
/// *   'img_tokens'    - Vector of Ascii tokens representing each pixel from the original image
/// *   'color_flag'    - Defines color output for the terminal
pub fn print_img_to_console(img_tokens: Vec<AsciiToken>, color_flag: bool) {
    if color_flag {
        write_color_output(img_tokens)
    } else {
        let img_str: String = img_tokens
            .iter()
            .map(|ascii_token| ascii_token.token)
            .collect();
        println!("{}", img_str);
    }
}

pub fn print_gif_to_console(img_frames: Vec<AsciiFrame>, color_flag: bool) {
    if color_flag {
        for frame in img_frames {
            process::Command::new("clear").status().unwrap();
            write_color_output(frame.frame_tokens);
            thread::sleep(Duration::from_millis(frame.delay));
        }
    } else {
        for frame in img_frames {
            let img_str: String = frame
                .frame_tokens
                .iter()
                .map(|ascii_token| ascii_token.token)
                .collect();
            process::Command::new("clear").status().unwrap();
            println!("{}", img_str);
            thread::sleep(Duration::from_millis(frame.delay));
        }
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
    fn normalize() {
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
    fn asciify_img() {
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
    fn generate_img() {
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
        let mut img: DynamicImage = image::open(PathBuf::from(path)).expect("File not Found...");
        img = normalize_img(img, 72);
        let res: Vec<AsciiToken> = convert_img_to_ascii_tokens(img, &ASCII_SIMPLE.to_vec());
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
        let mapping: Vec<char> = vec!['-', '}'];
        let mut img: DynamicImage = image::open(PathBuf::from(path)).expect("File not Found...");
        img = normalize_img(img, 72);
        let res: Vec<AsciiToken> = convert_img_to_ascii_tokens(img, &mapping);
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
        let mut img: DynamicImage = image::open(PathBuf::from(path)).expect("File not Found...");
        img = normalize_img(img, 20);
        let res: Vec<AsciiToken> = convert_img_to_ascii_tokens(img, &ASCII_SIMPLE.to_vec());
        let actual: String = res.iter().map(|ascii_token| ascii_token.token).collect();
        assert_eq!(expected, actual);
    }
}
