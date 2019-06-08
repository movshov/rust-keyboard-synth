# Rust-Keyboard-Synthesizer Project Review
(c) Copyright 2019 Bar Movshovich <br/>
    Email: movshov@pdx.edu <br/>
(c) Copyright 2019 Larry Chiem <br/>
    Email: clarry@pdx.edu <br/>
(c) Copyright 2019 Andrew Wyatt <br/>
    Email: wyat2@pdx.edu <br/>
    
Class: CS410 Rust Programming/ CS410 Computer, Sound, and Music

# What was built:
We built a Rust Piano Synthesizer with midi input from a midi keyboard. Our original goal
was for the synthesizer to sound like a grand piano. However after a couple of weeks of
research, we discovered that the task of synthesizing a realistic grand piano sound is the
modern holy grail of piano synthesizers dating as far back as 1954.
# How it worked:
To accomplish this synthesizer, we created a midi buffer reader to grab the input from the
midi keyboard and push a hashset of notes currently playing to a callback stream that will
continuously provide sound.
# What doesn't work:
We were not able to generate a grand piano sounding synthesizer. As mentioned above, a
realistic grand piano synthesizer was more complex than what we had originally intended
and with the given time-frame would not have been possible. So instead we went with a
simple sine with synthesizer. We also tried to incorporate a visual keyboard (successfully)
to highlight (unsuccessfully) to the user what keys were currently being pressed.
# What lessons were learned:
We were all very inexperienced with Rust to begin with. However, throughout the term, we all
got better equated with the syntax and structure of Rust. We learned how sound was
produced and how to alter a sine wave using an envelope. We also got introduced to using
Piston for creating visuals of a keyboard. We also learned about the overall syntax on how
synthesizers work. We got more experienced at using git when working with multiple
branches.
