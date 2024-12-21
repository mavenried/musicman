<div align=center>
<h1>Music Manager</h1>
  <p>Simple cli for playing music in the <code>~/Music</code> folder</p>
</div>

## What is musicman
Recursively scans the music directory for files (all files, so please only have music in there :P) and allows for playback of found songs. 
Has a queue implemented. Queues can be saved as playlists for quick access.
## Installation
The recommended way is to build from source, or using cargo.
```sh
cargo install musicman
```

## Commands
Running musicman presents you with a prompt. 

```
$ musicman
musicman❯
```

## The available commands
### add
Adds matches to you queue. 

`Syntax: add <search_term>`

>will prompt you to choose if multiple matches are found!

### show | ls
Show the current queue.

`Syntax: show`<br>
`Syntax: ls`
### current | cp
Shows the currently playing track<br>
`Syntax: show cp` (-_-)<br>
`Syntax: ls cp`

### p | play | pause
Pause/resume playback

`Syntax: p`<br>
`Syntax: play`<br>
`Syntax: pause`

### clear
Clears queue

`Syntax: clear`

### next 
Skip to the next song in queue. Optionally takes the number of tracks to skip

`Syntax: next`<br>
`Syntax: next <n>`

### prev
Skip to the previous song in queue. Optionally takes the number of tracks to skip

`Syntax: prev`<br>
`Syntax: prev <n>`

### replay
Replay the current song.

`Syntax: replay`

### playlist | pl
Playlist creation and playing
#### new
Saves the current queue as a playlist
>Playlist names are case sensitive (except on MacOS)

<br>

`Syntax: playlist new <name`
#### show | ls
Shows all the available playlists<br>
`Syntax: playlist show`<br>
`Syntax: playlist ls`
#### load
Load the specified playlist as the queue <br>
`Syntax: playlist new <name`
>Replaces the queue!

### exit
Quit the player

`Syntax: exit`

-----








