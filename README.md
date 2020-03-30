# Nigant
A Thesaurus for the Terminal

## Etymology
`Nigant` comes the Sanskrit word `निघण्टु` (Nighaṇṭu), meaning a glossary or a partial lexicon.

## Usage
Since the current version of the application is a beta release, we do **not** distribute it via the usual channels. You will have to build it before use. Refer section on [Building](#Building) for build instructions.

```
$ ./target/release/nigant <word> 
```

```
$ ./target/release/nigant fudge                                                                                                 
fudge
Definitions:
	 - a soft candy made from sugar, butter, and milk or cream.
	 - an instance of faking or ambiguity
	 - a piece of late news inserted in a newspaper page.
	 - present or deal with (something) in a vague, noncommittal, or inadequate way, especially so as to conceal the truth or mislead
	 - nonsense! (expressing disbelief or annoyance)

Synonyms:
chewy sweet, toffee, compromise, cover-up, halfway house, evade, dodge, skirt, avoid, duck, shift ground about
```

## Building
nigant is written in Rust, so you'll need to grab a Rust installation in order to compile it. In general, nigant tracks the latest stable release of the Rust compiler.

To build nigant:
```
$ git clone https://github.com/twinair/nigant
$ cd nigant
$ cargo build --release
$ ./target/release/nigant --version
nigant 0.1.0
```
