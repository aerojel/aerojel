use smallvec::SmallVec;

use crate::{error::Error, id::AerojelId, image::Image, items::{Item, ItemRange}, lyrics::Lyrics, playback_log::PlaybackLog};

pub enum ItemsRequest {
    Plain(( /* TODO */ )),
    Personalized(( /* TODO */ )),
    Search(( /* TODO */ )),
}

pub struct ItemDelta {
    pub new_name: Option<String>,
    // TODO
}

pub enum PlaylistModification {
    Insert {
        item_id: AerojelId,
        at: usize,
    },
    Append(AerojelId),
    Remove {
        item_id: AerojelId,
    },
    Move {
        item_id: AerojelId,
        to_index: usize,
    }
}

pub struct PlaylistDelta {
    pub modifications: SmallVec<[PlaylistModification; 1]>,
}

pub enum PlaybackLogExtensionCommand {
    Pause,
    Resume,
    SetNext(AerojelId),
    Insert {
        item_id: AerojelId,
        at_index: usize,
    },
    Remove(usize),
    Move {
        from_index: usize,
        to_index: usize,
    },
}

/// Core API of `aerojel`. All interactions with client should be made through this.
pub trait Client: Send + Sync {
    fn request_item(&self, id: &AerojelId) -> impl Future<Output = Result<Item, Error>> + Send;

    fn request_items(&self, request: ItemsRequest) -> impl Future<Output = Result<ItemRange, Error>> + Send;

    fn request_lyrics(&self, id: &AerojelId) -> impl Future<Output = Result<Lyrics, Error>> + Send;

    fn request_image(&self, id: &AerojelId) -> impl Future<Output = Result<Image, Error>> + Send;

    /// Request a log of playback events.
    /// 
    /// Functionality of this function depends heavily on the value of `since`. If `since` is:
    ///  - `None`, returns `PlaybackLog` that contains `Initial` entry which is the latest state.
    ///  - id of the latest entry, waits untill another entry is added and returns it.
    ///  - id of some old entry, returns a list of entries that were added since that point.
    fn request_playback_log(
        &self,
        since: Option<&AerojelId>,
    ) -> impl Future<Output = Result<PlaybackLog, Error>> + Send;
    
    fn modify_item(&self, id: &AerojelId, delta: ItemDelta) -> impl Future<Output = Result<(), Error>> + Send;

    fn modify_playlist(&self, id: &AerojelId, delta: PlaylistDelta) -> impl Future<Output = Result<(), Error>> + Send;
    
    /// None id modifies the latest state.
    fn extend_playback_log(
        &self,
        version_id: Option<&AerojelId>,
        command: PlaybackLogExtensionCommand,
    ) -> impl Future<Output = Result<(), Error>> + Send;
    
    fn update_lyrics(&self, id: &AerojelId, new: Lyrics) -> impl Future<Output = Result<(), Error>> + Send;

    fn update_image(&self, id: &AerojelId, new: Image) -> impl Future<Output = Result<(), Error>> + Send;

    fn create_playlist(&self, name: String) -> impl Future<Output = Result<AerojelId, Error>> + Send;

    // TODO: add settings api
}

/// Dyn-compatible wrapper used by BoxedClient.
#[async_trait::async_trait]
trait DynClient: Send + Sync {
    async fn request_item(&self, id: &AerojelId) -> Result<Item, Error>;

    async fn request_items(&self, request: ItemsRequest) -> Result<ItemRange, Error>;

    async fn request_lyrics(&self, id: &AerojelId) -> Result<Lyrics, Error>;

    async fn request_image(&self, id: &AerojelId) -> Result<Image, Error>;

    async fn request_playback_log(&self, since: Option<&AerojelId>) -> Result<PlaybackLog, Error>;
    
    async fn modify_item(&self, id: &AerojelId, delta: ItemDelta) -> Result<(), Error>;

    async fn modify_playlist(&self, id: &AerojelId, delta: PlaylistDelta) -> Result<(), Error>;

    async fn extend_playback_log(&self, version_id: Option<&AerojelId>, command: PlaybackLogExtensionCommand) -> Result<(), Error>;
    
