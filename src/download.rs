use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

/// Downloads a file from the specified URL and saves it to the local directory.
///
/// # Arguments
/// * `url` - Base URL where the file is hosted.
/// * `filename` - Name of the file to be downloaded and saved locally.
pub fn download_file(url: &str, filename: &str) {

    // Send a GET request to the server to retrieve the file
    let mut resp = reqwest::blocking::get(format!("{}/{}", url, filename))
        .expect("Failed to send request");

    // Define the path where the downloaded file will be saved
    let str_path = format!("downloads");
    let path = Path::new(str_path.as_str());

    // Create the "downloads" directory if it doesn't already exist
    std::fs::create_dir_all(path).expect("Failed to create directory");

    // Create a file at the specified path to save the downloaded content
    let mut out = File::create(path.join(filename)).expect("Failed to create file");

    // Copy the content from the response into the file
    io::copy(&mut resp, &mut out).expect("Failed to copy content");

    // Print success message
    println!("Download {} OK! ", &filename);
}
