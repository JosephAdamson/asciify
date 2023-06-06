use std::{
    fs::OpenOptions,
    io::Write,
    process, thread,
    time::Duration,
};
use crate::utils::{AsciiToken, AsciiFrame, build_output_file_name, supports_truecolor};
use crate::convert_img::{ConvertedFile, process_file};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use rgb2ansi256::rgb_to_ansi256;


/// Parses ascii pixel vector and prints colored output to the terminal
///
/// # Arguments
///
/// * 'tokens'    - Vector of Ascii tokens representing each pixel from the original image
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
            stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Ansi256(ansci_val))))
                .expect("Failed to set color");
        }
        write!(&mut stdout, "{}", token.token).expect("failed to write");
    }
}

/// Prints asciified image to the console
///
/// # Arguments
///
/// * 'img_tokens'    - Vector of Ascii tokens representing each pixel from the original image
/// * 'color_flag'    - Defines color output for the terminal
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

/// Prints gif frames to the console
///
/// # Arguments
///
/// * 'img_frames'    - Vector of asciified gif frames
/// * 'color_flag'    - Determines color output
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

/// Wrapper function to that processes img files and outputs them to the terminal
///
/// # Arguments
///
/// * 'path_arg'    - File path to the text file
/// * 'pixel_scale'       - Maximum bound used for width
/// * 'detail_flag' - Dictate the amount of ascii characters use
pub fn output_to_console(
    path_arg: String,
    pixel_scale: Option<u32>,
    detail_flag: bool,
    color_flag: bool,
    mapping: Option<String>,
) -> Result<(), &'static str> {
    match process_file(path_arg, pixel_scale, detail_flag, mapping) {
        ConvertedFile::IMAGE(img_tokens) => {
            print_img_to_console(img_tokens, color_flag);
            return Ok(());
        }
        ConvertedFile::GIF(img_frames) => {
            print_gif_to_console(img_frames, color_flag);
            return Ok(());
        }
        ConvertedFile::ERROR(e) => panic!("{}", e),
    };
}


// Save asciified img to file
pub fn save_img(path_arg: String, detail_flag: bool, mapping: Option<String>, pixel_scale: Option<u32>) {
    let file_name: String = build_output_file_name(&path_arg).unwrap();

    let ascii_data: ConvertedFile = process_file(path_arg, pixel_scale, detail_flag, mapping);
    match ascii_data {
        ConvertedFile::IMAGE(img) => {
            let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_name)
            .expect("Could not write to file");

            let img_str: String = img.iter()
                .map(|ascii_token| {ascii_token.token})
                .collect();
            file.write_all(img_str.as_bytes()).expect("Could not write to file");
        },
        ConvertedFile::GIF(gif) => {

        },
        ConvertedFile::ERROR(e) => {

        }
    }
}