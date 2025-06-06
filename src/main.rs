//LIBRARY_PATH="/opt/homebrew/lib" DYLD_LIBRARY_PATH="/opt/homebrew/lib" cargo run
//cargo run --release
// ./target/release/yt_player

use std::error::Error;
use std::io;
use std::thread;
use std::time::Duration;
use yt_player::player::Player;
use yt_player::youtube_client::YoutubeClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let yt_client = YoutubeClient::new();
    let mut player = Player::new()?;

    let result = yt_client.search_youtube().await?;
    player.play(&result, &result).unwrap();

    loop {
        let mut input = String::new();
        println!("Enter command (1:Play/Pause / 2:Search / 3:Exit):");
        io::stdin().read_line(&mut input).expect("");

        match input.trim() {
            "1" => {
                player.play_pause().unwrap();
            }
            "2" => {
                if let Ok(result) = yt_client.search_youtube().await {
                    player.play(&result, &result).unwrap();
                }
            }
            "3" => {
                println!("Exiting...");
                return Ok(());
            }
            "4" => {
                println!("スリープタイマーをセット(分)");

                loop{
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
        
                    match input.trim().parse::<u64>(){
                        Ok(choice) => {
                            thread::sleep(Duration::from_secs(choice));
                            return Ok(());
                        }
                        Err(_) => {
                            println!("数字を入力してください")
                        }
                    }
                }  
            }
            _ => {
                println!("無効なコマンドです");
                continue;
            }
        };
    }
}
