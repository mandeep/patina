use std::path::Path;

use clap::{App, Arg};

mod macros;
mod reader;
mod render;
mod utils;

use crate::reader::{read_exr_image, read_hdr_image};
use crate::render::render;

fn main() {
    let matches =
        App::new("viewimg").version("0.9.0")
                           .arg(Arg::with_name("image").help("The file path to the image to view")
                                                       .index(1)
                                                       .required(true))
                           .get_matches();

    let file = matches.value_of("image").unwrap();

    let filepath = Path::new(&file);

    if filepath.is_file() {
        if let Some(extension) = filepath.extension() {
            let image_buffer = match extension.to_str().unwrap() {
                "exr" => match read_exr_image(filepath) {
                    Ok(image) => image,
                    Err(error) => exit!("{:?}", error),
                },

                _ => match read_hdr_image(filepath) {
                    Ok(image) => image,
                    Err(error) => exit!("{:?}", error),
                },
            };

            if let Err(error) = render(image_buffer, filepath) {
                exit!("{}", error);
            }
        }
    } else {
        exit!("ERROR: Could not read path. Please provide a valid HDR image path.");
    }
}
