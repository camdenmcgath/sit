# Speedy Git aka Sit
Sit is a command line tool built to wrap git commands into quicker and more concise commands with safer defaults. This is currently a very basic implementation and a young app, so there is a lot of room for development!

## Installation
Currently, only binary installations for Windows and a [crates.io](https://crates.io/crates/sit) installation for Rust devs are supported. Deploying to other binary targets is planned for the future, as well as registering with many package registries!

### Binaries
#### Windows
Download ```sit.exe``` from the [latest releases page](https://github.com/camdenmcgath/sit/releases) to a directory in your ```PATH```, like the ```~\Downloads``` directory. To double-check existing paths, run ```$env:Path -split ';'``` in powershell. To add to the ```PATH``` variable, run ```$addPath = 'C:\YourPath'```.
### Crates.io (Rust Devs)
[Installing Rust](https://www.rust-lang.org/tools/install) may be worth it for easy updating (```cargo update```) rather than manual binary installations. If Rust is installed, you can run `cargo install sit`. It will copy binary targets to `~/.cargo/bin/`, so if `sit` is having trouble running, ensure that your shell is checking that directory! 

## Commands
| `sit` | `git` |
| ------ | ----- |
| `sit commit <msg>` | `git add . `<br /> `git commit --message="msg"` |
| `sit push` | `git push --set-upstream origin <curr_branch>` |
| `sit update <msg>` | `git add .` <br /> `git commit --message="msg"`<br /> `git push --set-upstream origin <curr_branch>` |
| `sit make <url>` | `git init` <br /> `git remote add origin <url>` |

### Notes
- sit commands will fail if current directory is not a git repository
- commands listed with the `<curr_branch>` parameter use the current working branch
- `sit make` will only `git init` if repo is not already initialized
