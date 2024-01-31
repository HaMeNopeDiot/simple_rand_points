extern crate rand;

use rand::Rng;
use std::io;

fn main() {
    let mut rng = rand::thread_rng();
    let mut op = 6;
    
    println!("Write lvl: ");

    let mut lvl = String::new();

    io::stdin().read_line(&mut lvl).expect("Incorrect input!");

    let i_lvl: u32 = match lvl.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    op += i_lvl;

    let mut science_points = vec![3, 3, 3];
    for _i in 0..op {
        let choise = rng.gen_range(0..3);
        science_points[choise] += 1;
    }

    

    let nat_s = science_points[0];
    let tec_s = science_points[1];
    let soc_s = science_points[2];

    println!("Natural science: {nat_s}");
    println!("Tech science: {tec_s}");
    println!("Social science: {soc_s}");
}
