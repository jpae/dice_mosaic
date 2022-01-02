use std::collections::HashMap;
use uuid::Uuid;
use image::{DynamicImage, GenericImage, GrayImage};
use itertools::Itertools;

const DICE_NUM: u8 = 6;

#[derive(Copy, Clone)]
pub enum Resolution {
    Low = 3,
    Medium = 4,
    High = 5,
}

pub struct DiceMosaic {
    // original image in grayscale
    img: GrayImage,
    // dimensions of the new image to create
    dimensions: (u32, u32),
    // cell is in reference to the original image and how many pixels each dice image will cover
    cell_width: u32,
    cell_height: u32,
    // The pixel dimensions of the diceX.png. Assumes a square image
    dice_pixels: u32,
    // Hash of how many dice of a specific dice face is used
    dice_counter: HashMap<u32, u32>,
}

impl DiceMosaic {
    pub fn new(file: &String, dice_pixels: u32, resolution: Resolution) -> DiceMosaic {
        // Create DynamicImage of target image, panic if failure to process file
        let file = image::open(file).unwrap();
        // Create Grayscale image
        let img: GrayImage = file.to_luma8();

        // Calculate mosaic dimensions
        let (x, y) = img.dimensions();
        // Subtract any remainder pixels so the final image has no black edges
        let res = resolution as u32;
        let dimensions = (x * res - ((x * res) % dice_pixels), y * res - ((y * res) % dice_pixels));
        let (img_cnt_x, img_cnt_y) = (dimensions.0 / dice_pixels, dimensions.1 / dice_pixels);
        let (cell_width, cell_height) = (x / img_cnt_x, y / img_cnt_y);
        // Initialize empty dice face counter
        let dice_counter = HashMap::new();

        DiceMosaic {
            img, dimensions, cell_width, cell_height, dice_pixels, dice_counter
        }
    }

    pub fn process(&mut self) -> () {
        // Create DynamicImage objects of dice faces, panic if failure to process file
        let dice: Vec<image::DynamicImage> = DiceMosaic::initialize_dice().unwrap();

        // Image to save at the end of process
        let mut output_image = DynamicImage::new_rgba8(self.dimensions.0, self.dimensions.1);
        
        for w in 0..self.dimensions.0 / self.dice_pixels {
            for h in 0..self.dimensions.1 / self.dice_pixels {
                let value = DiceMosaic::avg_value(&self.img, (w * self.cell_width,
                                                                        h * self.cell_height), 
                                                             (self.cell_width,
                                                                        self.cell_height));
                let num = DiceMosaic::dice_face(value);
                let dice_img = &dice[num as usize];
                let count = self.dice_counter.entry(num as u32).or_insert(0);
                *count += 1;

                output_image.copy_from(dice_img, w * self.dice_pixels, h * self.dice_pixels)
                            .unwrap_or_else(|err| eprintln!("{:?}", err));
            }
        }

        // Panic if failure to save image
        let img_name = format!("asset/output/{}.jpeg", Uuid::new_v4());
        println!("Creating {}", img_name);
        output_image.save(img_name).unwrap();
    }

    pub fn stats(&self) -> () {
        let mut total_dice = 0;
        for i in self.dice_counter.values() {
            total_dice += i;
        }

        println!("Dice Counters:");
        for (key, value) in self.dice_counter.iter().sorted_by_key(|x| x.0) {
            println!("  dice face {}: {}", key + 1, value);
        }
        println!("Total dice needed: {}", total_dice);
    }

    fn initialize_dice() -> Result<Vec<DynamicImage>, String> {
        let mut images = Vec::new();
        for num in 1..=DICE_NUM {
            let file = format!("asset/dice/dice{}.png", num);
            let img: DynamicImage = match image::open(file.clone()).ok() {
                Some(image) => image,
                None => return Err(format!("Failed to open: {:?}", file))
            };
            images.push(img);
        }
        Ok(images)
    }

    fn avg_value(img: &GrayImage, corner: (u32, u32), dimensions: (u32, u32)) -> u8 {
        let mut count: u32 = 0;
        let mut sum: u32 = 0;

        for x in corner.0..(corner.0 + dimensions.0) {
            for y in corner.1..(corner.1 + dimensions.1) {
                count += 1;
                sum += img.get_pixel(x, y).0[0] as u32;
            }
        }

        (sum / count) as u8
    }

    fn dice_face(value: u8) -> u8 {
        let band_length: f64 = u8::MAX as f64 / DICE_NUM as f64;
        DICE_NUM - (value as f64 / band_length).ceil() as u8
    }
}