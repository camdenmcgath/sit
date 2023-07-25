# Speedy Git aka Sit
Sit is a command line tool built to wrap git commands into quicker and more concise commands with safer defaults. 

## Installation
Currently, only installations from source at [crates.io](https://crates.io/crates/sit) are supported. Binary installations are currently in the works. This is currently a very basic implementation and a young app, so there is a lot of room for development! 

### From Source (Crates.io)
If Rust is installed, you can run `cargo install sit`. It will copy binary targets to `~/.cargo/bin/`, so if `spit` is having trouble running, ensure that your shell is checking that directory! 
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

## Contributing
Please submit pull requests for any of the following: 
- Codebase design/implementation changes. If there are design flaws, let's fix them!
- New features: check out the Features to Implement/Settle section for some ideas, or experiment with your own!
- Error reports/ bug fixes: Please report any issues while using spit, and/or tinker with solutions!
- Quality of life, documentation, or any other helpful fixes or suggestions are greatly appreciated!

## Features to Implement/Settle
These are some features and ideas that make up the vision of where sit could eventually go. Feel free to add to this list or begin working on one of these features!
- [ ] pretty printing
- [ ] unsafe mode (or not, just more customizable and overwriteable)
  - [ ] pipe (|) or similar operator for custom chaining of commands. Could call this unsafe mode
    - [ ] also make spit wrap almost all git commands for this mode, customizability 
  - [ ] configuration file or settings for user to configure own defaults
  - [ ] user-defined combinations? maybe under configuration settings as well?
- [ ] support for binary installations, package managers
  - [ ] Windows
  - [ ] MacOS
  - [ ] Linux
- [ ] More sit command combos !!
- [ ] More tests
- [ ] Logging
