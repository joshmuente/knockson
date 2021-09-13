# knockson
simple multi-threaded port scanner written in rust

[![asciicast](https://asciinema.org/a/6Hif1DUyPORuNjgTe87b17hCh.svg)](https://asciinema.org/a/6Hif1DUyPORuNjgTe87b17hCh)

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
