/// advent of code 
///
/// problem 7 
///
/// test weird future ip addrs for ssl and tls support
///

pub static PROBLEM_NUMBER: &'static str = "7"; 

fn is_abba(slice: &str) -> bool {
    let mut in_hypernet = false;
    let mut valid = false;
    let slice: Vec<_> = slice.chars().collect();

    for window in slice.windows(4) {
        if window[0] == '[' || window[0] == ']' {
            in_hypernet = !in_hypernet;
            continue;
        }

        if window[0] != window[1] && window[1] == window[2] && window[0] == window[3] {
            if in_hypernet {
                return false;
            } else {
                valid = true;
            }
        }

    }

    valid
}

fn is_aba(slice: &str) -> bool {
    let (mut abas, mut babs) = (Vec::new(), Vec::new()); 
    let mut in_hypernet = false;
    let slice: Vec<_> = slice.chars().collect();


    for window in slice.windows(3) {
        if window[0] == '[' || window[0] == ']' {
            in_hypernet = !in_hypernet;
            continue;
        }

        if window[0] == window[2] && window[0] != window[1] {
            if in_hypernet {
                babs.push( (window[1], window[0], window[1]) );
            } else {
                abas.push( (window[0], window[1], window[0]) );
            }
        }
    }

    for aba in &abas {
        for bab in &babs {
            if aba == bab {
                return true;
            }
        }
    }

    false
}

pub fn adv_main(input: Vec<String>) {

    let (tls, ssl) = input.iter().map(|s| (is_abba(s) as u64, is_aba(s) as u64))
                                 .fold((0,0), |(t, s), (tls, ssl)| {
        (t + tls, s + ssl)
    });
                            

    println!("tls support: {}\nssl support: {}", tls, ssl);

}

