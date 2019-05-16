/* Purpose: Set up MidiInput connection to the keyboard and begin 
 * reading input that the user issues (piano notes pressed). Sadly
 * there will be some latency in the midi cable of about 6ms. Nothing
 * can be done to fix this so it may sound a little slow to play the 
 * notes. 
 */


use std::io::{stdin, stdout, Write};
use std::error::Error;
//use std::time::Duration;
//use std::thread::sleep;
//use std::{thread, time};
use midir::{MidiInput, MidiOutput, Ignore}; //MIDI reader/writer.
use wmidi::MidiMessage::{self, *};  //MIDI message converter. 
use pitch_calc::Step;
//use pitch_calc::{Step,Hz,Letter,LetterOctave,ScaledPerc};
//use dimensioned::si::Hertz;
//use dimensioned::si;
//use dimensioned::dimensions::Frequency;
use sound_stream::{CallbackFlags, CallbackResult, SoundStream, Settings, StreamParams};
use core::f32::consts::PI;



/*Purpose: Setup Midi connections to the keyboard and to one output port.  Most likely the output port will also be the keyboard. Once a NOTE_ON input is detected call the generate_sound() function that will take the broken down midi message and generate a sine wave of that note 
 * along with its desired volume (velocity). Lines 30:80 were based off
 * of https://github.com/Boddlnagg/midir/blob/master/examples/test_forward.rs. 
 */
pub fn run() -> Result<(), Box<Error>> {
    let mut input = String::new(); 
    let mut midi_in = MidiInput::new("midir input")?;
    midi_in.ignore(Ignore::None);
    let midi_out = MidiOutput::new("midir output")?;
    println!("Available input ports:"); 
    for i in 0..midi_in.port_count() {
        println!("{}: {}", i, midi_in.port_name(i)?);
    }
    print!("Please select input port: ");
    stdout().flush()?;
    stdin().read_line(&mut input)?;
    let in_port: usize = input.trim().parse()?; 

    println!("\nAvailable output ports:");
    for i in 0..midi_out.port_count(){
        println!("{}: {}", i, midi_out.port_name(i)?);
    }
    println!("Please select output port: ");
    stdout().flush()?;
    input.clear();
    stdin().read_line(&mut input)?;
    let out_port: usize = input.trim().parse()?;

    println!("\nOpening connections");
    let _in_port_name = midi_in.port_name(in_port)?;
    let _out_port_name = midi_out.port_name(out_port)?;

    let mut conn_out = midi_out.connect(out_port, "midi-forward")?;
    //const NOTE_ON_MSG: u8 = 0x90; //MIDI default NOTE_ON message.
    const NOTE_OFF_MSG: u8 = 0x80;  //MIDI default NOTE_OFF message.
        //_conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    let _conn_in = midi_in.connect(in_port, "midir-forward", move |_stamp, message, _| {
        conn_out.send(message).unwrap_or_else(|_| println!("Error when forwarding message ... "));
        match MidiMessage::from_bytes(message){
            Ok(NoteOn(_, note, velocity)) => {
                if velocity != 0{   //the key is only being pressed down. 
                    println!("Stamp {:?}, NoteOn {:?}",_stamp, message);
                    //generate_sound(Step(note as f32).hz(), velocity as f32);    //note by default is U8 "8bit unsigned integer".          
                    generate_sound(Step(f32::from(note)).hz(), f32::from(velocity));    //note by default is U8 "8bit unsigned integer".          
                    //let _ = conn_out.send(&[NOTE_ON_MSG, note, velocity]);  //send NOTE_ON_MSG, play note at ceratin velocity.
                }
                else{   //the  user has let go of the key.
                    let _ = conn_out.send(&[NOTE_OFF_MSG, note, 0]);  //send NOTE_OFF_MSG, play note at 0 velocity. aka turn off note. 
                }
            },
            _ => {}}}, ())?;

    println!("Connection open, forwarding from '{}' to '{}' (press enter to exit) ...", _in_port_name, _out_port_name);
    input.clear();
    stdin().read_line(&mut input)?; // wait for next enter key press
    println!("Closing connection");
    Ok(())
}

/*Purpose: Generate a sound having been given the frequency and the velocity.  
* note should now be the frequency that we want to play. 
* velocity is how hard the user pressed the piano key assuming that the keyboard has 
* way of recording velocity.
*/
fn generate_sound(_note:f32, _velocity:f32){
//Sample rate.
let rate:f32 = 48000.0;

//Keymap contains currently-held notes for keys.
//let keymap = dict()

//Note map contains currently-playing operators.
//let notemap = set();

//Conversion factor for Hz to radians.
let _hz_to_rads = 2 as f32 * PI / rate;

//Attack time in secs and samples for AR envelope.
let _t_attack = 0.010;
let _s_attack = rate * _t_attack;

//Release time in secs and samples for AR envelope.
let _t_release = 0.30;
let _s_release = rate * _t_release;


envelope();

}
/*Purpose: Alter the sine wave that gets passed in to better match the 
 * desired sound we want to hear. The goal for this assignment is to 
 * get the sine wave to sound like a grand piano if possible. 
 */
fn envelope(){


}


