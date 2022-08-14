# SwayLid

A rust program that tells sway over ipc to enable or disable my laptop screen as needed.


## Usage

install with `cargo install --git https://github.com/Avemark/swaylid`

See example systemd units in [examples](examples/units/)

## Future

The monitor status code is overly complex because I am looking at also managing monitor layouts
between workplaces. I'll probably extract that to another crate.

I'll also look at making a threaded and more tightly integrated systemd daemon that controls it's
own lifecycle and sets appropriate statuses with the systemd notification bus.