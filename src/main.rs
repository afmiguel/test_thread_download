use std::time::{Duration, Instant};
mod download;
use download::download_file;

const URL: &str = "http://arquivos.afonsomiguel.com";

fn main() {
    // Download Sequencial
    let start = Instant::now();
    for f_cont in 0..=9 {
        download_file(
            URL,
            format!("arquivo_{}.jpg", f_cont).as_str(),
        );
    }
    let duration: Duration = start.elapsed();
    println!(
        "Download em {:.1} segundos",
        duration.as_millis() as f32 / 1000.
    );
}
