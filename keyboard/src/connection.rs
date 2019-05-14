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
use midir::{MidiInput, MidiOutput, Ignore};
use wmidi::MidiMessage::{self, *};
use pitch_calc::{Step,Hz,Letter,LetterOctave,ScaledPerc};
//use dimensioned::si::Hertz;
//use dimensioned::si;
//use dimensioned::dimensions::Frequency;
use synth::Synth;


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
    //const NOTE_ON_MSG: u8 = 0x90;
    const NOTE_OFF_MSG: u8 = 0x80;
        //_conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    let _conn_in = midi_in.connect(in_port, "midir-forward", move |_stamp, message, _| {
        conn_out.send(message).unwrap_or_else(|_| println!("Error when forwarding message ... "));
        match MidiMessage::from_bytes(message){
            Ok(NoteOn(_, note, velocity)) => {
                if velocity != 0{   //the key is only being pressed down. 
                    println!("Stamp {:?}, NoteOn {:?}",_stamp, message);
                    generate_sound(Step(note as f32).hz(), velocity as f32);    //note by default is U8 "8bit unsigned integer".          
                    //let _ = conn_out.send(&[NOTE_ON_MSG, note, velocity]);  //send NOTE_ON_MSG, play note at ceratin velocity.

                }
                else{
                    let _ = conn_out.send(&[NOTE_OFF_MSG, note, 0]);  //send NOTE_OFF_MSG, play note at ceratin velocity.
                }
            },
            //Ok(NoteOff(_, _, _)) => println!("NoteOff {:?}", message), //will never happen with my midi cable.
            _ => {}}}, ())?;

    println!("Connection open, forwarding from '{}' to '{}' (press enter to exit) ...", _in_port_name, _out_port_name);
    input.clear();
    stdin().read_line(&mut input)?; // wait for next enter key press
    println!("Closing connection");
    Ok(())
}

/*Purpose: Generate a sound having been given the frequency and the velocity.  
* note should now be the frequency that we want to play. 
* velocity is how hard the user pressed the piano key assuming that it has a 
* way of recording velocity.
* If the velocity is 0 it means the user has let go of the key. 
*/
fn generate_sound(_note:f32, _velocity:f32){
    let mut synth = {
        use synth::{Point, Oscillator, oscillator, Envelope};

        // The following envelopes should create a downward pitching sine wave that gradually quietens.
        // Try messing around with the points and adding some of your own!
        let amp_env = Envelope::from(vec!(
                //         Time ,  Amp ,  Curve
                Point::new(0.0  ,  0.0 ,  0.0),
                Point::new(0.01 ,  1.0 ,  0.0), Point::new(0.45 ,  1.0 ,  0.0),
                Point::new(0.81 ,  0.8 ,  0.0),
                Point::new(1.0  ,  0.0 ,  0.0),
                ));
        let freq_env = Envelope::from(vec!(
                //         Time    , Freq   , Curve
                Point::new(0.0     , 0.0    , 0.0),
                Point::new(0.00136 , 1.0    , 0.0),
                Point::new(0.015   , 0.02   , 0.0),
                Point::new(0.045   , 0.005  , 0.0),
                Point::new(0.1     , 0.0022 , 0.0),
                Point::new(0.35    , 0.0011 , 0.0),
                Point::new(1.0     , 0.0    , 0.0),
                ));

        // Now we can create our oscillator from our envelopes.
        // There are also Sine, Noise, NoiseWalk, SawExp and Square waveforms.
        let oscillator = Oscillator::new(oscillator::waveform::Sine, amp_env, freq_env, ());

        // Here we construct our Synth from our oscillator.
        Synth::retrigger(())
            .oscillator(oscillator) // Add as many different oscillators as desired.
            .duration(6000.0) // Milliseconds.
            //.base_pitch(LetterOctave(Letter::C, 1).hz()) // Hz.
            .base_pitch(_note) // Hz.
            .loop_points(0.49, 0.51) // Loop start and end points.
            .fade(500.0, 500.0) // Attack and Release in milliseconds.
            .num_voices(16) // By default Synth is monophonic but this gives it `n` voice polyphony.
            .volume(1.0)
            .detune(0.5)
            .spread(1.0)

            // Other methods include:
            // .loop_start(0.0)
            // .loop_end(1.0)
            // .attack(ms)
            // .release(ms)
            // .note_freq_generator(nfg)
            // .oscillators([oscA, oscB, oscC])
            // .volume(1.0)
    };

    synth.note_on(_note, _velocity);
    println!("playing note {}\n", _note);

}




