# Rust-Keyboard-Synthesizer
(c) Copyright 2019 Bar Movshovich <br/>
    Email: movshov@pdx.edu <br/>
(c) Copyright 2019 Larry Chiem <br/>
    Email: clarry@pdx.edu <br/>
(c) Copyright 2019 Andrew Wyatt <br/>
    Email: wyat2@pdx.edu <br/>
    
Class: CS410 Rust Programming/ CS410 Computer, Sound, and Music

# Purpose 
This code is a Rust polyphonic synthesizer that grabs midi inputs and plays the corresponding sound through the local audio (laptop/Desktop speakers). Note that currently 1-3 notes simultaneously have no issues. 4 note simultaneously produces a small amount of static and 5 notes simultaneously produces unrecognizable static noise. This code is heavily based off of Bart Masseys Python synthesizer. The envelope and apply_envelope functions in this program are directly based off of his example. 

# Running
To run this program a couple of prerequisites are needed. Most essentially you need to have a midi keyboard connected before running the program. You must also have Rust installed on your computer. 

To install Rust run these commands: <br/>
```curl https://sh.rustup.rs -sSf | sh ```<br/>
```rustup component add rustfmt```<br/>
```rustup component add clippy``` <br/>

To run the program clone the repositry to your local machine `git clone https://github.com/movshov/rust-keyboard-synth.git`. Change directory to keyboard `cd keyboard` then type `cargo run`. Rust's cargo manager will download all dependencies needed for the program to run. Once the program begins it will display a prompt saying `Available input ports`. If you have a midi keyboard plugged in it should now appear as a number followed by the keyboards name. 
For exmaple: ``` 0: USB Midi Interface```. 
Go ahead and choose that keyboard by typing in its corresponding number. If you have mulitple midi keyboards connected they will all show up here. Just choose whichever one you wish to use. If everything has gone accordingly you should now see this prompt: <br/> 
```opening connections```
```Connection open, forwarding from 'your keyboards name here' (press enter to exit) ... ```
Now you can press any key on your midi keyboard and you should now see that input displayed on the screen as "Note_On[...]" and when you let go of a key you should see "Note_Off[...]". Since this synthesizer is polyphonic you can play multiple keys at once. Once you are ready to exit simply hit "enter" or "control + c". 
# Resources Used:
Sound Stream - https://github.com/RustAudio/sound_stream/blob/master/examples/sine.rs <br/>
Bart's Example - https://github.com/pdx-cs-sound/fm/blob/master/fm.py <br/>
Midi message breakdown - https://github.com/wmedrano/wmidi/blob/master/src/lib.rs <br/>
Midi message reader - https://github.com/Boddlnagg/midir/blob/master/examples/test_forward.rs <br/>
Standard MIDI NOTE to Frequency - https://newt.phys.unsw.edu.au/jw/notes.html <br/>
