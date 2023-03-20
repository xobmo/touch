use anyhow::Result;
use pixels::{Pixels, SurfaceTexture};
use std::path::Path;
use std::fs;
use std::time::Duration;
use std::io::{BufRead, BufReader, Write};
use serialport::*;
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn write_zoom_to_serial(zoom_array: &[[u8; 4]; 4]) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let settings_file = fs::File::open("settings.txt")?;
    let reader = BufReader::new(settings_file);
    let mut lines = reader.lines();

    let port_name = lines.next().ok_or("No serial port specified")??;
    let baud_rate: u32 = lines
    .next()
    .ok_or("No baud rate specified")??
    .parse()
    .map_err(|_| "Invalid baud rate")?;

    let mut serial_port = serialport::new(port_name, baud_rate)
    .timeout(Duration::from_millis(1000))
    .open().expect("Failed to open port");

    for row in zoom_array {
        let row_string = row
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        write!(serial_port, "{}", row_string)?;
    }

    write!(serial_port, "\r\n")?; // Newline after the 2D array

    Ok(())
}

fn main() -> Result<()> {
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Image Display Demo")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut zoom_area: [[u8; 4]; 4] = [[0; 4]; 4];
 
    let surface_texture = SurfaceTexture::new(WIDTH, HEIGHT, &window);
    let mut pixels = Pixels::new(WIDTH, HEIGHT, surface_texture)?;
    let image_path = Path::new("image.png");
    let img = image::open(&image_path)
        .expect("Failed to load image")
        .into_rgba8();
    let img_dimensions = img.dimensions();

   event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input:
                        winit::event::KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(size) => {
                    pixels.resize_surface(size.width, size.height);
                }
                WindowEvent::CursorMoved { position, .. } => {
                    let x = position.x as u32;
                    let y = position.y as u32;
                    for i in 0..4 {
                        for j in 0..4 {
                            let i_ = i as usize;
                            let j_ = j as usize;
                            if x+i >= img_dimensions.0 ||
                               y+j >= img_dimensions.1 {
                                zoom_area[i_][j_] = 0;
                                continue;
                            }
                            let img_pixel = img.get_pixel(x+i, y+j);
                            let image::Rgba([r, g, b, a]) = img_pixel;
                            let gray_value = (0.1 * *r as f32 + 0.6 * *g as f32 + 0.3 * *b as f32) as u8;
                            zoom_area[i_][j_] = gray_value;
                       }
                    }
                    write_zoom_to_serial(&zoom_area).unwrap();
                }
                _ => (),
            }
            Event::RedrawRequested(_) => {
                let frame = pixels.get_frame_mut();
                let zoom_area_left = (WIDTH as f32 *0.7).round() as u32;
                let zoom_area_bottom = (HEIGHT as f32 *0.3).round() as u32;
                let zoom_area_width = WIDTH - zoom_area_left;
                let zoom_area_height = zoom_area_bottom;
                let zoom_pixel_width = zoom_area_width / 4;
                let zoom_pixel_height = zoom_area_height / 4;

                for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                    let x = (i % WIDTH as usize) as u32;
                    let y = (i / WIDTH as usize) as u32;
                    if x < zoom_area_left && y < img_dimensions.1 {
                        let img_pixel = img.get_pixel(x, y);
                        pixel.copy_from_slice(&img_pixel.0);
                    } else if x > zoom_area_left && y < zoom_area_bottom {
                        let zoom_x = ((x - zoom_area_left ) / zoom_pixel_width) as usize;
                        let zoom_y = ((zoom_area_bottom - y ) / zoom_pixel_height) as usize;
                        if zoom_x < 4 && zoom_y < 4 { 
                            pixel.copy_from_slice(&[255,255,255, zoom_area[zoom_x][zoom_y]]);
                        } else {
                            pixel.copy_from_slice(&[0, 0, 0, 255]);
                        }
                    } else {
                        pixel.copy_from_slice(&[0, 0, 0, 255]);
                    }
                }

                if pixels.render().is_err() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => (),
        }
    })
}

