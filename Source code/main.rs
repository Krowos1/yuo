use std::process::{Command, Stdio};
use std::io::{self, Write};

fn main() {
    // Request input for the video link
    println!("Enter the video URL for downloading:");
    let mut video_url = String::new();
    io::stdin().read_line(&mut video_url).expect("Error reading input");
    let video_url = video_url.trim();

    // Command to display available formats
    println!("Fetching available audio formats...");
    let format_list = Command::new("yt-dlp")
        .arg("-F") // Show available formats
        .arg(video_url)
        .output()
        .expect("Failed to execute yt-dlp to fetch the format list");

    // Filter audio formats
    let format_output = String::from_utf8(format_list.stdout).unwrap();
    let audio_formats: Vec<&str> = format_output
        .lines()
        .filter(|line| line.contains("audio")) // Keep only lines with audio
        .collect();

    if audio_formats.is_empty() {
        println!("No audio formats found for this video.");
        return;
    }

    println!("Available audio formats:\n");
    for line in &audio_formats {
        println!("{}", line);
    }

    // Request input for the format from the user
    println!("\nEnter the audio format ID to download:");
    let mut format_id = String::new();
    io::stdin().read_line(&mut format_id).expect("Error reading input");
    let format_id = format_id.trim();

    // Download audio with speed optimization
    println!("Downloading audio...");
    let status = Command::new("yt-dlp")
        .args([
            "-f", format_id,             // Format to download
            "-o", "output.%(ext)s",      // Path for saving
            "--concurrent-fragments", "16", // 16 concurrent streams
            "--http-chunk-size", "10M",  // Chunk size
            "--force-ipv4",              // Use IPv4
            "--no-continue",             // Start download from scratch
        ])
        .arg(video_url)
        .status()
        .expect("Failed to execute yt-dlp for downloading");

    if status.success() {
        println!("Audio downloaded successfully!");
    } else {
        eprintln!("Error occurred while downloading audio.");
    }
}
