use exr::image::rgba::{Image, Pixels};
use exr::prelude::f16;

pub fn extract_exr_data(image: &Image) -> Vec<u8> {
    let (width, height) = (image.resolution.0, image.resolution.1);

    let mut exr_data = vec![0u8; width as usize * height as usize * 3];

    let (minimum, maximum) = find_min_max(image);

    for i in 0..(width * height) {
        let x = i % width as usize;
        let y = i / width as usize;
        let index = image.vector_index_of_first_pixel_component(exr::math::Vec2(x, y));

        let data: (f32, f32, f32) = match &image.data {
            Pixels::F32(data) => (data[index + 0], data[index + 1], data[index + 2]),
            Pixels::F16(data) => {
                (data[index + 0].to_f32(), data[index + 1].to_f32(), data[index + 2].to_f32())
            }
            Pixels::U32(data) => {
                (data[index + 0] as f32, data[index + 1] as f32, data[index + 2] as f32)
            }
        };

        exr_data[3 * i + 0] =
            (gamma_correct(normalize_f32(data.0, minimum, maximum), 2.0) * 255.0) as u8;
        exr_data[3 * i + 1] =
            (gamma_correct(normalize_f32(data.1, minimum, maximum), 2.0) * 255.0) as u8;
        exr_data[3 * i + 2] =
            (gamma_correct(normalize_f32(data.2, minimum, maximum), 2.0) * 255.0) as u8;
    }
    exr_data
}

pub fn find_min_max(image: &Image) -> (f32, f32) {
    let min_max = match &image.data {
        Pixels::F32(data) => (data.iter().cloned().fold(0.0 / 0.0, f32::min),
                              data.iter().cloned().fold(0.0 / 0.0, f32::max)),
        Pixels::F16(data) => {
            let mut minimum = f16::MAX;
            let mut maximum = f16::MIN;

            for value in data {
                if *value < minimum {
                    minimum = *value;
                } else if *value > maximum {
                    maximum = *value;
                }
            }

            (minimum.to_f32(), maximum.to_f32())
        }
        Pixels::U32(data) => {
            ((*data.iter().min().unwrap()) as f32, (*data.iter().max().unwrap()) as f32)
        }
    };

    min_max
}

pub fn normalize_f32(value: f32, minimum: f32, maximum: f32) -> f32 {
    (value - minimum) / (maximum - minimum)
}

pub fn clamp_f32(value: f32, lower_bound: f32, upper_bound: f32) -> f32 {
    let minimum = value.max(lower_bound);
    let maximum = value.min(upper_bound);

    minimum.min(maximum)
}

pub fn clamp_rgb(value: f32) -> f32 { value.min(255.0).max(0.0) }

pub fn gamma_correct(luminance: f32, gamma: f32) -> f32 { luminance.powf(1.0 / gamma) }
