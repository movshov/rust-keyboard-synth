# rust-keyboard-synth
(c) Bar Movshovich 2019
CS410 Rust Programming

This code is a Rust polyphonic synthesizer that grabs midi inputs and plays the corresponding sound through the local audio (laptop/Desktop speakers). Note that currently 1-3 notes simultaneously have no issues. 4 note simultaneously produces a small amount of static and 5 notes simultaneously produces unrecognizable static noise. 

# Running
To run this program a couple of prerequisites are needed. Most essentially you need to have a midi keyboard connected before running the program, it will not start otherwise. You must also have Rust installed on your computer. 

To install Rust run these commands: 
```curl https://sh.rustup.rs -sSf | sh ```
```rustup component rustfmt add```
```rustup component clippy add ``` 

To run the program clone the repositry to your local machine. Change directory to keyboard `cd keyboard` then type `cargo run`. Rust's cargo manager will download all dependencies needed for the program to run. Once the program begins it will display the following prompt: 


# Resources Used:
Sound Stream - https://github.com/RustAudio/sound_stream/blob/master/examples/sine.rs
Bart's Example - https://github.com/pdx-cs-sound/fm/blob/master/fm.py
Midi message breakdown - https://github.com/wmedrano/wmidi/blob/master/src/lib.rs 
Midi message reader - https://github.com/Boddlnagg/midir/blob/master/examples/test_forward.rs
Standard MIDI NOTE to Frequency - https://newt.phys.unsw.edu.au/jw/notes.html
