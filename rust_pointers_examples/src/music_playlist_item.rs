#[derive(Debug)]
pub struct MusicPlaylistItem<'a> {
    pub name: String,
    pub artist: String,
    // next_track: &'a MusicPlaylistItem<'a>,
    pub next_track: Option<&'a MusicPlaylistItem<'a>>,
}

impl<'a> MusicPlaylistItem<'a> {
  pub fn next(&self) -> Option<&MusicPlaylistItem<'_>> {
      self.next_track
  }
}

#[derive(Debug)]
pub struct MusicPlaylistItem2 {
    name: String,
    artist: String,
    next_track: Box<MusicPlaylistItem2>,
}
