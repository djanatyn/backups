# backups

backup automation 🦀

built with:
* [borg](https://www.borgbackup.org/) for backups,
* [backblaze-b2](https://github.com/Backblaze/B2_Command_Line_Tool) for cloud storage,
* [systemd-creds](https://systemd.io/CREDENTIALS/) for service secrets,
* [systemd timer units](https://github.com/djanatyn/flake/blob/cb9a0cc3784389054b47b4d510a5f30030c2c793/desktop/default.nix#L214) for scheduled runs

# why write a rust program?

* better error handling,
* easier runtime reporting,
* easier to extend / understand,
* it's fun!

# workflow

```sh
# generate password
pass insert backups/shell-history/password

# create repo
BORG_PASSCOMMAND="pass show backups/shell-history/password" \
    borg init -e repokey /archive/shell-history

# export key
borg key export /archive/shell-history \
    | pass insert -m backups/shell-history/borg-key

# run first backup
BORG_PASSCOMMAND="pass show backups/shell-history/password" \
    borg create -v --stats "/archive/shell-history::initial backup" ~/.zsh_history
```
