pub struct MusicPlaylistItem<'a> {
    name: String,
    artist: String,
    next_track: &'a MusicPlaylistItem<'a>,
}

pub struct MusicPlaylistItem2 {
    name: String,
    artist: String,
    next_track: Box<MusicPlaylistItem2>,
}
