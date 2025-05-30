use rustypipe::{
    client::{RustyPipe, RustyPipeQuery},
    model::{VideoItem}, // VideoItem を明示的に使う
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

    pub async fn search_youtube(&self, query : &str) -> Result<Vec<VideoItem>, String> {
        // 型を明示する（VideoItem を取得したいので）
        match self.client.search::<VideoItem, _>(query).await {
            Ok(result) => {
                
              let items = result.items.items;
                Ok(items)
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                Err("Error in Search Result".to_string())
            }
        }
    }
}