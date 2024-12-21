use colored::Colorize;
use core::time;
use dirs::home_dir;
use std::io::Write;
use std::io::{stdin, stdout};
use std::process::exit;
use std::sync::{mpsc, Arc};
use std::thread::sleep;
mod enums;
mod handlers;
use enums::*;

pub fn init() {
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();
    let sink = Arc::new(sink);
    let mut queue: Vec<String> = Vec::new();
    let all_songs = handlers::index_all(
        home_dir()
            .unwrap()
            .join("Music")
            .to_str()
            .unwrap()
            .to_string(),
    );
    let mut current_index = 0;
    let (tx, rx) = std::sync::mpsc::channel();
    let ui_tx = tx.clone();
    std::thread::spawn(move || user_input(ui_tx));
    let (player_thread, prx) = std::sync::mpsc::channel();
    let player_sink = sink.clone();
    std::thread::spawn(move || player(tx, prx, player_sink));
    let mut interrupted = false;
    loop {
        if let Ok(recieved) = rx.recv() {
            match recieved {
                Command::Add(blob) => {
                    let songs = handlers::search(all_songs.clone(), &blob.to_lowercase());
                    if songs.len() != 1 {
                        if songs.is_empty() {
                            println!("{} {}", "No match found for".yellow(), blob.yellow().bold());
                        } else {
                            println!("{}", "Found multiple matches, which do I add? :".yellow());
                            let mut c = 0;
                            for i in &songs {
                                let i = i.split('/').last().unwrap();
                                c += 1;
                                println!("  {c}) {}", i.yellow().bold());
                            }
                        }
                    } else {
                        println!(
                            "{} {}",
                            "Added to queue:".green(),
                            songs[0].green().italic()
                        );
                        queue.push(songs[0].clone());
                        if queue.len() == 1 {
                            let song = queue[current_index].clone();
                            player_thread.send(PlayerCommand::Play(song)).unwrap();
                        }
                    }
                }
                Command::Replay => {
                    if !queue.is_empty() {
                        sink.clear();
                        let song = queue[current_index].clone();
                        let file = std::fs::File::open(&song).unwrap();
                        sink.append(rodio::Decoder::new(std::io::BufReader::new(file)).unwrap());
                        sink.play();
                        println!("{}", "Replaying...".yellow().italic());
                    } else {
                        println!("{}", "Queue empty.".yellow().italic());
                    }
                }
                Command::Toggle => {
                    if sink.is_paused() {
                        sink.play();
                        println!("{}", "Playing...".yellow().italic());
                    } else {
                        sink.pause();
                        println!("{}", "Paused.".yellow().italic());
                    }
                }
                Command::Clear => {
                    print!("\x1B[H\x1B[2J\x1B[3J");
                }
                Command::Next(n) => {
                    if !queue.is_empty() {
                        current_index = (current_index + n) % queue.len();
                        println!("{}", "Playing Next...".yellow().italic());
                        interrupted = true;
                        sink.clear();
                        let song = queue[current_index].clone();
                        player_thread.send(PlayerCommand::Play(song)).unwrap();
                    } else {
                        println!("{}", "Nothing in queue".yellow().italic());
                    }
                }
                Command::Prev(n) => {
                    if !queue.is_empty() {
                        if (current_index as i32 - n as i32) < 0 {
                            current_index =
                                queue.len() - (-(current_index as i32 - n as i32) as usize);
                        } else {
                            current_index -= n;
                        }
                        println!("{}", "Playing Next...".yellow().italic());
                        interrupted = true;
                        sink.clear();
                        let song = queue[current_index].clone();
                        player_thread.send(PlayerCommand::Play(song)).unwrap();
                    } else {
                        println!("{}", "Nothing in queue".yellow().italic());
                    }
                }
                Command::Exit => {
                    println!("{}", "Exiting...".yellow().italic());
                    exit(0);
                }
                Command::Show(cmd) => {
                    if !queue.is_empty() {
                        match cmd {
                            ShowCommand::Current => handlers::pretty_print(
                                &vec![queue[current_index]
                                    .clone()
                                    .split('/')
                                    .last()
                                    .unwrap()
                                    .to_string()],
                                "Current",
                                Some(0),
                            ),
                            ShowCommand::All => handlers::pretty_print(
                                &queue
                                    .iter()
                                    .map(|s| s.split('/').last().unwrap().to_string())
                                    .collect(),
                                "Queue",
                                Some(current_index),
                            ),
                        }
                    } else {
                        println!("{}", "Nothing in queue".yellow().italic());
                    }
                }
                Command::Playlist(cmd) => match cmd {
                    PlaylistCommand::New(name) => {
                        handlers::make_playlist(queue.clone(), name);
                    }
                    PlaylistCommand::List => {
                        handlers::show_playlists();
                    }
                    PlaylistCommand::Load(name) => {
                        println!(
                            "{} {}",
                            "Playing from playlist".green(),
                            name.green().bold()
                        );
                        let new_queue = handlers::load_playlist(name.clone() + ".list");
                        if !new_queue.is_empty() {
                            queue = new_queue.clone();
                            handlers::pretty_print(
                                &queue
                                    .iter()
                                    .map(|s| s.split('/').last().unwrap().trim().to_string())
                                    .collect(),
                                name.as_str(),
                                None,
                            );
                            interrupted = true;
                            sink.clear();
                            let song = queue[0].clone();
                            player_thread.send(PlayerCommand::Play(song)).unwrap();
                        }
                    }
                },
                Command::TrackEnd => {
                    if interrupted {
                        interrupted = false;
                    } else if !queue.is_empty() {
                        current_index = (current_index + 1) % queue.len();
                        let song = queue[current_index].clone();
                        player_thread.send(PlayerCommand::Play(song)).unwrap();
                    }
                }
            }
        }
    }
}

