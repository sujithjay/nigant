# Nigant
![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)
[![Crates.io](https://img.shields.io/crates/d/nigant.svg)](https://crates.io/crates/nigant)

A Thesaurus for the Terminal

## Etymology
`Nigant` comes the Sanskrit word `निघण्टु` (Nighaṇṭu), meaning a glossary or a partial lexicon.

## Usage

### with Cargo
You can install `Nigant` using `Cargo`.

`Nigant` relies on the [Oxford Developer API](https://developer.oxforddictionaries.com/). Generate your `API ID` & `API KEY`, and place it in a `nigant.ini` file as `NIGANT_APP_ID` & `NIGANT_APP_KEY`.

```
$ cat nigant.ini
NIGANT_APP_ID=<YOUR_APP_ID>
NIGANT_APP_KEY=<YOUR_APP_KEY>

$ cargo install nigant

$ nigant <word>
```

### by Building
You can build `Nigant` from source-code before use. Refer section on [Building](#Building) for build instructions. Generate your `API ID` & `API KEY`, and place it in a `nigant.ini` file as `NIGANT_APP_ID` & `NIGANT_APP_KEY`.

```
$ cat nigant.ini
NIGANT_APP_ID=<YOUR_APP_ID>
NIGANT_APP_KEY=<YOUR_APP_KEY>

$ ./target/release/nigant <word> 
```
### Example
```
$ nigant pastiche                                                                                                 
pastiche
Definitions:
	 - an artistic work in a style that imitates that of another work, artist, or period
	 - imitate the style of (an artist or work)

Synonyms:
imitation, parody, take-off, parody, take off, burlesque, pastiche, make fun of

Etymology:
	 - late 19th century: from French, from Italian pasticcio, based on late Latin pasta‘paste’
```

## Building
`Nigant` is written in Rust, so you'll need to grab a Rust installation in order to compile it. In general, `Nigant` tracks the latest stable release of the Rust compiler.

To build `Nigant`:
```
$ git clone https://github.com/twinair/nigant
$ cd nigant
$ cargo build --release	
```
