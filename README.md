# knockson
simple multi-threaded port scanner written in rust

![Example](./docs/knockson.svg)

# Install
## Using AUR
https://aur.archlinux.org/packages/knockson-bin/

`yay -Syu knockson-bin`

## Manual
Builds can be found here https://github.com/joshmuente/knockson/releases/

# Usage
```USAGE:
    knockson [OPTIONS]

FLAGS:
        --help       Print help information
    -V, --version    Print version information

OPTIONS:
    -a, --amount-thread <AMOUNT_THREAD>    [default: 10]
    -f, --from-port <FROM_PORT>            [default: 1]
    -h, --host <HOST>                      [default: 127.0.0.1]
    -t, --to-port <TO_PORT>                [default: 65535]
```
    
## Example
`knockson -f 1 -t 200 -h scanme.nmap.org -a 100`
