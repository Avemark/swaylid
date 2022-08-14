# Example unit files

Place in `~/.config/systemd/user`

Enable timer with `systemctl --user enable --now swaylock.timer`

The service itself is started by the timer and does not need enabling.