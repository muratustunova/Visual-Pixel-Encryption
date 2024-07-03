use image::{DynamicImage, GenericImageView, Rgba, ImageBuffer, ColorType}; // Görüntü işleme için gerekli kütüphaneler
use rand::{Rng, SeedableRng}; // Rastgele sayı üretimi için gerekli kütüphaneler
use rand::rngs::StdRng; // Standart rastgele sayı üretici
use sha2::{Sha256, Digest}; // Hashing için gerekli kütüphane
use std::env; // Ortam değişkenlerine erişim için gerekli kütüphane
use std::path::Path; // Dosya yolu işlemleri için gerekli kütüphane

fn main() {
    let args: Vec<String> = env::args().collect(); // Komut satırı argümanlarını topla

    if args.len() < 4 {
        println!("Kullanım: {} <görüntü yolu> <encrypt veya decrypt> <anahtar>", args[0]);
        return;
    }

    let img_path = &args[1]; // İlk argüman: görüntü dosyasının yolu
    let action = &args[2]; // İkinci argüman: yapılacak işlem (encrypt veya decrypt)
    let key = &args[3]; // Üçüncü argüman: anahtar

    let img = match image::open(&Path::new(img_path)) {
        Ok(img) => img, // Görüntü başarılı şekilde açılırsa img değişkenine ata
        Err(e) => {
            println!("Görüntü açılırken hata oluştu: {}", e);
            return; // Görüntü açılamazsa hata mesajı yazdır ve çık
        }
    };

    match action.as_str() {
        "encrypt" => {
            let corrupted_img = process_image(&img, key); // Görüntüyü encrypt
            match save_image(&corrupted_img, img_path) {
                Ok(_) => println!("Piksel encryptulmuş görüntü '{}' olarak kaydedildi.", img_path),
                Err(e) => println!("encryptuk görüntü kaydedilirken hata oluştu: {}", e),
            }
        }
        "decrypt" => {
            let fixed_img = process_image(&img, key); // Görüntüyü decrypt
            match save_image(&fixed_img, img_path) {
                Ok(_) => println!("Piksel decryptilmiş görüntü '{}' olarak kaydedildi.", img_path),
                Err(e) => println!("decryptilmiş görüntü kaydedilirken hata oluştu: {}", e),
            }
        }
        _ => println!("Geçersiz seçenek! 'encrypt' veya 'decrypt' seçeneğini kullanın."),
    }
}

fn generate_mask(width: u32, height: u32, key: &str) -> Vec<Rgba<u8>> {
    let mut hasher = Sha256::new(); // Sha256 hash nesnesi oluştur
    hasher.update(key); // Anahtarı hash nesnesine ekle
    let hash = hasher.finalize(); // Hash işlemini tamamla
    let seed: [u8; 32] = hash.into(); // Hash sonucunu seed olarak kullan
    let mut rng: StdRng = SeedableRng::from_seed(seed); // Seed'e dayalı rastgele sayı üretici oluştur

    (0..(width * height)).map(|_| {
        Rgba([
            rng.gen_range(0..=255), // Rastgele kırmızı bileşen
            rng.gen_range(0..=255), // Rastgele yeşil bileşen
            rng.gen_range(0..=255), // Rastgele mavi bileşen
            rng.gen_range(0..=255), // Rastgele alpha bileşen
        ])
    }).collect()
}

fn apply_mask(img: &DynamicImage, mask: &[Rgba<u8>]) -> DynamicImage {
    let (width, height) = img.dimensions(); // Görüntü boyutlarını al
    let mut masked_img = ImageBuffer::new(width, height); // Boş bir görüntü tamponu oluştur

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y); // Orijinal pikseli al
            let mask_pixel = &mask[(y * width + x) as usize]; // Maske pikselini al
            let new_pixel = Rgba([
                pixel[0] ^ mask_pixel[0], // XOR işlemi uygulayarak yeni kırmızı bileşen
                pixel[1] ^ mask_pixel[1], // XOR işlemi uygulayarak yeni yeşil bileşen
                pixel[2] ^ mask_pixel[2], // XOR işlemi uygulayarak yeni mavi bileşen
                pixel[3] ^ mask_pixel[3], // XOR işlemi uygulayarak yeni alpha bileşen
            ]);
            masked_img.put_pixel(x, y, new_pixel); // Yeni pikseli görüntüye koy
        }
    }

    DynamicImage::ImageRgba8(masked_img) // Maskelenmiş görüntüyü döndür
}

fn process_image(img: &DynamicImage, key: &str) -> DynamicImage {
    let (width, height) = img.dimensions(); // Görüntü boyutlarını al
    let mask = generate_mask(width, height, key); // Maske oluştur
    apply_mask(img, &mask) // Maskeyi görüntüye uygula
}

fn save_image(img: &DynamicImage, img_path: &str) -> Result<(), image::ImageError> {
    let mut output_path = Path::new(img_path).to_path_buf();
    output_path.set_extension("png"); // Çıktıyı PNG olarak kaydet

    img.save_with_format(output_path, image::ImageFormat::Png) // Görüntüyü PNG formatında kaydet
}
