use aoc_lib::AocImplementation;
use itertools::Itertools;
use image;

fn main() {
    let day = Day8{};
    day.start(8);
}

struct Day8 {}

impl AocImplementation<u8> for Day8 {
    fn process_input(&self, input: &str) -> Vec<u8> {
        input.split("").filter(|s| s != &"").map(|s| s.parse().unwrap()).collect()
    }

    fn execute(&self, input: Vec<u8>) -> Option<i32> {
        let width = 25;
        let height = 6;
        let layers = input.chunks(width * height).rev();

        let mut img_buf = image::ImageBuffer::new(width as u32, height as u32);

        for layer in layers {
            for (index, b) in layer.iter().enumerate() {
                let (x, y) = get_coords_from_index(width, height, index);
                println!("index: {}, x: {}, y: {}", index, x, y);
                if *b == 2 { continue; }
                let pixel = img_buf.get_pixel_mut(x, y);
                *pixel = match b {
                    0 => image::Rgb([0,0,0]),
                    1 => image::Rgb([255,255,255]),
                    _ => panic!("Unknown color: {}", b)
                }
            }
        }

        img_buf.save("code.png").unwrap();

        Some(0)
    }
}

fn get_coords_from_index(width: usize, height: usize, index: usize) -> (u32, u32) {
    let y = index / width;
    let x = index % width;

    (x as u32, y as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_coordinates_0_0() {
        let (x, y) = get_coords_from_index(25, 6, 0);
        assert_eq!(x, 0);
        assert_eq!(y, 0);
    }

    #[test]
    fn correct_coordinates_25_6() {
        let (x, y) = get_coords_from_index(25, 6, 25 * 6 - 1);
        assert_eq!(x + 1, 25);
        assert_eq!(y + 1, 6);
    }

}