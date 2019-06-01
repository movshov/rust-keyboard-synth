# rust-keyboard-synth
(c) Copyright 2019 Bar Movshovich 2019 <br/>
    Email: movshov@pdx.edu <br/>
(c) Copyright 2019 Larry Chiem 2019 <br/>
    Email: clarry@pdx.edu <br/>
(c) Copyright 2019 Andrew Wyatt 2019 <br/>
    Email: wyat2@pdx.edu <br/>
    
Class: CS410 Rust Programming/ CS410 Computer, Sound, and Music

# Purpose 
This code is a Rust polyphonic synthesizer that grabs midi inputs and plays the corresponding sound through the local audio (laptop/Desktop speakers). Note that currently 1-3 notes simultaneously have no issues. 4 note simultaneously produces a small amount of static and 5 notes simultaneously produces unrecognizable static noise. 

# Running
To run this program a couple of prerequisites are needed. Most essentially you need to have a midi keyboard connected before running the program. You must also have Rust installed on your computer. 

To install Rust run these commands: <br/>
```curl https://sh.rustup.rs -sSf | sh ```<br/>
```rustup component add rustfmt```<br/>
```rustup component add clippy``` <br/>

To run the program clone the repositry to your local machine. Change directory to keyboard `cd keyboard` then type `cargo run`. Rust's cargo manager will download all dependencies needed for the program to run. Once the program begins it will display the following prompt: 


# Resources Used:
Sound Stream - https://github.com/RustAudio/sound_stream/blob/master/examples/sine.rs <br/>
Bart's Example - https://github.com/pdx-cs-sound/fm/blob/master/fm.py <br/>
Midi message breakdown - https://github.com/wmedrano/wmidi/blob/master/src/lib.rs <br/>
Midi message reader - https://github.com/Boddlnagg/midir/blob/master/examples/test_forward.rs <br/>
Standard MIDI NOTE to Frequency - https://newt.phys.unsw.edu.au/jw/notes.html <br/>
