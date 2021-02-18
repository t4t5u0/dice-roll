// use rand::prelude::thread_rng;
// use rand::prelude::Rng;
// use std::env;

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     let (n, _, m) = arg_parse(&args[1]);
//     let x = roll(n, m);
//     let x_sum: u128 = x.iter().sum();
//     println!("{}d{} is : {:?} > {}", n, m, x, x_sum);
// }

// fn roll(n: usize, m: u128) -> Vec<u128> {
//     let mut rng = thread_rng();
//     let distr = rand::distributions::Uniform::new(1, m + 1);
//     let mut nums = vec![0u128; n];
//     for x in &mut nums {
//         *x = rng.sample(distr);
//     }
//     return nums;
// }

// fn arg_parse(arg: &String) -> (usize, String, u128) {
//     let result: Vec<&str> = arg.split("d").collect();
//     let n: usize = result[0].parse().unwrap();
//     let m: u128 = result[1].parse().unwrap();
//     return (n, "d".to_string(), m);
// }

// // fn index(arg: String) -> i32 {

// // }

// // fn roll(n: i32, m: i32) -> i32 {

// // }

// pub mod calc;

// fn main() {
//     let a = calc::ast::ConstantVal::new(33);
//     println!("ConstantVal={}", a.eval());
// }

pub mod calc;

use crate::calc::expr_eval;

fn main() {
    loop {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).ok();
        match expr_eval(&s) {
            Ok(val) => println!("ok:{}", val),
            _ => println!("構文エラー"),
        }
    }
}
