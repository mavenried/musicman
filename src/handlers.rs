use colored::Colorize;
use dirs::config_dir;
use std::fs;
use std::io::Write;
use unicode_width::UnicodeWidthStr;

/// Pretty-print a list with a title and optional selection
pub fn pretty_print(data: &Vec<String>, title: &str, selected: Option<usize>) {
    let index = selected.unwrap_or_else(|| usize::MAX);

    // Add title to calculate max width
    let mut data_cpy = data.clone();
    data_cpy.push(title.to_string());

    let maxlen = data_cpy
        .iter()
        .map(|s| UnicodeWidthStr::width(s.as_str()))
        .max()
        .unwrap_or(0);

    println!("╭{title:─<len$}╮", len = maxlen + 6);
    for (i, s) in data.iter().enumerate() {
        let marker = if i == index { "▶ " } else { "  " };
        println!(
            "│ {} {:<width$}  │",
            marker.yellow(),
            s.blue(),
            width = maxlen
        );
    }
    println!("╰{}╯", "─".repeat(maxlen + 6));
}

/// Show all playlists
pub fn show_playlists() {
    let configdir = config_dir().unwrap().join("musicman/playlists/");
    if !configdir.exists() {
        fs::create_dir_all(&configdir).unwrap();
        println!("{}", "No playlists".yellow().italic());
    } else {
        let mut playlists = Vec::new();
        for entry in fs::read_dir(&configdir).unwrap() {
            let name = entry.unwrap().file_name();
            let name = name
                .to_string_lossy()
                .split('.')
                .next()
                .unwrap()
                .to_string();
            playlists.push(name);
        }
        if playlists.is_empty() {
            println!("{}", "No playlists".yellow().italic());
        } else {
            pretty_print(&playlists, "Playlists", None);
        }
    }
}

/// Load a playlist
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
            .lines()
            .map(|s| s.to_string())
            .collect()
    }
}

/// Save a playlist
pub fn make_playlist(queue: &Vec<String>, name: String) {
    let configdir = config_dir().unwrap().join("musicman/playlists/");
    if !configdir.exists() {
        fs::create_dir_all(&configdir).unwrap();
    }
    let out = queue.join("\n");
    let name = name + ".list";
    let mut playlist_file = fs::File::create(configdir.join(name)).unwrap();
    write!(playlist_file, "{out}").unwrap();
}

/// Recursively index all files
pub fn index_all(root: String) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for entry in fs::read_dir(&root).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_dir() {
            let new_root = format!("{}/{}", root, entry.file_name().into_string().unwrap());
            out.extend(index_all(new_root));
        } else {
            out.push(entry.path().to_string_lossy().to_string());
        }
    }
    out
}

/// Unicode-safe search
pub fn search(names_in: &Vec<String>, target: &String) -> Vec<String> {
    let mut names = names_in.clone();
    if names.contains(target) {
        return vec![target.clone()];
    }

    let mut found = false;
    let mut index = 1;

    while !found {
        let mut searchlist: Vec<String> = Vec::new();
        let target_prefix: String = target.chars().take(index).collect();

        for name in &names {
            let name_short: String = name.split('/').last().unwrap().to_lowercase();
            let name_prefix: String = name_short.chars().take(index).collect();

            if name_prefix == target_prefix {
                searchlist.push(name.clone());
            }
        }

        if searchlist.is_empty() {
            break;
        }

        if index >= target.chars().count() {
            if searchlist.len() == 1 {
                found = true;
            } else {
                return searchlist;
            }
        }

        names = searchlist;
        index += 1;
    }

    if found {
        names
    } else {
        vec![]
    }
}
