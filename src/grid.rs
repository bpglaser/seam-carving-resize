use image::{DynamicImage, GenericImage, GrayImage, Luma, Pixel};

use num_traits::ToPrimitive;

use Point;
use PointPath;

pub struct EnergyGrid {
    rows: Vec<Vec<usize>>,
    darkest_value: usize,
}

impl EnergyGrid {
    pub fn from_image(image: &DynamicImage) -> Self {
        let mut darkest_value = 0;
        let mut rows = vec![]; // todo linked list? avoid shifts when updating?
        for y in 0..image.height() {
            let mut row = vec![];
            for x in 0..image.width() {
                let energy = calculate_pixel_energy(image, x, y);
                if energy > darkest_value {
                    darkest_value = energy;
                }
                row.push(energy);
            }
            rows.push(row);
        }
        EnergyGrid {
            rows,
            darkest_value,
        }
    }

    fn dimensions(&self) -> (usize, usize) {
        let width = self.rows[0].len();
        let height = self.rows.len();
        (width, height)
    }

    fn get(&self, x: usize, y: usize) -> usize {
        self.rows[y][x]
    }

    pub fn find_path(&self) -> PointPath {
        unimplemented!()
    }

    pub fn add_path(&mut self, path: &PointPath) {
        unimplemented!()
    }

    pub fn remove_path(&mut self, path: &PointPath) {
        unimplemented!()
    }

    pub fn rotate_clockwise(&mut self) {
        unimplemented!()
    }

    pub fn rotate_counterclockwise(&mut self) {
        unimplemented!()
    }

    pub fn as_image(&self) -> GrayImage {
        let (width, height) = self.dimensions();
        let darkest_value = self.darkest_value;
        let mut image = DynamicImage::new_luma8(width as u32, height as u32).to_luma();

        for y in 0..height {
            for x in 0..width {
                let ratio: f64 = u8::max_value() as f64 / darkest_value as f64;
                let grid_value: usize = self.get(x, y);
                let value = grid_value as f64 * ratio;
                let pixel = Luma { data: [value as u8] };
                image.put_pixel(x as u32, y as u32, pixel);
            }
        }

        image
    }
}

fn calculate_pixel_energy(image: &DynamicImage, x: u32, y: u32) -> usize {
    let (width, height) = image.dimensions();

    // Wrap around if neighbors are out of bounds
    let left_x = x.checked_sub(1).unwrap_or(width - 1);
    let right_x = (x + 1) % width;
    let up_y = y.checked_sub(1).unwrap_or(height - 1);
    let down_y = (y + 1) % height;

    let horizontal_square_gradient = square_gradient(image, left_x, y, right_x, y);
    let vertical_square_gradient = square_gradient(image, x, up_y, x, down_y);
    horizontal_square_gradient + vertical_square_gradient
}

fn square_gradient(image: &DynamicImage, x1: u32, y1: u32, x2: u32, y2: u32) -> usize {
    let pixel1 = image.get_pixel(x1, y1);
    let pixel1_channels = pixel1.channels();

    let pixel2 = image.get_pixel(x2, y2);
    let pixel2_channels = pixel2.channels();

    let mut sum = 0;
    for i in 0..pixel1_channels.len() {
        let a = pixel1_channels[i]
            .to_isize()
            .expect("Unable to convert value");
        let b = pixel2_channels[i]
            .to_isize()
            .expect("Unable to convert value");
        sum += (a - b).abs().pow(2); // Squared abs difference
    }
    sum as usize
}
