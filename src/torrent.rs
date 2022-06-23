#[derive(Debug, Default, Clone)]
pub struct TorrentInfo {
    pub name: String,
    pub trackers: Vec<String>,
    pub category: String,
    pub tags: Vec<String>,
    pub hash: String,
    pub state: TorrentState,
}

impl TorrentInfo {
    pub fn from_hash(hash: String) -> TorrentInfo {
        let mut def = TorrentInfo::default();
        def.hash = hash;

        def
    }
}

#[derive(Debug, Clone)]
pub enum TorrentState {
    /// Some error occurred
    Error,

    /// Torrent data files is missing
    MissingFiles,

    /// Torrent is being seeded and data is being transferred
    Uploading,

    /// Torrent is paused  but in a seeding state.
    PausedUploading,

    /// Torrent is queued but in a seeding state.
    QueuedUploading,

    /// Torrent is being downloaded and data is being transferred
    Downloading,

    /// Torrent is paused but in a downloading state.
    PausedDownloading,

    /// Torrent is queued but in a downloading state.
    QueuedDownloading,

    /// The state is unknown
    Unknown,
}

impl Default for TorrentState {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Debug, Default, Clone)]
pub struct TorrentUpload {
    /// URL(s) of the torrent files.
    pub urls: Vec<String>,

    /// Binary data of the torrents that are being added.
    /// Torrent file data that is being added. (Name, Bytes)
    pub torrents: Vec<(String, Vec<u8>)>,

    pub tags: Option<Vec<String>>,

    pub category: Option<String>,

    pub paused: Option<bool>,
}

impl TorrentUpload {
    pub fn builder() -> TorrentUploadBuilder {
        TorrentUploadBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct TorrentUploadBuilder {
    params: TorrentUpload
}

impl TorrentUploadBuilder {
    pub fn url(&mut self, url: &str) -> &mut Self {
        self.params.urls.push(url.to_string());
        self
    }

    pub fn torrent_file(&mut self, torrent_path: &str) -> &mut Self {
        let path = std::path::Path::new(torrent_path);
        
        self.torrent_path(path)
    }

    pub fn torrent_path(&mut self, torrent_path: &std::path::Path) -> &mut Self {
        let torrents = &mut self.params.torrents;
        torrents.push((
            torrent_path.file_name().unwrap().to_str().unwrap().to_string(),
            std::fs::read(torrent_path).unwrap(),
        ));
        
        self
    }

    pub fn torrent_data(&mut self, filename: String, data: Vec<u8>) -> &mut Self {
        let torrents = &mut self.params.torrents;
        torrents.push((
            filename,
            data,
        ));
        
        self
    }

    pub fn tags(&mut self, tags: Vec<String>) -> &mut Self {
        self.params.tags = Some(tags);
        self
    }

    pub fn tag(&mut self, tag: String) -> &mut Self {
        self.params.tags.as_mut().unwrap_or(&mut vec![]).push(tag);
        self
    }

    pub fn category(&mut self, category: String) -> &mut Self {
        self.params.category = Some(category);
        self
    }

    /// Set the upload to be paused by default.
    pub fn paused(&mut self) -> &mut Self {
        self.params.paused = Some(true);
        self
    }

    pub fn build(&self) -> TorrentUpload {
        self.params.clone()
    }
}

#[derive(Debug)]
pub struct TorrentTracker {
    /// Tracker URL
    pub url: String,

    /// Tracker status
    pub status: TrackerStatus,

    /// Tracker message (there is no way of knowing what this message is - it's up to tracker admins)
    pub message: Option<String>,
}

/// An enum that represents the status of a tracker.
/// Some of these statuses may not be supported by other trackers.
#[derive(Debug, PartialEq)]
pub enum TrackerStatus {
    /// Tracker is disabled (used for DHT, PeX, and LSD)
    Disabled = 0,

    /// Tracker has not been contacted yet
    NotContacted = 1,
    
    /// Tracker has been contacted and is working
    Working = 2,
    
    /// Tracker is updating
    Updating = 3,
    
    /// Tracker has been contacted, but it is not working (or doesn't send proper replies)
    NotWorking = 4
}