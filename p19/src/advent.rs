/// advent of code 2016 
///
/// problem 19
/// elves stealing cookies - josephus problem / optimization / finding patterns

pub static PROBLEM_NUMBER: &'static str = "18"; 

pub fn adv_main(input: Vec<String>) {
    let count: usize = input[0].parse().unwrap();

    let mut table = Vec::new();

    for i in 0..count {
        table.push(i+1);
    }

    while table.len() > 1 {
        let count = table.len();

        table = table.iter().enumerate().filter_map( |(c, &i)| {
            if c % 2 == 1 {
                None
            } else {
                Some(i)
            }
        }).collect();

        if count % 2 == 1 {
            table.remove(0);
        }
    }

    println!("last elf standing (next steal): {:?}", table[0]);

    // part 2
    let value = count as f64;
    let lowest_power = 3f64.powf( value.log(3f64).floor() );
    let res = 2.0 * lowest_power - value;

    let res = if res < 0.0 {
        lowest_power - (res * 2.0)
    } else if lowest_power == res {
        res
    } else {
        lowest_power - res
    };

    println!("last elf standing (opposite steal): {}", res);
}

