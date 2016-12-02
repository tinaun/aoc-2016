/// advent of code 
///
/// problem 2 part 2
///
/// given a list of relative directions on an odd diamond shaped keypad
/// find the final passcode

pub static PROBLEM_NUMBER: &'static str = "2b"; 


static KEYPAD: [[char; 5]; 5] = [ [' ', ' ', '1', ' ', ' '],
                                  [' ', '2', '3', '4', ' '],
                                  ['5', '6', '7', '8', '9'],
                                  [' ', 'A', 'B', 'C', ' '],
                                  [' ', ' ', 'D', ' ', ' ']];


type Keypad = (i32, i32);

fn decode_string(input: String, mut keypad: Keypad) -> (char, Keypad) {     
    
    for c in input.chars() {
        let mov = match c {
            'U' => { (0, -1) },
            'R' => { (1, 0) },
            'L' => { (-1, 0) },
            'D' => { (0, 1) },
            _ => { (0, 0) },
        };
    
        let nextmov = (keypad.0 + mov.0, keypad.1 + mov.1);

        if nextmov.0 < 5 && nextmov.1 < 5 && nextmov.0 > -1 && nextmov.1 > -1 
                && KEYPAD[nextmov.1 as usize][nextmov.0 as usize] != ' ' {

            keypad = nextmov;
        }

    }

    (KEYPAD[keypad.1 as usize][keypad.0 as usize],keypad)
}



pub fn adv_main(input: Vec<String>) {
    
    let mut pos = (0, 2);
    
    for code in input {
        let (digit, key_pos) = decode_string(code, pos);
        pos = key_pos;

        println!("{:?}", digit);
    }
}
