use std::thread::sleep;
use std::time::Duration;

use log::*;

use sdl2_timing::Sdl2Timing;

mod app_control;
mod stderrlog;
mod midi_container;
mod midi_sequencer;
mod scroller;
mod time_controller;
mod usage; // Hacked version of stderrlog crate

const SDL: &str = &"sdl";
/****Other files ************/
mod connection; //connection is name of file getting referenced. 

// fn main() {
fn main() -> Result<(), Box<std::error::Error>> {
  if let Err(error) = connection::run() {
    eprintln!("Error: {}", error);
    // std::process::exit(1);
  }
  let sdl_context = sdl2::init().unwrap();
  let video_subsystem = sdl_context.video().unwrap();
  info!(
      target: SDL,
      "display driver: {:?}",
      video_subsystem.current_video_driver()
  );
  let nr_displays = video_subsystem.num_video_displays()?;
/*
 * To test code run generage_sound(range, velocity)
 * make range 21:108
 */
//  connection::generate_sound(39, 80);
  std::process::exit(1);
  Ok(()) 
}

//hi/
//hi
