pub fn decrypt(bytestream: &mut Vec<u8>, password: &str){

    let mut address = 1;

    for byte in &mut bytestream.iter_mut() {
        *byte ^= (password.as_bytes()[address % password.len()] as usize + address) as u8;
        address += 1;
    }
}