use std::io;
use std::fs::File;

pub fn downloadtest(testname: &str, filenametosavedownload: &str) {
    let url = format!("{}{}{}", "http://antros.zst-grudziadz.pl/testak/sda222opAsDlwp12/", testname, "/data.tst");

    let mut resp = reqwest::blocking::get(&url)
    .expect("Request failed");
    
    assert_eq!(resp.status(), reqwest::StatusCode::OK, "Test {} not found on the server! Response code: {}", testname, resp.status());
    
    let mut out = File::create(&filenametosavedownload).expect("failed to create file");
    io::copy(&mut resp, &mut out).expect("Failed to copy content");
}