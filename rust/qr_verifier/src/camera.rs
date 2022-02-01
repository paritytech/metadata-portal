use std::path::{PathBuf};
use anyhow::anyhow;
use qr_reader_phone::process_payload::{process_decoded_payload, InProgress, Ready};
use image::{Luma, GrayImage, ImageBuffer};

use opencv::{
    prelude::*,
    Result,
    videoio,
    imgproc::{COLOR_BGR2GRAY, cvt_color,},
};

// Default camera settings
const DEFAULT_WIDTH: u32 = 640;
const DEFAULT_HEIGHT: u32 = 480;


pub fn read_qr_movie(source_file: &PathBuf) -> anyhow::Result<String> {
    let mut camera = create_camera(source_file)?;

    let mut out = Ready::NotYet(InProgress::None);
    let mut line = String::new();

    loop {
        match out {
            Ready::NotYet(decoding) => {
                out = match camera_capture(&mut camera) {
                    Ok(img) => process_qr_image(&img, decoding)?,
                    Err(_) => Ready::NotYet(decoding),
                };
            },
            Ready::Yes(a) => {
                line.push_str(&hex::encode(&a));
                break;
            },
        }
    }
    Ok(line)
}

fn create_camera(source_file: &PathBuf) -> anyhow::Result<videoio::VideoCapture>
{
    #[cfg(not(ocvrs_opencv_branch_32))]
    let mut camera = videoio::VideoCapture::from_file(source_file.to_str().unwrap(), videoio::CAP_ANY)?;

    let mut frame = Mat::default();

    match camera.read(&mut frame) {
        Ok(_) if frame.size()?.width > 0 => Ok(camera),
        Ok(_) => Err(anyhow!("Zero frame size.")),
        Err(e) => Err(anyhow!("Can`t read camera. {}", e)),
    }
}

fn camera_capture(camera: &mut videoio::VideoCapture) -> Result<GrayImage> {
    let mut frame = Mat::default();
    camera.read(&mut frame)?;

    let mut image: GrayImage = ImageBuffer::new(DEFAULT_WIDTH, DEFAULT_HEIGHT);
    let mut ocv_gray_image = Mat::default();

    cvt_color(&frame, &mut ocv_gray_image, COLOR_BGR2GRAY, 0)?;

    for y in 0..ocv_gray_image.rows() {
        for x in 0..ocv_gray_image.cols() {
            let pixel : Luma<u8> = Luma([*ocv_gray_image.at_2d(y,x)?]);
            image.put_pixel(x as u32, y as u32, pixel);
        };
    };

    Ok(image)
}


fn process_qr_image(image: &GrayImage, decoding: InProgress,) -> anyhow::Result<Ready> {
    let mut qr_decoder = quircs::Quirc::new();
    let codes = qr_decoder.identify(image.width() as usize, image.height() as usize, image);

    match codes.last() {
        Some(Ok(code)) => {
            match code.decode() {
                Ok(decoded) => {
                    process_decoded_payload(decoded.payload, decoding)
                },
                Err(_) => {
                    Ok(Ready::NotYet(decoding))
                }
            }
        },
        Some(_) => Ok(Ready::NotYet(decoding)),
        None => Ok(Ready::NotYet(decoding)),
    }
}
