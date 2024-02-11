#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Playlist {
    pub name: String,
    pub id: String,
    pub channel: Channel,
    pub url: String,
    pub videos: Vec<Video>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Video {
    pub name: String,
    pub id: String,
    pub channel: Channel,
    pub url: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Channel {
    pub name: String,
    pub id: String,
    pub url: String,

    #[serde(skip)]
    pub videos: Option<Vec<Video>>,
    #[serde(skip)]
    pub playlists: Option<Vec<Playlist>>,
}
