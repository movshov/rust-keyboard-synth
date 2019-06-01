/* Purpose: Set up MidiInput connection to the keyboard and begin 
 * reading input that the user issues (piano notes pressed). Sadly
 * there will be some latency in the midi cable of about 6ms. Nothing
 * can be done to fix this so it may sound a little slow to play the 
 * notes. 
 */
use std::io::{stdin, stdout, Write}; 
use std::error::Error; 
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use midir::{MidiInput, Ignore}; //MIDI reader/writer.
use wmidi::MidiMessage::{self, *};  //MIDI message converter. 
use sound_stream::{CallbackFlags, CallbackResult, SoundStream, Settings, StreamParams};
use core::f64::consts::PI;

//sample rate.
const RATE:f64 = 48000.00; 
const T_RELEASE:f64 = 0.10;
const S_RELEASE:f64 =  RATE * T_RELEASE;  //4,800
const T_ATTACK:f64 =  0.010;
const S_ATTACK:f64 =  RATE * T_ATTACK;  //480.

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct NoteInfo {
    time: u64,
    key: u8,
    release_time: i64,
    release_length: i64,
    velocity: u8
}

impl NoteInfo {
    fn new(_key: u8, _velocity: u8) -> NoteInfo {
        NoteInfo{
            time: 0,
            key: _key,
            release_time: -1,
            release_length: -1,
            velocity: _velocity,
        }
    }
    //fn off(&self) -> NoteInfo {

    fn off(&mut self){
        /*NoteInfo{
            time: self.time,
            key: self.key,
            release_time: self.time as i64,
            release_length: (S_RELEASE /(self.velocity as f64)) as i64,
            velocity: self.velocity,
        }
        */
        self.release_time = self.time as i64;
        self.release_length = (S_RELEASE/(self.velocity as f64)) as i64;
    }

    /*This envelope is based off of Bart Massey's Envelope in his fm.py file
     * lines 83 - 94. Credit for this function goes directly to him. This is the 
     * same function implemented in Rust. 
     */
    fn envelope(&self) -> f64{
        let time = self.time as f64;
        if self.release_time != -1{
            let rt = time - (self.release_time) as f64;
            if rt >= self.release_length as f64{
                return -1.00;
            }
            return 1.00 - (rt / self.release_length as f64) as f64
        }
        if time < S_ATTACK{
            return time/S_ATTACK as f64;
        }
        return 1.00;
    }

    /*Using the key_to_freq vector, that we calculated earlier which hold all radian values, grab
     * the radian value for "self.key". Next, calculate the volume we need to use with the 
     * velocity_conversion function. Finally, generate a sine wave to return by passing in the
     * radian value * time for the note and increment this specific notes time by 1. 
     */
    fn calculate_sin(&mut self, key_to_freq:Vec<f64>) -> f64{
        //return the radians of a given key. 
        let rads = key_to_freq[self.key as usize];  
        //multiply sine wave by velocity to get "volume"
        let amp = velocity_conversion(self.velocity) * (f64::sin(rads * self.time as f64));    
        self.time += 1;
        amp
    }
}

fn apply_envelope(notes_playing:&mut HashSet<NoteInfo>, key_to_freq:Vec<f64>) -> f64{
    //new vector of notes that need to be removed.
    let mut remove:Vec<NoteInfo> = Vec::new();    
    //Total sum of all sin waves combined and adjusted. 
    let mut result = 0 as f64;
    //loop through all notes currently playing. 
    for note in notes_playing.clone().iter(){   
        //call envelope function for each note.
        let env = note.envelope();  
        //if the note is done playing. 
        if env ==  -1.00{
            println!("Release is complete get rid of the note\n");
            //add note to remove vector to be removed later. 
            remove.push(*note); 
            continue
        }
        else{
            let key_clone = key_to_freq.clone();
            let mut note_copy = note.clone();
            //calculate sine wave and add to amp.
            result += env * note_copy.calculate_sin(key_clone); 
            remove.push(*note);
            notes_playing.insert(note_copy);
        }
    }
    //loop through remove vector and remove.
    for x in remove.iter_mut(){ 
        //remove note from notes_playing.
        notes_playing.remove(x);   
    }
    //empty the remove vector.
    remove.drain(..);
    //return the amp to be played of all sine waves combined.
    return 0.1 * result    
}
/* Purpose: This run function has two main parts. The first being a callback sound stream that will 
 * continously play sound through the speaker through a buffer of hashsets. The second is a midi
 * input stream that will grab any incoming midi messages and depending on whether a Note_On or
 * Note_Off signal was given, either add a note to the buffer or remove an already existing note
 * from the same buffer.  
 */
