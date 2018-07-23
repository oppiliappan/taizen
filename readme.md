# Taizen

![scrot.png](https://0x0.st/sVXt.png)

Browse Wikipedia pages from the command line.  

## Installation
This project uses [Cursive crate](https://github.com/gyscos/Cursive), so before installing `Taizen`
make sure you have installed necessary Cursive [dependencies](https://github.com/gyscos/Cursive/wiki/Install-ncurses).

```shell
git clone https://github.com/nerdypepper/taizen
cd taizen
cargo run
```

## Usage

Taizen uses a **stack** like model.
Articles are opened on new layers, pop a layer to go back.  
Hit `s` to search  
Hit `q` to quit  
Hit `t` to pop a layer from the article stack  
