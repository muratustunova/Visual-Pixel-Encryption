use image::GenericImage;
use image::{DynamicImage, GenericImageView, Rgba};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use sha2::{Digest, Sha256};
use std::env;
use std::path::Path;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!(
            "Usage: {} <image path> <encrypt or decrypt> <key>",
            args[0]
        );
        return;
    }

    let img_path = &args[1];
    let action = &args[2];
    let key = &args[3];

    let img = match image::open(&Path::new(img_path)) {
        Ok(img) => img,
        Err(e) => {
            println!("An error occurred while opening the image {}", e);
            return;
        }
    };

    // İşlemin başladığı zamanı al
    let start = Instant::now();

    let processed_img = match action.as_str() {
        "encrypt" => process_image(&img, key),
        "decrypt" => process_image(&img, key),
        _ => {
            println!(
                "Invalid operation: {}. Use 'encrypt' or 'decrypt'.",
                action
            );
            return;
        }
    };

    // İşlemin bittiği zamanı hesapla
    let duration = start.elapsed();
    let seconds = duration.as_secs();
    let milliseconds = duration.subsec_millis();

    match processed_img.save(img_path) {
        Ok(_) => println!("Image saved as '{}'.", img_path),
        Err(e) => println!("An error occurred while saving the image{}", e),
    }

    // Süreyi ekrana yazdır
    println!("\noperation: {} seconds took {} milliseconds.\n", seconds, milliseconds);
}

fn generate_mask(width: u32, height: u32, key: &str) -> Vec<Rgba<u8>> {
    let mut hasher = Sha256::new();
    hasher.update(key);
    let hash = hasher.finalize();
    let seed: [u8; 32] = hash.into();
    let mut rng: StdRng = SeedableRng::from_seed(seed);

    let mut mask = Vec::new();
    for _ in 0..(width * height) {
        let pixel = Rgba([
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
            255,
        ]);
        mask.push(pixel);
    }
    mask
}

fn apply_mask(img: &DynamicImage, mask: &Vec<Rgba<u8>>) -> DynamicImage {
    let (width, height) = img.dimensions();
    let mut masked_img = img.clone();

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let mask_pixel = &mask[(y * width + x) as usize];
            let new_pixel = Rgba([
                pixel[0] ^ mask_pixel[0],
                pixel[1] ^ mask_pixel[1],
                pixel[2] ^ mask_pixel[2],
                pixel[3],
            ]);
            masked_img.put_pixel(x, y, new_pixel);
        }
    }
    masked_img
}

fn process_image(img: &DynamicImage, key: &str) -> DynamicImage {
    let (width, height) = img.dimensions();
    let mask = generate_mask(width, height, key);
    apply_mask(img, &mask)
}
