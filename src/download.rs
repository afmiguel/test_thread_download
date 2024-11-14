use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

pub fn download_file(url: &str, filename: &str) {
    print!("Baixando {} ... ", &filename);
    io::stdout().flush().unwrap();
    let mut resp = reqwest::blocking::get(format!("{}/{}", url, filename)).expect("request failed");
    let str_path = format!("downloads/{}", filename);
    let path = Path::new(str_path.as_str());
    std::fs::create_dir_all(path).expect("failed to create directory");
    let mut out = File::create(path.join(filename)).expect("failed to create file");
    io::copy(&mut resp, &mut out).expect("failed to copy content");
    println!("OK!");
}
