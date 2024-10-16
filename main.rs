use image::{DynamicImage, GenericImageView, Rgba, ImageBuffer};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use sha2::{Sha256, Digest};
use std::env;
use std::path::{Path};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!("Kullanım: {} <görüntü yolu> <encrypt veya decrypt> <anahtar>", args[0]);
        return;
    }

    let img_path = &args[1];
    let action = &args[2];
    let key = &args[3];

    let img = match image::open(&Path::new(img_path)) {
        Ok(img) => img,
        Err(e) => {
            println!("Görüntü açılırken hata oluştu: {}", e);
            return;
        }
    };

    match action.as_str() {
        "encrypt" => {
            let corrupted_img = process_image(&img, key);
            match save_image(&corrupted_img, img_path, "encrypted") {
                Ok(_) => println!("Piksel encrypt edilmiş görüntü 'output/encrypted_{}' olarak kaydedildi.", img_path),
                Err(e) => println!("encrypt görüntü kaydedilirken hata oluştu: {}", e),
            }
        }
        "decrypt" => {
            let fixed_img = process_image(&img, key);
            match save_image(&fixed_img, img_path, "decrypted") {
                Ok(_) => println!("Piksel decrypt edilmiş görüntü 'output/decrypted_{}' olarak kaydedildi.", img_path),
                Err(e) => println!("decrypt görüntü kaydedilirken hata oluştu: {}", e),
            }
        }
        _ => println!("Geçersiz seçenek! 'encrypt' veya 'decrypt' seçeneğini kullanın."),
    }
}

fn generate_mask(width: u32, height: u32, key: &str) -> Vec<Rgba<u8>> {
    let mut hasher = Sha256::new();
    hasher.update(key);
    let hash = hasher.finalize();
    let seed: [u8; 32] = hash.into();
    let mut rng: StdRng = SeedableRng::from_seed(seed);

    (0..(width * height)).map(|_| {
        Rgba([ 
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
        ])
    }).collect()
}

fn apply_mask(img: &DynamicImage, mask: &[Rgba<u8>]) -> DynamicImage {
    let (width, height) = img.dimensions();
    let mut masked_img = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let mask_pixel = &mask[(y * width + x) as usize];
            let new_pixel = Rgba([ 
                pixel[0] ^ mask_pixel[0],
                pixel[1] ^ mask_pixel[1],
                pixel[2] ^ mask_pixel[2],
                pixel[3] ^ mask_pixel[3],
            ]);
            masked_img.put_pixel(x, y, new_pixel);
        }
    }

    DynamicImage::ImageRgba8(masked_img)
}

fn process_image(img: &DynamicImage, key: &str) -> DynamicImage {
    let (width, height) = img.dimensions();
    let mask = generate_mask(width, height, key);
    apply_mask(img, &mask)
}

fn save_image(img: &DynamicImage, img_path: &str, action: &str) -> Result<(), image::ImageError> {
    let path = Path::new(img_path);
    let file_name = path.file_name().unwrap().to_str().unwrap();
    let output_dir = Path::new("output");
    std::fs::create_dir_all(output_dir).unwrap();

    let new_file_name = if action == "decrypted" {
        file_name.replace("encrypted_", "decrypted_")
    } else {
        format!("{}_{}", action, file_name)
    };
    
    let mut output_path = output_dir.join(new_file_name);
    output_path.set_extension("png");

    img.save_with_format(output_path, image::ImageFormat::Png)
}
