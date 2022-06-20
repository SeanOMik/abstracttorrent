use crate::torrent::{TorrentInfo, TorrentTracker, TorrentUpload};
use super::{TorrentClient, ClientResult};
use qbittorrent::QBittorrentClient;

use async_trait::async_trait;

impl From<&qbittorrent::TorrentInfo> for TorrentInfo {
    fn from(torrent: &qbittorrent::TorrentInfo) -> Self {
        TorrentInfo {
            name: torrent.name.clone(),
            trackers: vec![torrent.tracker.clone()], // NOTE: qBittorrent only gives us one tracker.
            category: torrent.category.clone(),
            tags: torrent.tags.clone(),
            hash: torrent.hash.clone(),
        }
    }
}

// For some reason I have to implement this twice smh
impl Into<qbittorrent::TorrentInfo> for &TorrentInfo {
    fn into(self) -> qbittorrent::TorrentInfo {
        let mut t = qbittorrent::TorrentInfo::default();
        t.name = self.name.clone();
        t.tracker = self.trackers.first().unwrap_or(&String::new()).clone();
        t.category = self.category.clone();
        t.tags = self.tags.clone();
        t.hash = self.hash.clone();

        t
    }
}

// For some reason I have to implement this twice smh
impl Into<qbittorrent::TorrentInfo> for TorrentInfo {
    fn into(self) -> qbittorrent::TorrentInfo {
        let mut t = qbittorrent::TorrentInfo::default();
        t.name = self.name.clone();
        t.tracker = self.trackers.first().unwrap_or(&String::new()).clone();
        t.category = self.category.clone();
        t.tags = self.tags.clone();
        t.hash = self.hash.clone();

        t
    }
}

impl From<qbittorrent::TorrentUpload> for TorrentUpload {
    fn from(upload: qbittorrent::TorrentUpload) -> Self {
        TorrentUpload {
            urls: upload.urls,
            torrents: upload.torrents,
            tags: upload.tags,
            category: upload.category,
            paused: upload.paused,
        }
    }
}

impl Into<qbittorrent::TorrentUpload> for &TorrentUpload {
    fn into(self) -> qbittorrent::TorrentUpload {
        let mut t = qbittorrent::TorrentUpload::default();
        t.urls = self.urls.clone();
        t.torrents = self.torrents.clone();
        t.tags = self.tags.clone();
        t.category = self.category.clone();
        t.paused = self.paused;

        t
    }
}

impl From<&qbittorrent::TorrentTracker> for TorrentTracker {
    fn from(tracker: &qbittorrent::TorrentTracker) -> Self {
        TorrentTracker {
            url: tracker.url.clone(),
            status: tracker.status.clone().into(),
            message: Some(tracker.message.clone()),
        }
    }
}

impl From<qbittorrent::TrackerStatus> for crate::torrent::TrackerStatus {
    fn from(status: qbittorrent::TrackerStatus) -> Self {
        match status {
            qbittorrent::TrackerStatus::Disabled => crate::torrent::TrackerStatus::Disabled,
            qbittorrent::TrackerStatus::NotContacted => crate::torrent::TrackerStatus::NotContacted,
            qbittorrent::TrackerStatus::Working => crate::torrent::TrackerStatus::Working,
            qbittorrent::TrackerStatus::Updating => crate::torrent::TrackerStatus::Updating,
            qbittorrent::TrackerStatus::NotWorking => crate::torrent::TrackerStatus::NotWorking,
        }
    }
}

#[async_trait]
impl<'a> TorrentClient<'a> for QBittorrentClient<'a> {
    async fn login(&mut self, url: &'a str, username: &'a str, password: &'a str) -> ClientResult<()> {
        Ok(Self::login(&mut self, url, username, password).await?)
    }

    async fn get_torrent_list(&self) -> ClientResult<Vec<TorrentInfo>> {
        Ok(Self::get_torrent_list(self).await?.iter().map(|t| t.into()).collect())
    }

    async fn get_torrent_trackers(&self, torrent: &TorrentInfo) -> ClientResult<Vec<TorrentTracker>> {
        Ok(Self::get_torrent_trackers(self, &torrent.into()).await?.iter().map(|t| t.into()).collect())
    }

    async fn add_torrent_tracker(&self, torrent: &TorrentInfo, tracker_url: String) -> ClientResult<()> {
        Ok(Self::add_torrent_tracker(self, &torrent.into(), tracker_url).await?)
    }

    async fn replace_torrent_tracker(&self, torrent: &TorrentInfo, old_url: String, new_url: String) -> ClientResult<()> {
        Ok(Self::replace_torrent_tracker(self, &torrent.into(), old_url, new_url).await?)
    }

    async fn remove_torrent_tracker(&self, torrent: &TorrentInfo, tracker_url: String) -> ClientResult<()> {
        Ok(Self::remove_torrent_tracker(self, &torrent.into(), tracker_url).await?)
    }

    async fn add_torrent(&self, torrent: &TorrentUpload) -> ClientResult<()> {
        Ok(Self::add_torrent(self, &torrent.into()).await?)
    }

    async fn remove_torrent(&self, torrent: &TorrentInfo, delete_files: bool) -> ClientResult<()> {
        Ok(Self::remove_torrent(self, &torrent.into(), delete_files).await?)
    }

    async fn remove_torrents(&self, torrents: Vec<TorrentInfo>, delete_files: bool) -> ClientResult<()> {
        let torrents: Vec<qbittorrent::TorrentInfo> = torrents.iter().map(|t| t.into()).collect();
        Ok(Self::remove_torrents(self, torrents, delete_files).await?)
    }

    async fn get_tags(&self) -> ClientResult<Vec<String>> {
        Ok(Self::get_tags(self).await?)
    }

    async fn create_tag(&self, tag: &str) -> ClientResult<()> {
        Ok(Self::create_tag(self, tag).await?)
    }

    async fn delete_tag(&self, tag: &str) -> ClientResult<()> {
        Ok(Self::delete_tag(self, tag).await?)
    }
}