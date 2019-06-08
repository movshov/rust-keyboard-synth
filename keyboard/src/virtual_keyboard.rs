extern crate piston_window;

use piston_window::*;

fn virtualKeyboard() {


    let keysize = 40.0;
    let keyh = 200.0;
    let bk_keyh = 100.0;
    let bk_keysize = 20.0;
    let red = [1.0, 0.0, 0.0, 1.0];
    let white = [1.0, 1.0, 1.0, 1.0];
    let black = [0.0, 0.0, 0.0, 1.0];
    let mut color = white;
    
    let one = 1.0;
    
    let mut window: PistonWindow =
        WindowSettings::new("keyboard", [702; 2])
            .build().unwrap();
            
    
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            clear([0.5, 0.5, 0.5, 1.0], g);
            
            // WHITE KEYS
            // 1
            let mut x = keysize;
            rectangle(color, // white
                      [0.0, 0.0, x, keyh], // rectangle
                      c.transform, g);
                      
            // 2
            x = x + one;
            rectangle(color, // red
                      [x, 0.0, keysize, keyh], // rectangle
                      c.transform, g);
                      
            // 3
            x = x + one + keysize;
            rectangle(color, // color
                      [x, 0.0, keysize, keyh], // rectangle
                      c.transform, g);
            
            // 4
            x = x + one + keysize;
            rectangle(color, // red
                      [x, 0.0, keysize, keyh], // rectangle
                      c.transform, g);
            
            // 5
            x = x + one + keysize;
            rectangle(color, // red
                      [x, 0.0, keysize, keyh], // rectangle
                      c.transform, g);
            
            // 6
            x = x + one + keysize;
            rectangle(color, // red
                      [x, 0.0, keysize, keyh], // rectangle
                      c.transform, g);
            
            // 7
            x = x + one + keysize;
            rectangle(color, // red
                      [x, 0.0, keysize, keyh], // rectangle
                      c.transform, g);
            
            // 8
            x = x + one + keysize;
            rectangle(color, // red
                      [x, 0.0, keysize, keyh], // rectangle
                      c.transform, g);
            
            // 9
            x = x + one + keysize;
            rectangle(color, // red
                      [x, 0.0, keysize, keyh], // rectangle
                      c.transform, g);
            
            // 10
            x = x + one + keysize;
            rectangle(color, // red
                      [x, 0.0, keysize, keyh], // rectangle
                      c.transform, g);
            
            // 11
            x = x + one + keysize;
            rectangle(color, // red
                      [x, 0.0, keysize, keyh], // rectangle
                      c.transform, g);
            
            // 12
            x = x + one + keysize;
            rectangle(color, // red
                      [x, 0.0, keysize, keyh], // rectangle
                      c.transform, g);
            
            // 13
            x = x + one + keysize;
            rectangle(color, // red
                      [x, 0.0, keysize, keyh], // rectangle
                      c.transform, g);
            
            // 14
            x = x + one + keysize;
            rectangle(color, // red
                      [x, 0.0, keysize, keyh], // rectangle
                      c.transform, g);
            
            // 15
            x = x + one + keysize;
            rectangle(color, // red
                      [x, 0.0, keysize, keyh], // rectangle
                      c.transform, g);
                      
            // BLACK KEY
            let space = 30.0;
            //color = black;
            // 1
            x = 30.0;
            rectangle(black, // red
                      [x, 0.0, bk_keysize, bk_keyh], // rectangle
                      c.transform, g);
                      
            // 2
            x = x + x + 10.0;
            rectangle(black, // red
                      [x, 0.0, bk_keysize, bk_keyh], // rectangle
                      c.transform, g);
                      
            // 3
            x = x + x + 10.0;
            rectangle(black, // red
                      [x, 0.0, bk_keysize, bk_keyh], // rectangle
                      c.transform, g);
                      
            // 4
            x = x + 30.0 + 10.0;
            rectangle(black, // red
                      [x, 0.0, bk_keysize, bk_keyh], // rectangle
                      c.transform, g);
                      
            // 5
            x = x + 30.0 + 10.0;
            rectangle(black, // red
                      [x, 0.0, bk_keysize, bk_keyh], // rectangle
                      c.transform, g);
            
            // 6
            x = x + space + space + space;
            rectangle(black, // red
                      [x, 0.0, bk_keysize, bk_keyh], // rectangle
                      c.transform, g);
                      
            // 7
            x = x + space + 10.0;
            rectangle(black, // red
                      [x, 0.0, bk_keysize, bk_keyh], // rectangle
                      c.transform, g);
                      
            // 8
            x = x + space + space + 20.0;
            rectangle(black, // red
                      [x, 0.0, bk_keysize, bk_keyh], // rectangle
                      c.transform, g);
                      
            // 9
            x = x + space + 10.0;
            rectangle(black, // red
                      [x, 0.0, bk_keysize, bk_keyh], // rectangle
                      c.transform, g);
                      
            // 10
            x = x + space + 10.0;
            rectangle(black, // red
                      [x, 0.0, bk_keysize, bk_keyh], // rectangle
                      c.transform, g);
        });
    }
}