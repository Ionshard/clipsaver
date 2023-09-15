# Clipsaver

Clipsaver is a simple utility that allows you to save an image currently stored in your OS' clipboard and save it as a file.

## Why?

I like to use Cmd+Shift+4 on MacOS to screenshot a portion of my window, most of the time I immediately paste this image into a chat program or website. However sometimes I want the screenshot to be longer lived or I need it as a file due to a restriction at the source. Instead of needing to go into the options of the clip tool and switch back to storing the screenshot as a file (and then remember to switch it back afterwards) this tool was created to allow me to quickly save the image in the clipboard to disk.

Additionally it was a good way to get started in Rust

## Install

Use [cargo-bininstall](https://github.com/cargo-bins/cargo-binstall/tree/main#cargo-binaryinstall)

```bash
cargo-binstall clipsaver
```

## Usage

```bash
clipsaver -d ~/Pictures/Screenshots
```