pub fn run() -> Result<(), Box<dyn Error>> {
    //Conversion factor for Hz to radians.
    let _hz_to_rads = 2.00 * PI / RATE as f64; 
    // Conversion table for keys to radian frequencies so we only have to calculate this once. 
    let mut key_to_freq: Vec<f64> = Vec::new(); 
    for key in 0..=128 {
        //call the note_to_frequency function which converts a midi key to its frequency. 
        //Then using our _hz_to_rads conversion factor convert the frequency to radians. 
        key_to_freq.push(note_to_frequency(_hz_to_rads, key as f64));
    }
    //create a Mutex containing Hashset of notes currently playing.
    let _buffer = Arc::new(Mutex::new(HashSet::<NoteInfo>::new()));    

    /*************************************CALLBACK_START ***********************************/
    let buffer_clone = Arc::clone(&_buffer);
    //Start the callback stream to continously play. 
    let callback = Box::new(move |output: &mut[f32], settings: Settings, _, _: CallbackFlags|{
        for frame in output.chunks_mut(settings.channels as usize) {
            let key_clone = key_to_freq.clone();
            //lock the data so we can use it.
            let mut data = buffer_clone.lock().unwrap(); 
            //apply our envelope to 
            let amp:f32 = apply_envelope(&mut data, key_clone) as f32; 

            for channel in frame {
                *channel = amp;
            }
        }
        CallbackResult::Continue 
    });

    let _stream = SoundStream::new().output(StreamParams::new()).run_callback(callback).unwrap();
    //while let Ok(true) = stream.is_active() {}
    /***********************************END_OF_CALLBACK**********************************/

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

    /*Wait for midi input. Once an input is detected match it with either "NoteON" or "NoteOff".
     * Some cables send "NoteOn" but with 0 velocity to indicate a note has been released. To
     * account for this we check if the velocity is not 0 in the "NoteOn" match. This is why there
     * is repeated code in this section for the "NoteOff" match. 
     */
    let buffer_copy = Arc::clone(&_buffer);
    let _receive = midi_in.connect(in_port, "midir-forward", move |_stamp, message, _| {
        match MidiMessage::from_bytes(message){
            Ok(NoteOn(_, note, _velocity)) => {
                if _velocity != 0{   //the key is only being pressed down. 
                    println!("NoteOn {:?}",message);
                    let mut data = buffer_copy.lock().unwrap(); //get mut function.
                    let new_note = NoteInfo::new(note, _velocity);
                    data.insert(new_note);
                } else{   //the  user has let go of the key NOTE_ON with 0 velocity.
                    println!("NoteOff {:?}",message);
                    //lock clone of buffer.
                    let mut data = buffer_copy.lock().unwrap();    
                    //loop through all notes_playing.
                    for notes in data.clone().iter(){   
                        if notes.key == note{    
                            let mut note_copy = notes.clone();
                            //turn off the note.
                            note_copy.off();     
                            //remove the original note.
                            data.remove(notes);   
                            //insert old note but as off.
                            data.insert(note_copy);  
                        }
                    }
                }
            },
            Ok(NoteOff(_, note, _velocity)) => {
                println!("NoteOff {:?}",message);
                //lock clone of buffer.
                let mut data = buffer_copy.lock().unwrap();    
                for notes in data.clone().iter(){   //loop through all notes_playing.
                    if notes.key == note{    //found the note. 
                        let mut note_copy = notes.clone();
                        //turn off the note.
                        note_copy.off();     
                        //remove the original note.
                        data.remove(notes);   
                        //add new note that's turning off.
                        data.insert(note_copy);  
                    }
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

//Convert a note to its corresponding frequency then to its radian value. 
fn note_to_frequency(_hz_to_rads:f64, _note:f64) -> f64{
    let base:f64 = 2.00;
    _hz_to_rads * 440.00 * base.powf((_note - 69.00)/12.00)
}

//Calculate volume based off of velocity.
fn velocity_conversion(velocity:u8)->f64{
    (velocity as f64 + 25.00)/150.00
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
