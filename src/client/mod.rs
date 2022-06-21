pub mod qbittorrent;

use async_trait::async_trait;

use crate::{torrent::{TorrentInfo, TorrentTracker, TorrentUpload}, error::ClientError};

pub type ClientResult<T> = Result<T, ClientError>;

#[async_trait]
pub trait TorrentClient {
    async fn login(&mut self, url: &str, username: &str, password: &str) -> ClientResult<()>;

    async fn get_torrent_list(&self) -> ClientResult<Vec<TorrentInfo>>;

    async fn get_torrent_trackers(&self, torrent: &TorrentInfo) -> ClientResult<Vec<TorrentTracker>>;
    async fn add_torrent_tracker(&self, torrent: &TorrentInfo, tracker_url: String) -> ClientResult<()>;
    async fn replace_torrent_tracker(&self, torrent: &TorrentInfo, old_url: String, new_url: String) -> ClientResult<()>;
    async fn remove_torrent_tracker(&self, torrent: &TorrentInfo, tracker_url: String) -> ClientResult<()>;

    async fn add_torrent(&self, torrent: &TorrentUpload) -> ClientResult<()>;
    async fn remove_torrent(&self, torrent: &TorrentInfo, delete_files: bool) -> ClientResult<()>;
    async fn remove_torrents(&self, torrents: Vec<TorrentInfo>, delete_files: bool) -> ClientResult<()>;

    async fn get_tags(&self) -> ClientResult<Vec<String>>;
    async fn create_tag(&self, tag: &str) -> ClientResult<()>;
    async fn delete_tag(&self, tag: &str) -> ClientResult<()>;
}