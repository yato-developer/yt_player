use libmpv2::Mpv; // We are not using libmpv library because it was requiring user to install an old version which was not available in many distros so we decided to opt for libmpv2 which is a fork of it
use std::{f32::MANTISSA_DIGITS, sync::Arc};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::fs::{create_dir_all};

pub struct Player {
    /// An instance of the MPV player wrapped in an `Arc` for thread safety.
    pub player: Arc<Mpv>,
    pub current_song: Option<Song>,
}

#[derive(Debug, thiserror::Error)]
pub enum MpvError {
    #[error("Mpv error: {0}")]
    Mpv(#[from] libmpv2::Error),
    #[error("Failed to initialize MPV")]
    InitializationError,
    #[error("Command execution failed: {0}")]
    CommandError(String),
    #[error("Failed to load file: {0}")]
    LoadFileError(String),
    #[error("Property retrieval failed: {0}")]
    PropertyError(String),
    #[error("Unknown error: {0}")]
    Other(String),
}

pub enum PlayerControllerStatus {
    Playing,
    Paused,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Song {
    title: String,
    url: String,
}

#[derive(Debug, Deserialize)]
pub struct MusicList {
    pub songs: Vec<Song>,
}


impl Player {
    /// Creates a new `Player` instance and configures MPV settings for optimized audio playback.
    pub fn new() -> Result<Self, MpvError> {
        let mpv = Mpv::new()?;

        mpv.set_property("video", "no")?;
        mpv.set_property("loop", "no")?;

        mpv.set_property("ytdl-raw-options", "no-check-certificate=")?;
        mpv.set_property("loop", "inf")?; // Looping enabled (to be removed with autoplay)
        mpv.set_property(
            "http-header-fields",
            "User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64)",
        )?;

        // Audio optimization
        mpv.set_property("audio-buffer", 0.1)?; // 100ms audio buffer
        mpv.set_property("audio-channels", "stereo")?; // Force stereo audio

        let mpv = Arc::new(mpv);
        Ok(Self { player: mpv, current_song: None })
    }


    pub fn player_controller(&self ,command : PlayerControllerStatus){
        match command {
        PlayerControllerStatus::Playing => {
            self.unpause().unwrap();
        },
        PlayerControllerStatus::Paused => {
            self.pause().unwrap();
        },
        }
    }

    pub fn current_play(&mut self, title: &str, url: &str) {
        self.current_song = Some(Song {
            title: title.to_string(),
            url: url.to_string(),
        });
    }

    /// Loads and plays a media file from a given URL.
    pub fn play(&mut self,title : &str, url: &str, ) -> Result<(), MpvError> {
         if let Ok(true) = self.player.get_property("pause") {
            self.unpause()?;
        } // Quick fix will improve 
        self.player.command("loadfile", &[url])?; // Replace the current playback
        self.current_play(title, url);
        Ok(())
    }

    /// Pauses playback.
    pub fn pause(&self) -> Result<(), MpvError> {
        self.player.command("set", &["pause", "yes"])?;
        Ok(())
    }

    /// Resumes playback.
    pub fn unpause(&self) -> Result<(), MpvError> {
        self.player.command("set", &["pause", "no"])?;
        Ok(())
    }

    /// Toggles between play and pause states.
    pub fn play_pause(&self) -> Result<(), MpvError> {
        match self.player.get_property::<bool>("pause") {
            Ok(true) => self.unpause()?,
            Ok(false) => self.pause()?,
            Err(_) => todo!(),
        }
        Ok(())
    }

    /// Seeks forward by 5 seconds in the current track.
    pub fn seek_forward(&self) -> Result<(), MpvError> {
        self.player.command("seek", &["5", "relative"])?;
        Ok(())
    }

    /// Seeks backward by 5 seconds in the current track.
    pub fn seek_backword(&self) -> Result<(), MpvError> {
        self.player.command("seek", &["-5", "relative"])?;
        Ok(())
    }

    /// Retrieves the current playback time as a string.
    pub fn get_current_time(&self) -> String {
        self.player
            .get_property("time-pos")
            .unwrap_or(0.0)
            .to_string()
    }

    /// Retrieves the duration of the currently playing media.
    pub fn duration(&self) -> String {
        self.player
            .get_property("duration")
            .unwrap_or(0.0)
            .to_string()
    }

    /// Returns whether a media file is currently playing.
    pub fn is_playing(&self) -> Result<bool, MpvError> {
        let pause: bool = self.player.get_property("pause")?;
        Ok(!pause)
    }

    pub fn fetch_music_list(&self) -> MusicList{
        let mut f = File::open("data/music_list.json").expect("file not found");

        let mut contents = String::new();
        f.read_to_string(&mut contents)
            // ファイルの読み込み中に問題がありました
            .expect("something went wrong reading the file");

        let music_list: MusicList = serde_json::from_str(&contents).expect("JSON decode error");

        music_list
    }
}
