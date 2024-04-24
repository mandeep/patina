use std::path::Path;

use image2::Image;
use image2::{io, ImageBuf, Rgb};
use exr::image::RgbChannels;
use exr::prelude::{ReadLayers, ReadChannels};

// fn extract_exr_data(image: &Image<Layer<SpecificChannels<RgbaImage, RgbaChannels>>>) -> Vec<u8> {
//     let (width, height) = (image.layer_data.size.0, image.layer_data.size.1);

//     let mut exr_data = vec![0u8; width as usize * height as usize * 3];


//     exr_data.par_chunks_mut(3).enumerate().for_each(|(i, pixel)| {
//         let x = i % width as usize;
//         let y = i / width as usize;

//         let data: (f32, f32, f32, f32) = &image.layer_data;

//         pixel[0] = compensate(data.0);
//         pixel[1] = compensate(data.1);
//         pixel[2] = compensate(data.2);
//     });

//     exr_data
// }


pub fn read_exr_image(filepath: &Path) -> Result<ImageBuf<u8, Rgb>, exr::error::Error> {
    let reader = exr::image::read::read()
    .no_deep_data()
    .largest_resolution_level()
    .rgb_channels(
    |resolution, _channels: &RgbChannels| -> ImageBuf<u8, Rgb> {
            ImageBuf::new(
                resolution.width(),
                resolution.height()
            )
        },

        // set each pixel in the png buffer from the exr file
        |png_pixels, position, (r, g, b): (f32, f32, f32)| { 
            png_pixels.set_f(position.x(), position.y(), 0, r.into());
            png_pixels.set_f(position.x(), position.y(), 1, g.into());
            png_pixels.set_f(position.x(), position.y(), 2, b.into());
        }
    )
    .first_valid_layer()
    .all_attributes();

    match reader.from_file(filepath) {
        Ok(image) => {
            Ok(image.layer_data.channel_data.pixels)
        },
        Err(error) => Err(error),

    }
}

pub fn read_hdr_image(filepath: &Path) -> Result<ImageBuf<u8, Rgb>, image2::Error> {
    match io::read(filepath) {
        Ok(hdr_image_buffer) => Ok(hdr_image_buffer),

        Err(error) => Err(error),
    }
}

