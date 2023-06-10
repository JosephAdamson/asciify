use crate::utils::{
    get_file_extension, is_supported_format, AsciiFrame, AsciiToken,
};
use image::{
    codecs::gif::GifDecoder, imageops::FilterType, AnimationDecoder, DynamicImage, Frame,
    GenericImageView, Rgba,
};
use std::{
    fs::{File, OpenOptions},
    path::PathBuf,
};

const ASCII_DETAILED: [char; 70] = [
    ' ', '.', '\'', '`', '^', '"', ',', ':', ';', 'I', 'l', '!', 'i', '>', '<', '~', '+', '_', '-',
    '?', ']', '[', '}', '{', '1', ')', '(', '|', '\\', '/', 't', 'f', 'j', 'r', 'x', 'n', 'u', 'v',
    'c', 'z', 'X', 'Y', 'U', 'J', 'C', 'L', 'Q', '0', 'O', 'Z', 'm', 'w', 'q', 'p', 'd', 'b', 'k',
    'h', 'a', 'o', '*', '#', 'M', 'W', '&', '8', '%', 'B', '@', '$',
];

const ASCII_SIMPLE: [char; 10] = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];

const MAX_VALUE: f64 = 255.0;

pub enum ConvertedFile {
    IMAGE(Vec<AsciiToken>),
    GIF(Vec<AsciiFrame>),
    ERROR(String),
}

/// Returns an descaled Dynamic image
///
/// # Arguments
///
/// * 'img'           - A pixel matrix
/// * 'pixel_scale'   - pixel scale factor used to resize the image
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
/// * 'intensity'     - pixel intensity
/// * 'detail_flag'   - dictate the amount of ascii characters use
fn asciify_intensity(intensity: u32, ascii_table: &Vec<char>) -> char {
    let index: f64 = (intensity as f64 / MAX_VALUE) * ((ascii_table.len() - 1) as f64);
    return ascii_table[index as usize];
}

/// Convert a DynamicImage's pixel values into a AsciiTokens
///
/// # Arguments
///
/// * 'img'           - Rgba pixel matrix
/// * 'ascii_table'   - Char vector of mappable ascii characters
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
                    rgb: (pixel[0], pixel[1], pixel[2]),
                    parent_img_width: width,
                    parent_img_height: height
                });
            }
        }
        if y % 2 == 0 {
            img_tokens.push(AsciiToken {
                token: '\n',
                rgb: (0, 0, 0),
                parent_img_width: width,
                parent_img_height: height
            });
        }
    }
    return img_tokens;
}

/// Returns a collection of frames, each frame representing an asciified version of the
/// frame from the original input gif.
///
/// # Arguments
///
/// * 'gif'             - Gif file wrapped in a decoder
/// * 'ascii_table'     - Char vector of mappable ascii characters
/// * 'scale'           - Maximum bound used for width
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
        let int_delay: (u64, u64) = (frame_ratio.0 as u64, frame_ratio.1 as u64);
        let ascii_frame: AsciiFrame = AsciiFrame {
            frame_tokens: ascii_tokens,
            delay: int_delay,
        };
        tokenized_gif.push(ascii_frame);
    }
    return tokenized_gif;
}

/// Reads file and converts image data into a vector of AsciiToken data.
///
/// # Arguments
///
/// * 'path_arg'    - File path to the text file
/// * 'scale'       - Maximum bound used for width
/// * 'detail_flag' - Dictate the amount of ascii characters use
pub fn process_file(
    path_arg: String,
    pixel_scale: Option<u32>,
    detail_flag: bool,
    mapping: Option<String>,
) -> ConvertedFile {
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

            return ConvertedFile::GIF(img_frames);
        } else {
            let mut img: DynamicImage =
                image::open(PathBuf::from(path_arg)).expect("File not Found...");
            img = normalize_img(img, scale);
            let img_tokens: Vec<AsciiToken> = convert_img_to_ascii_tokens(img, &ascii_table);

            return ConvertedFile::IMAGE(img_tokens);
        }
    } else {
        return ConvertedFile::ERROR(String::from("File format not supported for file"));
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

    // #[test]
    // fn get_dims_test() {let path: String = String::from("assets/ferris.jpg");
    //     let expected: (u32, u32) = (730, 487);
    //     let actual: (u32, u32) = get_dimensions(&path, Option::None);
    //     assert_eq!(actual, expected);
    // }
}
