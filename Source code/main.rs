use std::process::{Command, Stdio};
use std::io::{self, Write};
use std::path::Path;

fn main() {
    // Path to the local yt-dlp binary (should be in the same directory as the exe)
    let yt_dlp_path = "./yt-dlp";

    // Check if the yt-dlp binary exists in the program's directory
    if !Path::new(yt_dlp_path).exists() {
        eprintln!("Error: yt-dlp not found. Place yt-dlp in the same directory as this program.");
        return;
    }

    // Prompt the user for a video URL
    println!("Enter the video URL to download:");
    let mut video_url = String::new();
    io::stdin().read_line(&mut video_url).expect("Failed to read input");
    let video_url = video_url.trim();

    // Command to list available formats
    println!("Fetching available audio formats...");
    let format_list = Command::new(yt_dlp_path)
        .arg("-F") // Show available formats
        .arg(video_url)
        .output()
        .expect("Failed to run yt-dlp to fetch format list");

    // Filter for audio formats
    let format_output = String::from_utf8(format_list.stdout).unwrap();
    let audio_formats: Vec<&str> = format_output
        .lines()
        .filter(|line| line.contains("audio")) // Keep only audio formats
        .collect();

    if audio_formats.is_empty() {
        println!("No audio formats found for this video.");
        return;
    }

    println!("Available audio formats:\n");
    for line in &audio_formats {
        println!("{}", line);
    }

    // Prompt the user for a format ID
    println!("\nEnter the format ID to download:");
    let mut format_id = String::new();
    io::stdin().read_line(&mut format_id).expect("Failed to read input");
    let format_id = format_id.trim();

    // Download the selected audio format
    println!("Downloading audio...");
    let status = Command::new(yt_dlp_path)
        .args([
            "-f", format_id,             // Specify the format
            "-o", "output.%(ext)s",      // Save file with original extension
            "--concurrent-fragments", "16", // Use 16 simultaneous download threads
            "--http-chunk-size", "10M",  // Chunk size of 10 MB
            "--force-ipv4",              // Force IPv4
        ])
        .arg(video_url)
        .status()
        .expect("Failed to run yt-dlp for downloading");

    if status.success() {
        println!("Audio downloaded successfully!");
    } else {
        eprintln!("Error during audio download.");
    }
}
