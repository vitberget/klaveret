# Klaveret

A Rust project for learning embedded Rust for myself, and writing a keyboard firmware.

## Goals

- [ ] A normal functioning keyboard
- [ ] Switch between templates/layouts (qwerty/dvorak/other) or such

## Sources I learned from

* <https://reltech.substack.com/p/getting-started-with-rust-on-a-raspberry>
* <https://github.com/rp-rs/rp2040-project-template>
* <https://blog.scottlogic.com/2022/12/08/building-a-rusty-vim-clutch.html>
* <https://doc.rust-lang.org/book/>
* <http://blog.timhutt.co.uk/std-embedded-rust/index.html>
* <https://github.com/rp-rs/rp-hal/blob/main/rp2040-hal/examples/uart.rs>

## Serial monitoring

### screen

```sh
screen /dev/ttyACM0 9600
```

exit: `Ctrl-A \`

### cat

```shell
cat /dev/ttyACM0
```
