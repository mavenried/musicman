use crate::types::*;
use colored::Colorize;
use std::{
    io::{stdin, stdout, Write},
    sync::{mpsc, Arc},
    thread::sleep,
    time,
};

pub fn user_input(tx: mpsc::Sender<Command>) {
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
        if input.is_empty() {
            continue;
        }
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
                        "ls" | "show" => tx.send(Command::Playlist(PlaylistCommand::List)).unwrap(),
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
                if cmd.parse::<usize>().is_ok() {
                    tx.send(Command::Number(cmd.to_string())).unwrap();
                } else {
                    tx.send(Command::Error(cmd.to_string())).unwrap();
                }
            }
        }
    }
}

pub fn player(
    tx: mpsc::Sender<Command>,
    prx: mpsc::Receiver<PlayerCommand>,
    sink: Arc<rodio::Sink>,
) {
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
