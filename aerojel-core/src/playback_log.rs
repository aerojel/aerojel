use std::time::Duration;

use smallvec::SmallVec;

use crate::{id::AerojelId, items::Item};

#[derive(Default, Clone, Copy, Debug)]
pub struct PlaybackPosition {
    item_index: usize,
    offset: Option<Duration>,
}

impl PlaybackPosition {
    pub fn new(item_index: usize, offset: Option<Duration>) -> Self {
        Self {
            item_index,
            offset
        }
    }

    pub fn index(&self) -> usize {
        self.item_index
    }

    pub fn offset(&self) -> Option<Duration> {
        self.offset
    }
}

#[derive(Default, Clone, Copy)]
pub enum ProgressStatus {
    #[default]
    Playing,
    Paused,
    Buffering,
}

#[derive(Default)]
pub struct InititalEntry {
    pub position: PlaybackPosition,
    pub buffered_percentage: f32,
    pub progress_status: ProgressStatus,
    pub timeline: Vec<Item>,
}

pub enum LogEntry {
    Initial(InititalEntry),
    Paused(PlaybackPosition),
    Resumed(PlaybackPosition),
    PositionChanged(PlaybackPosition),
    TimelineItemAdded {
        index: usize,
        item: Item,
    },
    TimelineItemRemoved {
        index: usize,
    },
    TimelineItemMoved {
        from: usize,
        to: usize,
    },
}

pub struct PlaybackLog {
    pub version_id: AerojelId,
    pub entries: SmallVec<[LogEntry; 1]>,
}

impl PlaybackLog {
    pub fn new(version_id: AerojelId) -> Self {
        Self {
            version_id,
            entries: SmallVec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: LogEntry) {
        self.entries.push(entry);
    }
}