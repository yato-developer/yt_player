//LIBRARY_PATH="/opt/homebrew/lib" DYLD_LIBRARY_PATH="/opt/homebrew/lib" cargo run
//cargo run --release

use std::error::Error;
use yt_player::player::Player;
use yt_player::youtube_client::YoutubeClient;
use std::io;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let yt_client = YoutubeClient::new();
    let mut player = Player::new()?;


    let result = yt_client.search_youtube().await?;
    player.play(&result, &result).unwrap();

    loop {
        let mut input = String::new();
        println!("Enter command (1:Play/Pause Toggle/2:Search):");
        io::stdin()
            .read_line(&mut input)
            .expect("");

        match input.trim() {
            "1" => {
                player.play_pause().unwrap();
            },
            "2" => {
                if let Ok(result) = yt_client.search_youtube().await {
                    player.play(&result, &result).unwrap();
                }
            },
            _ => {
                println!("無効なコマンドです");
                continue;
            }
        };
    }
}
