use async_trait::async_trait;

use crate::{torrent::{TorrentInfo, TorrentTracker}, error::ClientError};

pub type ClientResult<T> = Result<T, ClientError>;

#[async_trait]
pub trait TorrentClient {
    async fn login(&self, url: &str, username: &str, password: &str) -> ClientResult<()>;

    async fn get_torrent_list(&self) -> ClientResult<Vec<TorrentInfo>>;

    async fn get_torrent_trackers(&self, torrent: &TorrentInfo) -> ClientResult<Vec<TorrentTracker>>;
    async fn add_torrent_Tracker(&self, torrent: &TorrentInfo, tracker_url: String) -> ClientResult<()>;
    async fn replace_torrent_Tracker(&self, torrent: &TorrentInfo, old_url: String, new_url: String) -> ClientResult<()>;
    async fn remove_torrent_Tracker(&self, torrent: &TorrentInfo, tracker_url: String) -> ClientResult<()>;


    async fn add_torrent(&self, torrent: &TorrentInfo) -> ClientResult<()>;
    async fn remove_torrent(&self, torrent: &TorrentInfo, delete_files: bool) -> ClientResult<()>;
}