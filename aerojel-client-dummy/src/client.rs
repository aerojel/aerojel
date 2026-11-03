use aerojel_core::{
    client::{
        Client, ItemDelta, ItemsRequest, PlaylistDelta,
    },
    error::Error,
    id::AerojelId,
    image::Image,
    items::{
        Item, ItemRange,
    },
    lyrics::Lyrics,
    playback_log::{
        InititalEntry, LogEntry, PlaybackLog,
    },
};

pub struct DummyClient {
    _private: (),
}

impl DummyClient {
    pub fn new() -> Self {
        Self {
            _private: (),
        }
    }
}

impl Client for DummyClient {
    async fn request_item(&self, _id: &AerojelId) -> Result<Item, Error> {
        Err(Error::InvalidId)
    }

    async fn request_items(&self, _request: ItemsRequest) -> Result<ItemRange, Error> {
        Ok(ItemRange::empty())
    }

    async fn request_lyrics(&self, _id: &AerojelId) -> Result<Lyrics, Error> {
        Ok(Lyrics::Plain("".to_string()))
    }

    async fn request_image(&self, _id: &AerojelId) -> Result<Image, Error> {
        Ok(Image::empty_rgba())
    }

    async fn request_playback_log(
        &self,
        since: Option<&AerojelId>,
    ) -> Result<PlaybackLog, Error> {
        if let Some(_id) = since {
            std::future::pending().await
        } else {
            let mut log = PlaybackLog::new(AerojelId::InlineBytes { data: [0u8; 16], len: 0 });
            log.add_entry(LogEntry::Initial(InititalEntry::default()));

            Ok(log)
        }
    }

    async fn modify_item(&self, _id: &AerojelId, _delta: ItemDelta) -> Result<(), Error> {
        Ok(())
    }

    async fn modify_playlist(&self, _id: &AerojelId, _delta: PlaylistDelta) -> Result<(), Error> {
        Ok(())
    }

    async fn extend_playback_log(
        &self,
        _version_id: Option<&AerojelId>,
        _command: aerojel_core::client::PlaybackLogExtensionCommand,
    ) -> Result<(), Error> {
        Ok(())
    }
    
    async fn update_lyrics(&self, _id: &AerojelId, _new: Lyrics) -> Result<(), Error> {
        Ok(())
    }

    async fn update_image(&self, _id: &AerojelId, _new: Image) -> Result<(), Error> {
        Ok(())
    }

    async fn create_playlist(&self, _name: String) -> Result<AerojelId, Error> {
        Ok(AerojelId::InlineBytes { data: [0u8; 16], len: 0 })
    }
}