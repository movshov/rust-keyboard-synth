/* Purpose: Set up MidiInput connection to the keyboard and begin 
 * reading input that the user issues (piano notes pressed). Sadly
 * there will be some latency in the midi cable of about 6ms. Nothing
 * can be done to fix this so it may sound a little slow to play the 
 * notes. 
 */
use std::io::{stdin, stdout, Write}; use std::error::Error; use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use midir::{MidiInput, Ignore}; //MIDI reader/writer.
use wmidi::MidiMessage::{self, *};  //MIDI message converter. 
use sound_stream::{CallbackFlags, CallbackResult, SoundStream, Settings, StreamParams};
use core::f64::consts::PI;
const RATE:f64 =48000.00; //sample rate.
const T_RELEASE:f64 = 0.10;
const S_RELEASE:f64 =  RATE * T_RELEASE;
const T_ATTACK:f64 =  0.010;
const S_ATTACK:f64 =  RATE * T_ATTACK;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Op {
    time: u64,
    key: u8,
    release_time: u64,
    release_length: u64,
    rads: u64,
    velocity: u8
}

impl Op {
    /*
    fn new(&self, _key: u8, _rads: u64, _velocity: u8) -> Op {
        Op{
            time: 0,
            key: _key,
            release_time: 0,
            release_length: 0,
            rads: _rads,
            velocity: _velocity
        }
    }
    */
    fn off(&self, velocity:u64) -> Op {
        Op{
            time: self.time,
            key: self.key,
            release_time:self.time,
            release_length: (S_RELEASE as u64) / velocity,
            rads: self.rads,
            velocity: 0,
        }
    }

    fn envelope(&self) -> f64 {
        let t = self.time as f64;
        if self.release_time != 0{
            let rt = t - self.release_time as f64;
            if rt >= S_RELEASE{
                return 0.00;
            }
            return 1.00 - (rt / S_RELEASE) as f64;
        }
        if t < S_ATTACK{
            (t/S_ATTACK) as f64
        }
        else{
            1.00
        }
    }

    fn calculate_amp(mut self) -> f64{
        let amp =  f64::sin(self.rads as f64 * self.time as f64);
        self.time += 1;
        amp
    }
}

fn apply_envelope(notes_playing:&mut HashSet<Op>) -> u64{
    let mut remove:Vec<Op> = Vec::new();    //new vector of notes that need to be removed.
    let mut result = 0 as f64;
    for note in notes_playing.iter(){   //loop through all notes_playing.
        let env = note.envelope();  //call envelope function for each note.
        if env ==  0.00{   //release is complete get rid of note. 
            remove.push(*note); //add note to remove vector.
        }
        else{
            result += env * note.calculate_amp(); //calculate sine wave and add to amp.
        }
    }

    for x in remove.iter_mut(){ //loop through remove vector and remove.
        notes_playing.remove(x);   //remove note from notes_playing.
    } 
    remove.drain(..);   //empty the remove vector. 

    (0.1 * result) as u64   //return the amp to be played of all sine waves combined. 
}
/* Purpose: Setup Midi connections to the keyboard and to one output port.  Most likely the output port will also be the keyboard. 
 * Once a NOTE_ON input is detected call the generate_sound() function that will take the broken down midi message and 
 * generate a sine wave of that note along with its desired volume (velocity). Lines 30:80 were based off
 * of https://github.com/Boddlnagg/midir/blob/master/examples/test_forward.rs.  
 */
