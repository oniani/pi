mod lib;

use lib::pi;
use std::env;

// Default precision
const PRECISION: u32 = 100;

// Default number of iterations
const ITERATIONS: u32 = 100;

fn main() {
    // Command line arguments
    let args: Vec<_> = env::args().collect();

    // Make decisions based on the number of arguments
    match args.len() {
        1 => {
            print!("Approximation: {:?}\n", pi(PRECISION, ITERATIONS));
            print!("Precision:     {:?}\n", PRECISION);
            print!("Iterations:    {:?}\n", ITERATIONS);
        }

        2 => {
            let precision = args[1]
                .parse::<u32>()
                .expect("Precision must be a positive integer!");
            if precision <= 0 {
                panic!("Precision must be a positive integer!");
            } else {
                print!(
                    "Approximation: {:?}\n",
                    pi(precision as u32, ITERATIONS)
                );
                print!("Precision:  {:?}\n", precision);
                print!("Iterations: {:?}\n", ITERATIONS);
            }
        }

        3 => {
            let precision = args[1]
                .parse::<u32>()
                .expect("Precision must be a positive integer!");
            let iterations = args[2]
                .parse::<u32>()
                .expect("Number of iterations must be a positive integer!");
            if precision <= 0 {
                panic!("Precision must be a positive integer!");
            } else if iterations <= 0 {
                panic!("Number of iterations must be a positive integer!");
            } else {
                print!(
                    "Approximation: {:?}\n",
                    pi(precision as u32, iterations as u32)
                );
                print!("Precision:  {:?}\n", precision);
                print!("Iterations: {:?}\n", iterations);
            }
        }

        _ => panic!("Redundant argument."),
    }
}
