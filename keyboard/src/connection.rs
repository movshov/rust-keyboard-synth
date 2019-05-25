/* Purpose: Set up MidiInput connection to the keyboard and begin 
 * reading input that the user issues (piano notes pressed). Sadly
 * there will be some latency in the midi cable of about 6ms. Nothing
 * can be done to fix this so it may sound a little slow to play the 
 * notes. 
 */
use std::io::{stdin, stdout, Write};
use std::error::Error;
use std::collections::HashSet;
//use std::time::Duration; 
//use std::thread::sleep; 
use std::thread;
use std::sync::{Arc, Mutex};
use midir::{MidiInput, Ignore}; //MIDI reader/writer.
use wmidi::MidiMessage::{self, *};  //MIDI message converter. 
use sound_stream::{CallbackFlags, CallbackResult, SoundStream, Settings, StreamParams};
use core::f64::consts::PI;
const rate:u64 =48000; //sample rate.
const t_release:u64 = 0;
const s_release:u64 =  rate * t_release;
const t_attack:u64 =  0;
const s_attack:u64 =  rate * t_attack;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Op {
    time: u64,
    key: u8,
    release_time: u64,
    rads: u64
}

impl Op {
    fn new(&self, key: u8, rads: f64) -> Op {
        Op{
            time: 0,
            key: 0,
            release_time: 0,
            rads: 0
        }
    }

    fn off(&self) -> Op {
        Op{
            time: self.time,
            key: self.key,
            release_time:self.time,
            rads: self.rads,
        }
    }

    fn envelope(&self) -> f64 {
        let t = self.time;
        if self.release_time != 0{
            let rt = t - self.release_time;
            if rt >= s_release{
                return 0.00;
            }
            return 1.00 - (rt / s_release) as f64;
        }
        if t < s_attack{
            return (t/s_attack) as f64;
        }
        else{
            return 1.00; 
        }
    }

    fn calculate_amp(mut self) -> f64{
        let amp =  f64::sin(self.rads as f64 * self.time as f64);
        self.time += 1;
        return amp;
    }
}

/* Purpose: Setup Midi connections to the keyboard and to one output port.  Most likely the output port will also be the keyboard. 
 * Once a NOTE_ON input is detected call the generate_sound() function that will take the broken down midi message and 
 * generate a sine wave of that note along with its desired volume (velocity). Lines 30:80 were based off
 * of https://github.com/Boddlnagg/midir/blob/master/examples/test_forward.rs.  
 */
pub fn run() -> Result<(), Box<Error>> {
    let _hz_to_rads = 2.00 * PI / rate as f64; //Conversion factor for Hz to radians.
    let mut key_to_freq: Vec<f64> = Vec::new(); // Conversion table for keys to radian frequencies.
    for key in 0..=128 {
        key_to_freq.push(note_to_frequency(_hz_to_rads, key as f64));
    }

    //let mut notes_playing:HashSet<Op> = HashSet::new(); //hashset of struct OP of notes currently playing. 
    //let _buffer = Arc::new(Mutex::new(HashSet::new()));   //will not have duplicates. 
    let _buffer = Mutex::new(HashSet::new());   //will not have duplicates. 
    let playing:bool = true;

/*************************************CALLBACK_START ***********************************/
    let callback = Box::new(move |output: &mut[f32], settings: Settings, _, _: CallbackFlags|{
        for frame in output.chunks_mut(settings.channels as usize) {
            let mut data = _buffer.lock().unwrap();    //lock the data so we can use it. 
            //let amp:f32 = apply_envelope(&mut notes_playing) as f32; 
            let amp:f32 = apply_envelope(&mut data) as f32; 
            for channel in frame {
                *channel = amp;
            }
        }
        if playing == true{ 
            CallbackResult::Continue 
        } else { 
            CallbackResult::Complete 
        }
    });

    let stream = SoundStream::new().output(StreamParams::new()).run_callback(callback).unwrap();
    while let Ok(true) = stream.is_active() {}
/***********************************END_OF_CALLBACK**********************************/

    fn apply_envelope(notes_playing:&mut HashSet<Op>) -> u64{
        //*notes_playing = notes_playing.iter().cloned().filter(|Op| Op.envelope() != 0).collect()
        let mut remove:Vec<&Op> = Vec::new();    //new vector of notes that need to be removed.
        let mut result = 0 as f64;
        for note in notes_playing.iter(){   //loop through all notes_playing.
            let env = note.envelope();  //call envelope function for each note.
            if env ==  0.00{   //release is complete get rid of note. 
                remove.push(note.clone()); //add note to remove vector.
            }
            else{
                result += env * note.calculate_amp(); //calculate sine wave and add to amp.
            }
        }
        
        for x in remove.iter_mut(){ //loop through remove vector and remove.
            notes_playing.remove(x);   //remove note from notes_playing.
        } 
        remove.drain(..);   //empty the remove vector. 
        
        return (0.1 * result) as u64
    }
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
    //let mut new_note = Op {time:0, key:0, release_time:0, rads:0};
    let _receive = midi_in.connect(in_port, "midir-forward", move |_stamp, message, _| {
        match MidiMessage::from_bytes(message){
            Ok(NoteOn(_, note, velocity)) => {
                if velocity != 0{   //the key is only being pressed down. 
                    println!("Stamp {:?}, NoteOn {:?}",_stamp, message);
                    //notes_playing.insert(Op {time:0, key:note, release_time:0, rads:key_to_freq[note as usize] as u64});
                    {
                    let mut data = _buffer.lock().unwrap();
                    data.insert(Op {time:0, key:note, release_time:0, rads:key_to_freq[note as usize] as u64});
                    }
                }
                else{   //the  user has let go of the key NOTE_ON with 0 velocity.
                    println!("Stamp {:?}, NoteOff {:?}",_stamp, message);
                    //notes_playing.remove(&note);
                }
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
/*
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
/*
fn apply_envelope(mut notes_playing) -> u64{

    let mut result = 0 as f64;
    for note in notes_playing{
        let env = note.envelope();
        if env ==  0{
        //notes_playing.remove(note.key as usize);
        continue;
        }
        else{
            result += env * note.calculate_amp();
        }
    }
    return (0.1 * result) as u64
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
