// THIS FIRST SCRIPT WORKS ONLY THAT THE START AND END DURATION IS INCLUDED IN THE TEXT DOCUMENT

// use youtube_transcript::{Youtube, YoutubeBuilder, Transcript};
// use std::error::Error;
// use tokio::fs::File;
// use tokio::io::AsyncWriteExt;
// use std::time::Duration;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     // Define the YouTube video link
//     let link: &str = "https://www.youtube.com/watch?v=l17oRkhgqHE";

//     // Build the YouTube object
//     let binding = YoutubeBuilder::default();
//     let youtube_loader: Youtube = binding.build();

//     // Fetch the transcript
//     let transcript: Transcript = youtube_loader.transcript(link).await?;

//     // Create or open the transcript file
//     let mut file = File::create("transcript.txt").await?;

//     // Write the transcript to the file
//     for entry in transcript.transcripts {
//         let start = format_duration(entry.start);
//         let duration = format_duration(entry.duration);
//         let text = format!("{} (start: {}, duration: {})\n", entry.text, start, duration);
//         file.write_all(text.as_bytes()).await?;
//     }

//     println!("Transcript saved to transcript.txt");

//     Ok(())
// }

// fn format_duration(duration: Duration) -> String {
//     let secs = duration.as_secs();
//     let millis = duration.subsec_millis();
//     format!("{}.{:03}", secs, millis)
// }


use youtube_transcript::{Youtube, YoutubeBuilder, Transcript};
use std::error::Error;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Define the YouTube video link
    let link: &str = "https://www.youtube.com/watch?v=l17oRkhgqHE";

    // Build the YouTube object
    let binding = YoutubeBuilder::default();
    let youtube_loader: Youtube = binding.build();

    // Fetch the transcript
    let transcript: Transcript = youtube_loader.transcript(link).await?;

    // Create or open the transcript file
    let mut file = File::create("transcript1.txt").await?;

    // Write the cleaned transcript to the file
    for entry in transcript.transcripts {
        let text = format!("{}\n", entry.text);
        file.write_all(text.as_bytes()).await?;
    }
    
    //print that the transcript is saved 
    println!("Transcript saved to transcript1.txt");

    Ok(())
}