pub fn run() -> Result<(), Box<Error>> {
    let _hz_to_rads = 2.00 * PI / RATE as f64; //Conversion factor for Hz to radians.
    let mut key_to_freq: Vec<f64> = Vec::new(); // Conversion table for keys to radian frequencies.
    for key in 0..=128 {
        key_to_freq.push(note_to_frequency(_hz_to_rads, key as f64));
    }

    //let mut notes_playing:HashSet<Op> = HashSet::new(); //hashset of struct OP of notes currently playing. 
    //let _buffer = Arc::new(Mutex::new(HashSet::new()));   //will not have duplicates. 
    let _buffer = Arc::new(Mutex::new(HashSet::<Op>::new()));   //create a Mutex containing Hashset of notes currently playing. 

/*************************************CALLBACK_START ***********************************/
    let buffer_clone = Arc::clone(&_buffer);
    let callback = Box::new(move |output: &mut[f32], settings: Settings, _, _: CallbackFlags|{
        for frame in output.chunks_mut(settings.channels as usize) {
            //println!("inisde callback\n");
            let mut data = buffer_clone.lock().unwrap();    //lock the data so we can use it. 
            let amp:f32 = apply_envelope(&mut data) as f32; 
            for channel in frame {
                *channel = amp;
            }
            //Add break for inf loop. 
            //TODO need to create close function to change playing to false when user wants to leave. 
        }
        CallbackResult::Continue 
    });

    let _stream = SoundStream::new().output(StreamParams::new()).run_callback(callback).unwrap();
    //while let Ok(true) = stream.is_active() {}
/***********************************END_OF_CALLBACK**********************************/
    
    //hashset instead of vector. 
    //
    // call callback here.

    /************************ MIDI_START**************************************/
    let mut input = String::new(); 
    let mut midi_in = MidiInput::new("midir input")?;
    midi_in.ignore(Ignore::None);
    println!("Available input ports:"); 
    for i in 0..midi_in.port_count() {
        println!("{}: {}", i, midi_in.port_name(i)?);
    }
    print!("Please select input port: ");
    stdout().flush()?;
    stdin().read_line(&mut input)?;
    let in_port: usize = input.trim().parse()?; 

    println!("\nOpening connections");
    let _in_port_name = midi_in.port_name(in_port)?;

    //const NOTE_OFF_MSG: u8 = 0x80;  //MIDI default NOTE_OFF message.
    //SITS AND BLOCKS UNTIL A KEY IS PRESSED.
    let buffer_copy = Arc::clone(&_buffer);
    let _receive = midi_in.connect(in_port, "midir-forward", move |_stamp, message, _| {
        match MidiMessage::from_bytes(message){
            Ok(NoteOn(_, note, _velocity)) => {
                if _velocity != 0{   //the key is only being pressed down. 
                    println!("Stamp {:?}, NoteOn {:?}",_stamp, message);
                    let mut data = buffer_copy.lock().unwrap(); //get mut function.
                    data.insert(Op{time:0, key:note, release_time:0, release_length:0, rads:key_to_freq[note as usize] as u64, velocity:_velocity});
                } else{   //the  user has let go of the key NOTE_ON with 0 velocity.
                    println!("Stamp {:?}, NoteOff {:?}",_stamp, message);
                    let mut data = buffer_copy.lock().unwrap();    //lock clone of buffer.
                    data.remove(&Op {time:0, key:note, release_time:0, release_length:0, rads:0, velocity:0});  //remove this note. 
                }
            },
            Ok(NoteOff(_, note, _velocity)) => {
               println!("Stamp {:?}, NoteOff {:?}",_stamp, message);
               let mut data = buffer_copy.lock().unwrap();    //lock clone of buffer.
               data.remove(&Op {time:0, key:note, release_time:0, release_length:0, rads:0, velocity:0});  //remove this note. 

            },
            _ => {}}
    }, ())?;

    println!("Connection open, forwarding from '{}' (press enter to exit) ...", _in_port_name);
    input.clear();
    stdin().read_line(&mut input)?; // wait for next enter key press
    println!("Closing connection");
    Ok(())
}

/*********************************MIDI_END************************************/

fn note_to_frequency(_hz_to_rads:f64, _note:f64) -> f64{
    //println!("note is {}\n", _note);
    let base:f64 = 2.00;
    _hz_to_rads * 440.00 * base.powf((_note - 69.00)/12.00)
}

/*
/*Purpose: Generate a sine wave of a given frequency. 
 */
fn sine_wave(phase: f64) -> f32 {
    ((phase * PI * 2.0).sin() * 0.5) as f32
}
//=−1/4sin(3𝜋𝑥)+1/4sin(𝜋𝑥)+3/√2cos(𝜋𝑥)
fn piano_sine(phase: f64) -> f32 {
    return (-(0.25*(phase * PI * 3.0).sin())+(0.25*(phase * PI).sin())+(0.866*(phase * PI).cos())) as f32
}
*/

/*Purpose: Generate a sound having been given the frequency and the velocity.  
* note should now be the frequency that we want to play. 
* velocity is how hard the user pressed the piano key assuming that the keyboard has 
* way of recording velocity.
*/


/* use queue
 *  lock vector when adding and removing
 *
 *
 */
/* Plays a note from notes_playing for 2 seconds. Only works as monophonic in this exmaple. 
fn generate_sound(mut notes_playing:Vec<Op>){
let duration = 2.00 * 48000.00;
let mut time = 0.00;
let mut amp = 0.0;
let callback = Box::new(output: &mut[f32], settings: Settings, _, _: CallbackFlags){
    for frame in output.chunks_mut(settings.channels as usize) {
        //let amp = (f64::sin(_frequency1 * time) + f64::sin(_frequency2 * time)) as f32;   //plays one note at a time. 
        amp = apply_envelope(&notes_playing) as f32;
        //amp += notes_playing.envelope();
        for channel in frame {
            *channel = amp;
        }
    time += 1.00;
}
if time < duration{ 
    CallbackResult::Continue 
} else { 
    CallbackResult::Complete 
}
};

let stream = SoundStream::new().output(StreamParams::new()).run_callback(callback).unwrap();
while let Ok(true) = stream.is_active() {}

}
*/
/*  BARS TODO LIST
1. Create an array of existing keys with assocaited _hz_to_rads values. 
2. Create a play vector of notes to be played with note_ON signal. If note_off is present remove
from array. 
3. replace line 132 _frequency with lookup in vector of keys to be played. 
4. create fucntion to loop through array of keys that are on, calculate sin(_frequency * time) of
that note and add to amp. Finall return amp to be played in callback. 
*/
