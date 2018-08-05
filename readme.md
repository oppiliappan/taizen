![banner.png](https://0x0.st/sVMH.png)

Browse mediawiki pages from the command line.

## Installation
This project uses [Cursive crate](https://github.com/gyscos/Cursive), so before installing `Taizen`
make sure you have installed necessary Cursive [dependencies](https://github.com/gyscos/Cursive/wiki/Install-ncurses).

```shell
git clone https://github.com/nerdypepper/taizen
cd taizen
cargo run --release
```

## Usage

Taizen uses a **stack** like model.
Articles are opened on new layers, pop a layer to go back.
Hit `s` to search  
Hit `q` to quit  
Hit `t` to pop a layer from the article stack  

You can now view wikipedia pages in different languages, by passing the
language code as a commandline arg.
[List of language codes](https://en.wikipedia.org/wiki/List_of_Wikipedias#Detailed_list)

```
taizen --lang=sv
```


## Screenshot

![scrot.png](https://0x0.st/sVXt.png)
