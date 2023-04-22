use image::{DynamicImage, GenericImageView, Pixel};
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

struct Stats {
    power: u32,
    // Add other stats as needed
}

enum Action {
    Scan,
    SendSignal,
    // Add other actions as needed
}

struct Satellite {
    // Add fields for satellite components as needed
}


fn load_image(path: &str) -> DynamicImage {
    match image::open(path) {
        Ok(img) => img,
        Err(e) => panic!("Failed to open image file {}: {}", path, e),
    }
}
fn main() {
    let satellite_image = load_image("C:/Users/Mats/Rust Projects/satellite_game/assets/images/satellite_pixel_art.jpg");
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new("Satellite Game", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Clear the buffer to black
        for pixel in buffer.iter_mut() {
            *pixel = 0;
        }

        // Draw the satellite image in the center 
        let image_width = satellite_image.width();
        let image_height = satellite_image.height();
        //println!("WIDTH: {}, HEIGHT: {}", WIDTH, HEIGHT);
        //println!("image_width: {}, image_height:{}", image_width, image_height);
        let x_offset = 0usize;
        let y_offset = 0usize;
        for y in 0..image_height-1 {
            for x in 0..image_width-1 {
                let pixel = satellite_image.get_pixel(x, y);
                let rgba = pixel.to_rgba();
                let color = (rgba[0] as u32) << 16 | (rgba[1] as u32) << 8 | (rgba[2] as u32);
                let value = (y + y_offset as u32) as usize * WIDTH + (x + x_offset as u32) as usize;
                if value >= 307200{
                    break;
                }
                buffer[value] = color;
            }
        }

        // Update the window
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}