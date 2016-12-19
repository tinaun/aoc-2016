/// advent of code 
///
/// problem 3 part 2
///
/// validate more different triangles

pub static PROBLEM_NUMBER: &'static str = "3b"; 

fn is_valid(a: u32, b: u32, c: u32) -> bool {
    let mut vector = [a, b, c];
    vector.sort();

    (vector[0] + vector[1] > vector[2])
}


pub fn adv_main(input: Vec<String>) {
    let mut total = 0;
    let len = input.len();

    let mut sides: Vec<u32> = vec![0; len * 3];

    for (i, triangle) in input.iter().enumerate() {
        let parts: Vec<_> = triangle.trim().split_whitespace().collect();
        sides[i] = parts[0].parse().unwrap();
        sides[len + i] = parts[1].parse().unwrap();
        sides[len*2 + i] = parts[2].parse().unwrap();
    }

    for triangle in sides.chunks(3) {
        if is_valid(triangle[0], triangle[1], triangle[2]) {
            total += 1;
        }
    }

    println!("total: {}", total);
}
