use std::fs;
use encoding_rs;
use std::env;
mod decipher;
mod downloader;


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut bytestream: Vec<u8>;

    if args.len() < 4 {
        eprintln!("Usage: papajak-extended [<filename_to_read:string> or <filename_to_save_download:string>] <testpassword:string> <answers_file_name:string> (optional) <testname_to_download:string>");
        return;
    }

    let password: &String = &args[2];
    let answers_file_name: &String = &args[3];

    if args.len() == 4 {
        // Local mode
        let file_name_to_read: &String = &args[1];
        
        bytestream = fs::read(&file_name_to_read)
        .expect("Should have been able to read the file");

    } else if args.len() == 5 {
        let filename_to_save_download: &String = &args[1];
        let testname_to_download: &String = &args[4];

        downloader::downloadtest(&testname_to_download, &filename_to_save_download);
        bytestream = fs::read(&filename_to_save_download)
        .expect("Should have been able to read the file");
        
    } else {
        eprintln!("Usage: papajak-extended [<filename_to_read:string> or <filename_to_save_download:string>] <testpassword:string> <answers_file_name:string> (optional) <testname_to_download:string>");
        return;
    }

    // Decrypt vector
    decipher::decrypt(&mut bytestream, &password);

    // Decode vector
    let (cow, encoding_used, had_errors) = encoding_rs::WINDOWS_1250.decode(&bytestream);
    assert_eq!(encoding_used, encoding_rs::WINDOWS_1250, "The encoding used does not match the preferred one");
    assert!(!had_errors, "Decoding errors found");
    
    // Create string
    let decoded_string: String = cow.to_string();

    // Check if the string contains a password - this is a condition for correct decryption
    assert!(decoded_string.contains(password), "Password not found in decoded string, which means you propably entered the wrong password");

    // Save string
    fs::write(answers_file_name, decoded_string)
    .expect("Should have been able to write the file");
}
