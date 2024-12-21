use colored::Colorize;
use dirs::config_dir;
use std::fs;
use std::io::Write;

pub fn pretty_print(data: &Vec<String>, title: &str, selected: Option<usize>) {
    let mut index = -1;
    if selected.is_some() {
        index = selected.unwrap() as i32;
    }
    let mut data_cpy = data.clone();
    data_cpy.push(title.to_string());

    let maxlen = data_cpy
        .iter()
        .fold(0, |t, v| if v.len() > t { v.len() } else { t });
    println!("╭{title:─<len$}╮", len = maxlen + 6);
    let mut c = -1;
    for s in data {
        c += 1;
        let mut playing = "  ";
        if c == index {
            playing = "▶ "
        }
        println!("│ {} {:<maxlen$}  │", playing.yellow(), s.blue());
    }
    println!("╰{}╯", "─".repeat(maxlen + 6));
}
pub fn show_playlists() {
    let configdir = config_dir().unwrap().join("musicman/playlists/");
    if !configdir.exists() {
        fs::create_dir_all(configdir).unwrap();
        println!("{}", "No playlists".yellow().italic());
    } else {
        let mut playlists = Vec::new();
        for song in fs::read_dir(configdir).unwrap() {
            let song = song
                .unwrap()
                .file_name()
                .to_str()
                .unwrap()
                .split('.')
                .next()
                .unwrap()
                .to_string();
            playlists.push(song);
        }
        if playlists.is_empty() {
            println!("{}", "No playlists".yellow().italic());
        } else {
            pretty_print(&playlists, "Playlists", None)
        }
    }
}
pub fn load_playlist(name: String) -> Vec<String> {
    let playlist_path = config_dir()
        .unwrap()
        .join("musicman/playlists/")
        .join(&name);
    if !playlist_path.exists() {
        println!(
            "{} {}",
            "playlist: load: No such playlist".red(),
            name.red().bold()
        );
        vec![]
    } else {
        fs::read_to_string(playlist_path)
            .unwrap()
            .trim()
            .split('\n')
            .map(|s| s.to_string())
            .collect()
    }
}
pub fn make_playlist(queue: Vec<String>, name: String) {
    let configdir = config_dir().unwrap().join("musicman/playlists/");
    if !configdir.exists() {
        fs::create_dir_all(&configdir).unwrap();
    }
    let out = queue.join("\n");
    let name = name + ".list";
    let mut playlist_file = fs::File::create(configdir.join(name)).unwrap();
    write!(playlist_file, "{out}").unwrap();
}
pub fn index_all(root: String) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for entry in fs::read_dir(&root).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_dir() {
            let new_root = root.clone() + "/" + (entry.file_name().into_string().unwrap().as_str());
            // println!("{new_root}");
            for elem in index_all(new_root) {
                out.push(elem);
            }
        } else {
            out.push(entry.path().to_str().unwrap().to_string());
        }
    }
    out
}

pub fn search(names_in: Vec<String>, target: &String) -> Vec<String> {
    //println!("Searching for {target:?}");
    let mut names = names_in.clone();
    if names.contains(target) {
        vec![target.clone()]
    } else {
        let mut found = false;
        let mut index = 1;
        while !found {
            let mut searchlist: Vec<String> = Vec::new();
            let searchstr = &target[..index];
            for name in &names {
                let name_short = name.split("/").last().unwrap().to_lowercase();
                // println!("{name_short}");
                if name_short.len() > index && &name_short[..index] == searchstr {
                    searchlist.push(name.clone());
                }
            }
            // println!("{:?}", &searchlist);

            if searchlist.is_empty() {
                break;
            }

            if index > target.len() - 1 {
                if searchlist.len() == 1 {
                    found = true;
                } else {
                    return searchlist;
                }
            }

            names = searchlist.clone();
            index += 1;
        }
        if found {
            names
        } else {
            vec![]
        }
    }
}