    async fn update_lyrics(&self, id: &AerojelId, new: Lyrics) -> Result<(), Error>;

    async fn update_image(&self, id: &AerojelId, new: Image) -> Result<(), Error>;

    async fn create_playlist(&self, name: String) -> Result<AerojelId, Error>;
}

#[async_trait::async_trait]
impl<T: Client> DynClient for T {
    async fn request_item(&self, id: &AerojelId) -> Result<Item, Error> {
        <T as Client>::request_item(self, id).await
    }

    async fn request_items(&self, request: ItemsRequest) -> Result<ItemRange, Error> {
        <T as Client>::request_items(self, request).await
    }

    async fn request_lyrics(&self, id: &AerojelId) -> Result<Lyrics, Error> {
        <T as Client>::request_lyrics(self, id).await
    }

    async fn request_image(&self, id: &AerojelId) -> Result<Image, Error> {
        <T as Client>::request_image(self, id).await
    }

    async fn request_playback_log(&self, since: Option<&AerojelId>) -> Result<PlaybackLog, Error> {
        <T as Client>::request_playback_log(self, since).await
    }
    
    async fn modify_item(&self, id: &AerojelId, delta: ItemDelta) -> Result<(), Error> {
        <T as Client>::modify_item(self, id, delta).await
    }

    async fn modify_playlist(&self, id: &AerojelId, delta: PlaylistDelta) -> Result<(), Error> {
        <T as Client>::modify_playlist(self, id, delta).await
    }

    async fn extend_playback_log(
        &self,
        version_id: Option<&AerojelId>,
        command: PlaybackLogExtensionCommand,
    ) -> Result<(), Error> {
        <T as Client>::extend_playback_log(self, version_id, command).await
    }
    
    async fn update_lyrics(&self, id: &AerojelId, new: Lyrics) -> Result<(), Error> {
        <T as Client>::update_lyrics(self, id, new).await
    }

    async fn update_image(&self, id: &AerojelId, new: Image) -> Result<(), Error> {
        <T as Client>::update_image(self, id, new).await
    }

    async fn create_playlist(&self, name: String) -> Result<AerojelId, Error> {
        <T as Client>::create_playlist(self, name).await
    }
} 

pub struct BoxClient(Box<dyn DynClient>);

impl BoxClient {
    pub fn new<C: Client + 'static>(client: C) -> Self {
        Self(Box::new(client))
    }
}

impl Client for BoxClient {
    async fn request_item(&self, id: &AerojelId) -> Result<Item, Error> {
        self.0.request_item(id).await
    }

    async fn request_items(&self, request: ItemsRequest) -> Result<ItemRange, Error> {
        self.0.request_items(request).await
    }

    async fn request_lyrics(&self, id: &AerojelId) -> Result<Lyrics, Error> {
        self.0.request_lyrics(id).await
    }

    async fn request_image(&self, id: &AerojelId) -> Result<Image, Error> {
        self.0.request_image(id).await
    }

    async fn request_playback_log(
        &self,
        since: Option<&AerojelId>,
    ) -> Result<PlaybackLog, Error>
    {
        self.0.request_playback_log(since).await
    }

    async fn modify_item(&self, id: &AerojelId, delta: ItemDelta) -> Result<(), Error> {
        self.0.modify_item(id, delta).await
    }

    async fn modify_playlist(&self, id: &AerojelId, delta: PlaylistDelta) -> Result<(), Error> {
        self.0.modify_playlist(id, delta).await
    }

    async fn extend_playback_log(
        &self,
        version_id: Option<&AerojelId>,
        command: PlaybackLogExtensionCommand,
    ) -> Result<(), Error> {
        self.0.extend_playback_log(version_id, command).await
    }

    async fn update_lyrics(&self, id: &AerojelId, new: Lyrics) -> Result<(), Error> {
        self.0.update_lyrics(id, new).await
    }

    async fn update_image(&self, id: &AerojelId, new: Image) -> Result<(), Error> {
        self.0.update_image(id, new).await
    }

    async fn create_playlist(&self, name: String) -> Result<AerojelId, Error> {
        self.0.create_playlist(name).await
    }
}