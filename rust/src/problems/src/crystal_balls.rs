// Given two crystal balls that will break if dropped from high enough distance, determine the exact spot in which it will break in the most optimized way.

fn main() {
    let breaking_points = [false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, true, true, true];
    
    // We jump by sqrt(N)
    let jump_point = (breaking_points.len() as f64).sqrt().abs() as usize;
    let mut jumps: usize = jump_point - 1;

    'traversal: loop {
        if jumps >= breaking_points.len(){
            break;
        }
        if !breaking_points[jumps] {
            jumps += jump_point;
        } else {
            // When we find where the first ball breaks, we use the second ball to find the most
            // optimal breaking point
            let prev_jump = jumps - jump_point;
            for i in prev_jump..jumps {
                if breaking_points[i] {
                    println!("{i}");
                    break 'traversal;
                }
            }
        }
    }
}
