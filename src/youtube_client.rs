use rustypipe::{
    client::{RustyPipe, RustyPipeQuery},
    model::{VideoItem}, // VideoItem を明示的に使う
    param::StreamFilter,
};


pub struct YoutubeClient {
    client: RustyPipeQuery,
}

impl Default for YoutubeClient {
    fn default() -> Self {
        Self::new()
    }
}

impl YoutubeClient {
    pub fn new() -> Self {
        let rp = RustyPipe::new();
        let client = rp.query();
        YoutubeClient { client }
    }

    pub async fn search_youtube(&self) -> Result< String, String> {

        println!("Please enter search keywords");
        let mut query = String::new();
        std::io::stdin().read_line(&mut query).unwrap();
        let query = query.trim();

        // 型を明示する（VideoItem を取得したいので）
        match self.client.search::<VideoItem, _>(query).await {
            Ok(result) => {
               let url = self.select_url(result.items.items).await;
                Ok(url)
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                Err("Error in Search Result".to_string())
            }
        }
    }

    pub async fn select_url(&self ,items: Vec<VideoItem>) -> String{
        for (index, item) in items.iter().enumerate() {
            println!("{} {}", index + 1, item.name);
          }
        println!("\n");
        println!("Please select (enter number): ");

        loop{
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            match input.trim().parse::<usize>(){
                Ok(choice) => {
                    let url =   self.fetch_song_url(&items[choice - 1].id).await.unwrap();
                    return url;
                }
                Err(_) => {
                    println!("数字を入力してください")
                }
            }
        }  
         
    }

    pub async fn fetch_song_url(&self, id: &str) -> Result<String, String> {
        match self.client.player(&id).await {
            Ok(player) => match player.select_audio_stream(&StreamFilter::default()) {
                Some(stream) => Ok(stream.url.clone()),
                None => Err("Audio Stream not Found".to_string()),
            },
            Err(_) => Err("Link cannot be Found".to_string()),
        }
    }
}