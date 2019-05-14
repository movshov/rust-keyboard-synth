/* Purpose: Set up MidiInput connection to the keyboard and begin 
 * reading input that the user issues (piano notes pressed). Sadly
 * there will be some latency in the midi cable of about 6ms. Nothing
 * can be done to fix this so it may sound a little slow to play the 
 * notes. 
 */

mod sound;

use std::io::{stdin, stdout, Write};
use std::error::Error;
//use std::thread;
use midir::{MidiInput, Ignore};
use wmidi::MidiMessage::{self, *};
//use pitch_calc::Step;
//use dimensioned::si;
//use synth::Synth;
pub fn run() -> Result<(), Box<Error>> {
    let mut input = String::new();
    
    let mut midi_in = MidiInput::new("midir forwarding input")?;
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
    let in_port_name = midi_in.port_name(in_port)?;

    //_conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    let _conn_in = midi_in.connect(in_port, "midir-forward", move |_stamp, message, _| {
        match MidiMessage::from_bytes(message){
            Ok(NoteOn(_, _, _)) => {
                println!("Stamp {:?}, NoteOn {:?}",_stamp, message);
                sound::generate_sound()
            },
            //Ok(NoteOff(_, _, _)) => println!("NoteOff {:?}", message), //will never happen with my midi cable.
            _ => {}}}, ())?;

    println!("Connection open, forwarding from '{}' to (press enter to exit) ...", in_port_name);
    input.clear();
    stdin().read_line(&mut input)?; // wait for next enter key press
    println!("Closing connection");
    Ok(())
}

