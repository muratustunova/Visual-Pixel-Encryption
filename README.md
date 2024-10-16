# Encrypt pixels of images
Download and install Rust libraries. Then you should open an empty project. Add the codes in the folder I shared into the "main.rs" file of the project. You should do the same for the "Cargo.toml" file.

# How to use encryption for pixels
Select the image you want to encrypt and enter the following command. In place of the letter **"X"**, enter the path of the image you want to encrypt.

```
cargo run your/path/x.png encrypt [your password]
```

# How to use decryption for pixel
```
cargo run [(The file path is based on the same directory as the main.rs file.)x.png] decrypt [your current password]
```


