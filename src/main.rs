use std::error::Error;
use yt_player::player::Player;
use yt_player::youtube_client::YoutubeClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let yt_client = YoutubeClient::new();
    let mut player = Player::new()?;

    loop {
        println!("\n検索キーワードを入力してください（終了するには 'q' を入力）: ");
        let mut query = String::new();
        std::io::stdin().read_line(&mut query)?;
        let query = query.trim();

        if query == "q" {
            break;
        }

        let result = yt_client.search_youtube(query).await?;

        for (index, item) in result.iter().enumerate() {
            println!("{}. {}", index + 1, item.name);
        }

        println!("選択してください（番号を入力）: ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let choice: usize = input.trim().parse()?;

        if choice > 0 && choice <= result.len() {
            let selected_item = &result[choice - 1];
            println!("選択された動画: {}", selected_item.name);
            player.play(&selected_item.name, &selected_item.id)?;
        } else {
            println!("無効な選択です");
        }
    }

    Ok(())
}
