/// advent of code 
///
/// problem 2 part 1
///
/// given a list of relative directions on a 3x3 keypad
/// find the passcode, ignoring impossible directions

pub static PROBLEM_NUMBER: &'static str = "2a"; 


type Keypad = (i32, i32);


fn decode_string(input: String, mut keypad: Keypad) -> (i32, Keypad) {     
    
    for c in input.chars() {
        match c {
            'U' => { keypad.1 += if keypad.1 == 1 { 0 } else { 1 }; },
            'R' => { keypad.0 += if keypad.0 == 1 { 0 } else { 1 }; },
            'L' => { keypad.0 -= if keypad.0 == -1 { 0 } else { 1 }; },
            'D' => { keypad.1 -= if keypad.1 == -1 { 0 } else { 1 }; },
            _ => {},
        }
    }

    ((1 - keypad.1) * 3 + keypad.0 + 2, keypad)
}



pub fn adv_main(input: Vec<String>) {
    
    let mut pos = (0,0);
    
    for code in input {
        let (digit, key_pos) = decode_string(code, pos);
        pos = key_pos;

        println!("{:?}", digit);
    }
}
