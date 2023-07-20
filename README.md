# Speedy Git aka Spit
Sit is a command line tool built to wrap git commands in to quicker and more concise commands with safer defaults. Below is the list of currentyl supported spit commands and their git equivalents.

## Commands
| `sit` | `git` |
| ------ | ----- |
| `sit commit <msg>` | `git add . `<br /> `git commit --message="msg"` |
| `sit push` | `git push --set-upstream origin <curr_branch>` |
| `sit update <msg>` | `git add .` <br /> `git commit --message="msg"`<br /> `git push --set-upstream origin <curr_branch>` |
| `sit make <url>` | `git init` <br /> `git remote add origin <url>` |


