# huncomma_terminal

This is a terminal frontend for the `huncomma` library, which utilizes all of its detectors.

## How to use

### Install

Right now, this is not uploaded to crates.io, so you have to clone this repo and compile it yourself.

`git clone git@github.com:VaranTavers/huncomma_terminal.git`


`cargo build --release`

### Usage

This program reads the input from the standard input and writes the results into the standard output.

```
> cargo run
> Azt mondom hogy állj!
>
ln: 1, col: 13 potenciális vesszőhiba (80%): a 'hogy' szó elé általában vesszőt teszünk
```

You can pipe files into it:

Windows:

`type file.txt | cargo run`

Linux:

`cat file.txt | cargo run`


