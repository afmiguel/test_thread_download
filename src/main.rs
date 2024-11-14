use std::time::{Duration, Instant};
mod download;
use download::download_file;

const URL: &str = "http://arquivos.afonsomiguel.com";

fn main() {
    let filename_list = vec![
        "arquivo_1.jpg",
        "arquivo_2.jpg",
        "arquivo_3.jpg",
        "arquivo_4.jpg",
        "arquivo_5.jpg",
        "arquivo_6.jpg",
        "arquivo_7.jpg",
        "arquivo_8.jpg",
        "arquivo_9.jpg",
    ];

    let start = Instant::now();
    for filename in filename_list {
        download_file(URL, filename);
    }
    let duration: Duration = start.elapsed();
    println!(
        "Downloaded files in {:.1} seconds",
        duration.as_millis() as f32 / 1000.
    );
}
