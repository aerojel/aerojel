use std::time::Duration;

use smallvec::SmallVec;

use crate::id::AerojelId;

pub struct UserData {
    pub favorite: bool,
    pub downloaded: bool,
    pub pinned: bool,
}

#[non_exhaustive]
pub enum ItemSpecificFields {
    Artist {
        bio: Option<String>
    },
    Album {
        artist_ids: SmallVec<[AerojelId; 1]>,
    },
    Track {
        album_id: Option<AerojelId>,
        artist_ids: SmallVec<[AerojelId; 1]>,
    },
    AudioBook {
        author_ids: SmallVec<[AerojelId; 1]>,
    },
    Playlist {
        length: u32,
    },
}

pub enum ColorData {
    Solid(u32),
}

pub enum ImageSize {
    Small,
    Medium,
    Big,
    Full,
}

pub struct ImageDescriptor {
    pub size: ImageSize,
    pub id: AerojelId,
}

pub struct Item {
    pub id: AerojelId,
    pub name: String,
    pub image_descriptors: SmallVec<[ImageDescriptor; 1]>,
    pub color_data: Option<ColorData>,
    pub user_data: Option<UserData>,
    pub total_duration: Option<Duration>,
    pub specific_fields: ItemSpecificFields,
}

pub struct ItemRange {
    pub items: Vec<Item>,
    pub start: usize,
}

impl ItemRange {
    pub fn empty() -> Self {
        Self {
            items: Vec::new(),
            start: 0,
        }
    }
}

// pub mod request {
//     use crate::id::AerojelId;

//     pub struct Plain {
//         pub parent_id: Option<AerojelId>,
//     }

//     pub enum Personalized {
//         SimilarTo(AerojelId),
//     }

//     pub struct Timeline {
//         id: AerojelId,
//     }

//     pub struct Search {
//         pub query: String,
//     }

//     pub enum ItemsRequest {
//         Plain(Plain),
//         Personalized(Personalized),
//         Timeline(Timeline),
//         Search(Search),
//     }
// }

// pub struct PlainRequest {
//     query: String,
// }

// pub struct RecommendedRequest {
//     query: String,
// }

// pub struct SearchRequest {
//     query: String,
// }

// pub enum ItemsRequest {
//     Plain(PlainRequest),
//     Recommended(RecommendedRequest),
//     Search(SearchRequest),
// }