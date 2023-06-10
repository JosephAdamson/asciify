use crate::convert_img::{process_file, ConvertedFile};
use crate::utils::{build_output_file_name, supports_truecolor, AsciiFrame, AsciiToken};
use image::{ImageBuffer, Rgba, RgbaImage, Frame, Delay, codecs::gif::GifEncoder};
use imageproc::drawing::draw_text_mut;
use rgb2ansi256::rgb_to_ansi256;
use rusttype::{Font, Scale};
use std::{fs::{OpenOptions, File}, io::Write, process, thread, time::Duration};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

const SEGMENT_CONSTANT: u32 = 12;

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
                    token.rgb.0,
                    token.rgb.1,
                    token.rgb.2,
                ))))
                .expect("Failed to set color");
        } else {
            let ansci_val: u8 = rgb_to_ansi256(token.rgb.0, token.rgb.1, token.rgb.2);
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
            let delay: u64 = frame.delay.0 / frame.delay.1;
            thread::sleep(Duration::from_millis(delay));
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
            let delay: u64 = frame.delay.0 / frame.delay.1;
            thread::sleep(Duration::from_millis(delay));
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

/// Write ascii tokens to an image buffer
/// 
/// # Arguments
/// 
/// * 'img_canvas'      - Image buffer we write our data to
/// * 'img_frames'      - Vector of asciified gif frames
/// * 'color_flag'      - Defines color output for the terminal
/// * 'scale'           - scaling variable for a single character
/// * 'font'            - font for the output character
pub fn write_img(
    img_canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    img_tokens: Vec<AsciiToken>,
    color_flag: bool,
    scale: Scale,
    font: &Font
) {
    let mut y_pointer: i32 = 0;
    let mut x_pointer: i32 = 0 - SEGMENT_CONSTANT as i32;
    for token in img_tokens {
        // skip rendundant token for rendering and move on to the next line
        if token.token == '\n' {
            y_pointer = y_pointer + (SEGMENT_CONSTANT as i32 * 2);
            x_pointer = 0 - SEGMENT_CONSTANT as i32;
            continue;
        }

        x_pointer = x_pointer + SEGMENT_CONSTANT as i32;

        let rgb_val: Rgba<u8>;
        if color_flag {
            rgb_val = Rgba([token.rgb.0, token.rgb.1, token.rgb.2, 255]);
        } else {
            rgb_val = Rgba([255, 255, 255, 255]);
        }

        draw_text_mut(
            img_canvas,
            rgb_val,
            x_pointer,
            y_pointer,
            scale,
            &font,
            &token.token.to_string(),
        );
    }
}

/// Write asciified image to png or jpg file for output
///
/// # Aurguments
///
/// * 'img'                 - Vector of Ascii tokens representing each pixel from the original image
/// * 'color_flag'          - Defines color output for the terminal
/// * 'output_file_name     - File name of the output file
pub fn save_img(tokens: Vec<AsciiToken>, color_flag: bool, output_file_name: String) {
    let (w, h) = (tokens[0].parent_img_width, tokens[0].parent_img_height);
    let y_axis: u32 = h * SEGMENT_CONSTANT;
    let x_axis: u32 = w * SEGMENT_CONSTANT;
    let mut img_canvas: ImageBuffer<image::Rgba<u8>, Vec<u8>> = RgbaImage::new(x_axis, y_axis);

    let font: Vec<u8> = Vec::from(include_bytes!("../assets/Roboto-Regular.ttf") as &[u8]);
    let font: Font = Font::try_from_vec(font).unwrap();

    let font_size: f32 = SEGMENT_CONSTANT as f32 * 1.5;
    let scale = Scale {
        x: font_size,
        y: font_size,
    };
    write_img(&mut img_canvas, tokens, color_flag, scale, &font);
    img_canvas.save(output_file_name).unwrap();
}

/// Convert asciified images frames into a gif
/// 
/// # Arguments
/// 
/// * 'frames'              - A collection of encoded ascii frames.
/// * 'color_flag'          - Defines color output for the terminal
/// * 'output_file_name     - File name of the output file
pub fn save_gif(frames: Vec<AsciiFrame>, color_flag: bool, output_file_name: &String) {
    let (w, h) = (
        frames[0].frame_tokens[0].parent_img_width,
        frames[0].frame_tokens[0].parent_img_height,
    );
    let y_axis: u32 = h * SEGMENT_CONSTANT;
    let x_axis: u32 = w * SEGMENT_CONSTANT;

    let font: Vec<u8> = Vec::from(include_bytes!("../assets/Roboto-Regular.ttf") as &[u8]);
    let font: Font = Font::try_from_vec(font).unwrap();

    let font_size: f32 = SEGMENT_CONSTANT as f32 * 1.5;
    let scale = Scale {
        x: font_size,
        y: font_size,
    };

    let mut result: Vec<Frame> = Vec::new();
    // TODO: optimize this?
    for frame in frames {
        // give each frame a black background
        let mut img_canvas = RgbaImage::from_pixel(
            x_axis, 
            y_axis, 
            Rgba([0, 0, 0, 255]));
        // color background experiment
        
        write_img(&mut img_canvas, frame.frame_tokens, color_flag, scale, &font);
        let f: Frame = Frame::from_parts(img_canvas, 0, 0, Delay::from_numer_denom_ms(
            frame.delay.0 as u32, 
            frame.delay.1 as u32
        ));
        result.push(f);
    }

    let file: File = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(output_file_name)
        .expect("File could not be read");

    let mut encoder: GifEncoder<File> = GifEncoder::new(file);
    encoder.encode_frames(result).unwrap();
}

/// Save asciified img to file
///
/// # Arguments
///
/// * 'path_arg'        - File path to the text file
/// * 'detail_flag'     - Dictate the amount of ascii characters use
/// * 'color_flag'      - Defines color output for the terminal
/// * 'pixel_scale'     - Maximum bound used for width
pub fn save(
    path_arg: String,
    detail_flag: bool,
    color_flag: bool,
    mapping: Option<String>,
    pixel_scale: Option<u32>,
) {
    let file_name: String = build_output_file_name(&path_arg).unwrap();

    let ascii_data: ConvertedFile = process_file(path_arg, pixel_scale, detail_flag, mapping);
    match ascii_data {
        ConvertedFile::IMAGE(img) => save_img(img, color_flag, file_name),
        ConvertedFile::GIF(gif) => save_gif(gif, color_flag, &file_name),
        ConvertedFile::ERROR(e) => {
            panic!("{}", e);
        }
    }
}
