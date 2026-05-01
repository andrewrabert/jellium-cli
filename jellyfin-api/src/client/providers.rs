use crate::types;
use crate::error::Error;
use crate::util::encode_path;
use crate::Client;

impl Client {
    #[doc = "Gets the remote lyrics\n\nSends a `GET` request to `/Providers/Lyrics/{lyricId}`\n\nArguments:\n- `lyric_id`: The remote provider item id.\n"]
    pub async fn get_remote_lyrics(
        &self,
        lyric_id: &str,
    ) -> Result<types::LyricDto, Error> {
        self.request(reqwest::Method::GET, format!("/Providers/Lyrics/{}", encode_path(lyric_id)))
            .send()
            .await
    }

    #[doc = "Gets the remote subtitles\n\nSends a `GET` request to `/Providers/Subtitles/Subtitles/{subtitleId}`\n\nArguments:\n- `subtitle_id`: The item id.\n"]
    pub async fn get_remote_subtitles(
        &self,
        subtitle_id: &str,
    ) -> Result<reqwest::Response, Error> {
        self.request(reqwest::Method::GET, format!("/Providers/Subtitles/Subtitles/{}", encode_path(subtitle_id)))
            .send_response()
            .await
    }
}
