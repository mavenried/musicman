#[derive(Debug)]
pub enum Command {
    Add(String),
    Clear,
    Exit,
    Next(usize),
    Prev(usize),
    Toggle,
    Playlist(PlaylistCommand),
    Replay,
    Show(ShowCommand),
    TrackEnd,
    Number(String),
    Error(String),
}

#[derive(Debug)]
pub enum PlaylistCommand {
    New(String),
    Load(String),
    List,
}
#[derive(Debug)]
pub enum ShowCommand {
    Current,
    All,
}

#[derive(Debug)]
pub enum PlayerCommand {
    Play(String),
}
