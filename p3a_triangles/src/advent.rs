/// advent of code 
///
/// problem 3 part 1
///
/// validate triangles

pub static PROBLEM_NUMBER: &'static str = "3a"; 

fn is_valid(a: u32, b: u32, c: u32) -> bool {
    let mut vector = [a, b, c];
    vector.sort();

    (vector[0] + vector[1] > vector[2])
}


pub fn adv_main(input: Vec<String>) {
    let mut total = 0;
    
    for triangle in input {
        let parts: Vec<_> = triangle.trim().split_whitespace().collect();
        
        println!("{:?}", parts);
        let a: u32 = parts[0].parse().unwrap();
        let b: u32 = parts[1].parse().unwrap();
        let c: u32 = parts[2].parse().unwrap();
        

        if is_valid(a, b, c) {
            total += 1;
        }
    }

    println!("total: {}", total);
}