fn user_input(tx: mpsc::Sender<Command>) {
    loop {
        sleep(time::Duration::from_millis(10));
        print!("{}", "musicman❯ ".green().bold());
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        let input = input
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        match input[0].as_str() {
            "add" => {
                if input.len() > 1 {
                    tx.send(Command::Add(input[1..].join(" ").to_string()))
                        .unwrap();
                } else {
                    println!("{}", "add: Insufficient arguments".red());
                    println!("{}", "add <song name>".yellow().italic());
                }
            }
            "replay" => {
                tx.send(Command::Replay).unwrap();
            }
            "play" | "pause" | "p" => {
                tx.send(Command::Toggle).unwrap();
            }
            "clear" => {
                tx.send(Command::Clear).unwrap();
            }
            "next" | "prev" => {
                if input.len() > 1 {
                    if let Ok(n) = input[1].parse::<usize>() {
                        if input[0] == "next" {
                            tx.send(Command::Next(n)).unwrap();
                        } else {
                            tx.send(Command::Prev(n)).unwrap();
                        }
                    } else if input[0] == "next" {
                        println!("{}", "Usage: next <+ve number>".yellow().italic());
                    } else {
                        println!("{}", "Usage: prev <+ve number>".yellow().italic());
                    }
                } else if input[0] == "next" {
                    tx.send(Command::Next(1)).unwrap();
                } else {
                    tx.send(Command::Prev(1)).unwrap();
                }
            }
            "exit" => {
                tx.send(Command::Exit).unwrap();
            }
            "show" | "ls" => {
                if input.len() > 1 && input[1] == "cp" {
                    tx.send(Command::Show(ShowCommand::Current)).unwrap();
                } else {
                    tx.send(Command::Show(ShowCommand::All)).unwrap();
                }
            }
            "playlist" | "pl" => {
                if input.len() > 1 {
                    match input[1].as_str() {
                        "load" | "new" => {
                            let new = input[2..].join(" ").to_lowercase();
                            if input[1] == "load" {
                                tx.send(Command::Playlist(PlaylistCommand::Load(new)))
                                    .unwrap();
                            } else {
                                tx.send(Command::Playlist(PlaylistCommand::New(new)))
                                    .unwrap();
                            }
                        }
                        "ls" | "show" => {}
                        cmd => {
                            println!(
                                "{} {} {}",
                                "playlist: ".red(),
                                cmd.red().bold(),
                                " is not a valid command".red()
                            );
                            println!("{}", "Usage: playlist <add|new|show>".yellow());
                        }
                    }
                }
            }
            cmd => {
                println!("{} {}", "Unknown command".red(), cmd.red().bold());
                println!(
                    "{}",
                    "<add|clear|exit|next|p|playlist|prev|replay|show>"
                        .yellow()
                        .italic()
                );
            }
        }
    }
}

fn player(tx: mpsc::Sender<Command>, prx: mpsc::Receiver<PlayerCommand>, sink: Arc<rodio::Sink>) {
    loop {
        if let Ok(PlayerCommand::Play(song)) = prx.recv() {
            let file = std::fs::File::open(&song).unwrap();
            sink.append(rodio::Decoder::new(std::io::BufReader::new(file)).unwrap());
            sink.play();
            sink.sleep_until_end();
            tx.send(Command::TrackEnd).unwrap();
        }
    }
}
