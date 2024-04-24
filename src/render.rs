use std::path::Path;

use image2::{transform, Image, ImageBuf, Rgb};
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;

use crate::exit;

pub fn render(mut image: ImageBuf<u8, Rgb>, file: &Path) -> Result<(), Error> {
    let event_loop = EventLoop::new();

    let (width, height) = calculate_dimensions(&image, &event_loop);

    if width == 0 || height == 0 {
        exit!("Failed to get image dimensions");
    }

    if width != image.width() as u32 || height != image.height() as u32 {
        image = resize_image(&image, width, height)
    };

    let window = create_window(width, height, &event_loop, file);
    let mut pixels = create_pixel_buffer(&window, width, height);
    let mut input = WinitInputHelper::new();

    event_loop.run(move |event, _, control_flow| match event {
                  Event::RedrawRequested(_) => {
                      draw_pixels(pixels.frame_mut(), &image);
                      pixels.render();
                  }
                  _ => {
                      if input.update(&event) {
                          if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                              *control_flow = ControlFlow::Exit;
                          }

                          if let Some(size) = input.window_resized() {
                              resize_pixels(&mut pixels, size.to_logical(1.0));
                          }

                          window.request_redraw();
                      }
                  }
              });
}

fn calculate_dimensions(image: &ImageBuf<u8, Rgb>, event_loop: &EventLoop<()>) -> (u32, u32) {
    if image.width() < event_loop.primary_monitor().unwrap().size().width as usize
       && image.height() < event_loop.primary_monitor().unwrap().size().height as usize
    {
        (image.width() as u32, image.height() as u32)
    } else {
        let aspect_ratio = image.width() as f64 / image.height() as f64;

        // subtract 100 pixels from the minimum dimension to account for window border
        let minimum_dimension = event_loop.primary_monitor()
                                          .unwrap()
                                          .size()
                                          .width
                                          .min(event_loop.primary_monitor().unwrap().size().height)
                                as f64
                                - 125.0;
        ((minimum_dimension * aspect_ratio) as u32, minimum_dimension as u32)
    }
}

fn resize_image(image: &ImageBuf<u8, Rgb>, width: u32, height: u32) -> ImageBuf<u8, Rgb> {
    let mut resized_image = ImageBuf::new(width as usize, height as usize);
    transform::resize(&mut resized_image, image, width as usize, height as usize);

    resized_image
}

fn create_window(width: u32, height: u32, event_loop: &EventLoop<()>, file: &Path) -> Window {
    let size = LogicalSize::new(width as f64, height as f64);

    let filename =
        file.file_name().unwrap_or(std::ffi::OsStr::new("viewimg")).to_str().unwrap_or("viewimg");

    let window = WindowBuilder::new().with_title(filename)
                                     .with_inner_size(size)
                                     .with_min_inner_size(size)
                                     .with_max_inner_size(event_loop.primary_monitor()
                                                                    .unwrap()
                                                                    .size()
                                                                    .to_logical::<f64>(1.0))
                                     .with_resizable(true)
                                     .build(&event_loop)
                                     .unwrap();

    window
}

fn create_pixel_buffer(window: &Window, width: u32, height: u32) -> Pixels {
    let surface_texture = SurfaceTexture::new(width, height, window);
    Pixels::new(width, height, surface_texture).unwrap()
}

fn draw_pixels(frame: &mut [u8], image: &ImageBuf<u8, Rgb>) {
    let width = image.width();

    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let x = i % width as usize;
        let y = i / width as usize;
        let rgba = [image.get(x, y, 0), image.get(x, y, 1), image.get(x, y, 2), 255];
        pixel.copy_from_slice(&rgba);
    }
    
}

fn resize_pixels(pixels: &mut Pixels, size: LogicalSize<f64>) {
    let new_width = size.width.round() as u32;
    let new_height = size.height.round() as u32;

    pixels.resize_surface(new_width, new_height);
}
