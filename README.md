# Image Pixel Encryption/Decryption Tool  

This Rust project provides a lightweight tool to encrypt and decrypt image pixels using a **key-based random mask**. It's ideal for experimenting with visual encryption techniques or adding a layer of obfuscation to image data.  

## 🔧 Features  
- **Encrypt and Decrypt Images**: Apply a pixel-wise XOR operation with a random mask generated from a secure key.  
- **Key-Based Randomization**: Uses SHA-256 hashing and a seeded random generator to ensure consistent encryption and decryption.  
- **Supports PNG Format**: Process images and save encrypted or decrypted results in PNG format.  

## 🚀 How It Works  
1. A mask is generated based on the SHA-256 hash of the input key.  
2. Each pixel in the image is XORed with the corresponding mask value.  
3. The encrypted image is saved with a unique filename in the `output/` directory.  

### **Encryption Process**  
Given a key, the program generates a unique mask for the image. The mask ensures encryption is consistent for the same key and image combination.  

### **Decryption Process**  
Decrypting an image requires the same key used during encryption. Applying the XOR operation again restores the original image.  

## 📂 File Structure  
- **Input Image**: The image to be encrypted or decrypted.  
- **Output Directory**: Stores encrypted and decrypted images (`output/encrypted_*.png`, `output/decrypted_*.png`).  

## 📋 Usage  
### **Command**  
```bash
cargo run <image_path> <encrypt|decrypt> <key>
```
## Examples
### Encrypt an Image
```bash
cargo run ./input_image.png encrypt my_secret_key
```
Output: output/encrypted_input_image.png

### Decrypt an Image
```bash
cargo run ./output/encrypted_input_image.png decrypt my_secret_key
```
Output: output/decrypted_input_image.png

### How the Mask Is Generated
The mask is created using:

A SHA-256 hash of the key.
A seeded random number generator (StdRng) initialized with the hash.
Random RGBA pixel values for the entire image.
This ensures that the mask is consistent for the same key and image dimensions.

⚠️ Limitations
Output File Size: The encrypted images are saved as PNGs, which may increase file size.
Key Sensitivity: Decryption requires the exact same key as encryption.

📥 Installation
Clone this repository:
```bash
git clone https://github.com/<your-username>/<repo-name>.git
cd <repo-name>
```
Build the project:
```bash
cargo build
```
🤝 Contributing
Contributions are welcome! Feel free to fork the repository and submit a pull request.

📌 License
This project is licensed under the MIT License.
