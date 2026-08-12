pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "source_type")]
#[non_exhaustive]
pub enum ProjectExternalAudioResponseModelSourceContext {
        #[serde(rename = "music_explore_song")]
        #[non_exhaustive]
        MusicExploreSong {
            #[serde(default)]
            music_explore_song_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            title: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            description: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            bpm: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            vocals: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            lyrics: Option<String>,
        },

        #[serde(rename = "sfx")]
        #[non_exhaustive]
        Sfx {
            #[serde(skip_serializing_if = "Option::is_none")]
            sound_generation_history_item_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            text: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            generation_config: Option<HashMap<String, serde_json::Value>>,
        },

        #[serde(rename = "song")]
        #[non_exhaustive]
        Song {
            #[serde(default)]
            song_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            chat_id: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            title: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            description: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            genres: Option<Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            languages: Option<Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            is_explicit: Option<bool>,
            #[serde(skip_serializing_if = "Option::is_none")]
            bpm: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            generation_settings: Option<HashMap<String, serde_json::Value>>,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl ProjectExternalAudioResponseModelSourceContext {
    pub fn music_explore_song(music_explore_song_id: String) -> Self {
        Self::MusicExploreSong { music_explore_song_id, title: None, description: None, bpm: None, vocals: None, lyrics: None }
    }

    pub fn sfx() -> Self {
        Self::Sfx { sound_generation_history_item_id: None, text: None, generation_config: None }
    }

    pub fn song(song_id: String) -> Self {
        Self::Song { song_id, chat_id: None, title: None, description: None, genres: None, languages: None, is_explicit: None, bpm: None, generation_settings: None }
    }

    pub fn music_explore_song_with_title(music_explore_song_id: String, title: String, description: Option<String>, bpm: Option<i64>, vocals: Option<String>, lyrics: Option<String>) -> Self {
        Self::MusicExploreSong { music_explore_song_id, title: Some(title), description, bpm, vocals, lyrics }
    }

    pub fn music_explore_song_with_description(music_explore_song_id: String, title: Option<String>, description: String, bpm: Option<i64>, vocals: Option<String>, lyrics: Option<String>) -> Self {
        Self::MusicExploreSong { music_explore_song_id, title, description: Some(description), bpm, vocals, lyrics }
    }

    pub fn music_explore_song_with_bpm(music_explore_song_id: String, title: Option<String>, description: Option<String>, bpm: i64, vocals: Option<String>, lyrics: Option<String>) -> Self {
        Self::MusicExploreSong { music_explore_song_id, title, description, bpm: Some(bpm), vocals, lyrics }
    }

    pub fn music_explore_song_with_vocals(music_explore_song_id: String, title: Option<String>, description: Option<String>, bpm: Option<i64>, vocals: String, lyrics: Option<String>) -> Self {
        Self::MusicExploreSong { music_explore_song_id, title, description, bpm, vocals: Some(vocals), lyrics }
    }

    pub fn music_explore_song_with_lyrics(music_explore_song_id: String, title: Option<String>, description: Option<String>, bpm: Option<i64>, vocals: Option<String>, lyrics: String) -> Self {
        Self::MusicExploreSong { music_explore_song_id, title, description, bpm, vocals, lyrics: Some(lyrics) }
    }

    pub fn sfx_with_sound_generation_history_item_id(sound_generation_history_item_id: String, text: Option<String>, generation_config: Option<HashMap<String, serde_json::Value>>) -> Self {
        Self::Sfx { sound_generation_history_item_id: Some(sound_generation_history_item_id), text, generation_config }
    }

    pub fn sfx_with_text(sound_generation_history_item_id: Option<String>, text: String, generation_config: Option<HashMap<String, serde_json::Value>>) -> Self {
        Self::Sfx { sound_generation_history_item_id, text: Some(text), generation_config }
    }

    pub fn sfx_with_generation_config(sound_generation_history_item_id: Option<String>, text: Option<String>, generation_config: HashMap<String, serde_json::Value>) -> Self {
        Self::Sfx { sound_generation_history_item_id, text, generation_config: Some(generation_config) }
    }

    pub fn song_with_chat_id(song_id: String, chat_id: String, title: Option<String>, description: Option<String>, genres: Option<Vec<String>>, languages: Option<Vec<String>>, is_explicit: Option<bool>, bpm: Option<i64>, generation_settings: Option<HashMap<String, serde_json::Value>>) -> Self {
        Self::Song { song_id, chat_id: Some(chat_id), title, description, genres, languages, is_explicit, bpm, generation_settings }
    }

    pub fn song_with_title(song_id: String, chat_id: Option<String>, title: String, description: Option<String>, genres: Option<Vec<String>>, languages: Option<Vec<String>>, is_explicit: Option<bool>, bpm: Option<i64>, generation_settings: Option<HashMap<String, serde_json::Value>>) -> Self {
        Self::Song { song_id, chat_id, title: Some(title), description, genres, languages, is_explicit, bpm, generation_settings }
    }

    pub fn song_with_description(song_id: String, chat_id: Option<String>, title: Option<String>, description: String, genres: Option<Vec<String>>, languages: Option<Vec<String>>, is_explicit: Option<bool>, bpm: Option<i64>, generation_settings: Option<HashMap<String, serde_json::Value>>) -> Self {
        Self::Song { song_id, chat_id, title, description: Some(description), genres, languages, is_explicit, bpm, generation_settings }
    }

    pub fn song_with_genres(song_id: String, chat_id: Option<String>, title: Option<String>, description: Option<String>, genres: Vec<String>, languages: Option<Vec<String>>, is_explicit: Option<bool>, bpm: Option<i64>, generation_settings: Option<HashMap<String, serde_json::Value>>) -> Self {
        Self::Song { song_id, chat_id, title, description, genres: Some(genres), languages, is_explicit, bpm, generation_settings }
    }

    pub fn song_with_languages(song_id: String, chat_id: Option<String>, title: Option<String>, description: Option<String>, genres: Option<Vec<String>>, languages: Vec<String>, is_explicit: Option<bool>, bpm: Option<i64>, generation_settings: Option<HashMap<String, serde_json::Value>>) -> Self {
        Self::Song { song_id, chat_id, title, description, genres, languages: Some(languages), is_explicit, bpm, generation_settings }
    }

    pub fn song_with_is_explicit(song_id: String, chat_id: Option<String>, title: Option<String>, description: Option<String>, genres: Option<Vec<String>>, languages: Option<Vec<String>>, is_explicit: bool, bpm: Option<i64>, generation_settings: Option<HashMap<String, serde_json::Value>>) -> Self {
        Self::Song { song_id, chat_id, title, description, genres, languages, is_explicit: Some(is_explicit), bpm, generation_settings }
    }

    pub fn song_with_bpm(song_id: String, chat_id: Option<String>, title: Option<String>, description: Option<String>, genres: Option<Vec<String>>, languages: Option<Vec<String>>, is_explicit: Option<bool>, bpm: i64, generation_settings: Option<HashMap<String, serde_json::Value>>) -> Self {
        Self::Song { song_id, chat_id, title, description, genres, languages, is_explicit, bpm: Some(bpm), generation_settings }
    }

    pub fn song_with_generation_settings(song_id: String, chat_id: Option<String>, title: Option<String>, description: Option<String>, genres: Option<Vec<String>>, languages: Option<Vec<String>>, is_explicit: Option<bool>, bpm: Option<i64>, generation_settings: HashMap<String, serde_json::Value>) -> Self {
        Self::Song { song_id, chat_id, title, description, genres, languages, is_explicit, bpm, generation_settings: Some(generation_settings) }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
