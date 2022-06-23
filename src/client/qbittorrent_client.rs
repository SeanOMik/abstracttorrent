use crate::torrent::{TorrentInfo, TorrentTracker, TorrentUpload};
use super::{TorrentClient, ClientResult};

pub use qbittorrent;

use qbittorrent::{client::QBittorrentClient, common::GetTorrentListParams};

use async_trait::async_trait;

impl From<&qbittorrent::torrent::TorrentInfo> for TorrentInfo {
    fn from(torrent: &qbittorrent::torrent::TorrentInfo) -> Self {
        TorrentInfo {
            name: torrent.name.clone(),
            trackers: vec![torrent.tracker.clone()], // NOTE: qBittorrent only gives us one tracker.
            category: torrent.category.clone(),
            tags: torrent.tags.clone(),
            hash: torrent.hash.clone(),
            state: torrent.state.clone().into(),
        }
    }
}

// For some reason I have to implement this twice smh
impl Into<qbittorrent::torrent::TorrentInfo> for &TorrentInfo {
    fn into(self) -> qbittorrent::torrent::TorrentInfo {
        let mut t = qbittorrent::torrent::TorrentInfo::default();
        t.name = self.name.clone();
        t.tracker = self.trackers.first().unwrap_or(&String::new()).clone();
        t.category = self.category.clone();
        t.tags = self.tags.clone();
        t.hash = self.hash.clone();

        t
    }
}

// For some reason I have to implement this twice smh
impl Into<qbittorrent::torrent::TorrentInfo> for TorrentInfo {
    fn into(self) -> qbittorrent::torrent::TorrentInfo {
        let mut t = qbittorrent::torrent::TorrentInfo::default();
        t.name = self.name.clone();
        t.tracker = self.trackers.first().unwrap_or(&String::new()).clone();
        t.category = self.category.clone();
        t.tags = self.tags.clone();
        t.hash = self.hash.clone();

        t
    }
}

impl From<qbittorrent::torrent::TorrentUpload> for TorrentUpload {
    fn from(upload: qbittorrent::torrent::TorrentUpload) -> Self {
        TorrentUpload {
            urls: upload.urls,
            torrents: upload.torrents,
            tags: upload.tags,
            category: upload.category,
            paused: upload.paused,
        }
    }
}

impl Into<qbittorrent::torrent::TorrentUpload> for &TorrentUpload {
    fn into(self) -> qbittorrent::torrent::TorrentUpload {
        let mut t = qbittorrent::torrent::TorrentUpload::default();
        t.urls = self.urls.clone();
        t.torrents = self.torrents.clone();
        t.tags = self.tags.clone();
        t.category = self.category.clone();
        t.paused = self.paused;

        t
    }
}

impl From<&qbittorrent::torrent::TorrentTracker> for TorrentTracker {
    fn from(tracker: &qbittorrent::torrent::TorrentTracker) -> Self {
        TorrentTracker {
            url: tracker.url.clone(),
            status: tracker.status.clone().into(),
            message: Some(tracker.message.clone()),
        }
    }
}

impl From<qbittorrent::torrent::TrackerStatus> for crate::torrent::TrackerStatus {
    fn from(status: qbittorrent::torrent::TrackerStatus) -> Self {
        use qbittorrent::torrent::TrackerStatus as QBStatus;

        match status {
            QBStatus::Disabled => Self::Disabled,
            QBStatus::NotContacted => Self::NotContacted,
            QBStatus::Working => Self::Working,
            QBStatus::Updating => Self::Updating,
            QBStatus::NotWorking => Self::NotWorking,
        }
    }
}

impl From<qbittorrent::torrent::TorrentState> for crate::torrent::TorrentState {
    fn from(state: qbittorrent::torrent::TorrentState) -> Self {
        use qbittorrent::torrent::TorrentState as QBState;

        match state {
            QBState::Error => Self::Error,
            QBState::MissingFiles => Self::MissingFiles,
            QBState::Uploading | QBState::StalledUP | QBState::ForcedUP | QBState::CheckingUP => Self::Uploading,
            QBState::PausedUP => Self::PausedUploading,
            QBState::QueuedUP => Self::QueuedUploading,
            QBState::Downloading | QBState::StalledDL | QBState::ForcedDL |
                QBState::Allocating | QBState::MetaDownloading | QBState::CheckingDL |
                QBState::CheckingResumeData | QBState::Moving => Self::Downloading,
            QBState::PausedDL => Self::PausedDownloading,
            QBState::QueuedDL => Self::QueuedDownloading,
            QBState::Unknown => Self::Unknown,
        }
    }
}

#[async_trait]
impl TorrentClient for QBittorrentClient {
    async fn login(&mut self, url: &str, username: &str, password: &str) -> ClientResult<()> {
        Ok(Self::login(&mut self, url, username, password).await?)
    }

    async fn get_torrent_list(&self, params: Option<GetTorrentListParams>) -> ClientResult<Vec<TorrentInfo>> {
        Ok(Self::get_torrent_list(self, params).await?.iter().map(|t| t.into()).collect())
    }

    async fn get_torrent_trackers(&self, torrent: &TorrentInfo) -> ClientResult<Vec<TorrentTracker>> {
        Ok(Self::get_torrent_trackers(self, &torrent.into()).await?.iter().map(|t| t.into()).collect())
    }

    async fn add_torrent_tracker(&self, torrent: &TorrentInfo, tracker_url: String) -> ClientResult<()> {
        Ok(Self::add_torrent_tracker(self, &torrent.into(), tracker_url).await?)
    }

    async fn add_torrent_trackers(&self, torrent: &TorrentInfo, trackers: Vec<String>) -> ClientResult<()> {
        Ok(Self::add_torrent_trackers(self, &torrent.into(), trackers).await?)
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
        let torrents: Vec<qbittorrent::torrent::TorrentInfo> = torrents.iter().map(|t| t.into()).collect();
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