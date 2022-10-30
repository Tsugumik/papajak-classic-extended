use std::fs;
mod decipher;


fn main() {
    
    // Open file as bytes vector.
    let mut bytestream: Vec<u8> = fs::read("./data.tst")
    .expect("Should have been able to read the file");
    
    // Decrypt vector
    decipher::decrypt(&mut bytestream, "artykulacja");

    // Save bytestream as string
    let mut decryptedoutput: String = String::from("");

    for byte in bytestream.iter() {
        decryptedoutput.push(*byte as char);
    }

    fs::write("./output.txt", decryptedoutput)
    .expect("Should have been able to write the file");
    
}
