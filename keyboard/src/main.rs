
/****Other files ************/
mod connection; //connection is name of file getting referenced. 

fn main() {
  if let Err(error) = connection::run() {
    eprintln!("Error: {}", error);
    std::process::exit(1);
  }
    
}


