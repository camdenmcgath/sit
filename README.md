# Speedy Git aka Spit
Spit is a command line tool built to wrap git commands in to quicker and more concise commands with safer defaults. Below is the list of currentyl supported spit commands and their git equivalents.

## Commands
| `spit` | `git` |
| ------ | ----- |
| `spit commit <msg>` | `git add . `<br /> `git commit --message="msg"` |
| `spit push` | `git push --set-upstream origin <curr_branch>` |
| `spit update <msg>` | `git add .` <br /> `git commit --message="msg"`<br /> `git push --set-upstream origin <curr_branch>` |
| `spit make <url>` | `git init` <br /> `git remote add origin <url>` |


