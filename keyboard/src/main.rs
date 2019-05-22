
/****Other files ************/
mod connection; //connection is name of file getting referenced. 

fn main() {
    /*
  if let Err(error) = connection::run() {
    eprintln!("Error: {}", error);
    std::process::exit(1);
  }
  */
  
/*
 * To test code run generage_sound(range, velocity)
 * make range 21:108
 */
  connection::generate_sound(69, 80);
    
}

//hi/
//hi
