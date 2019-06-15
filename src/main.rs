use std::io;
use std::cmp::Ordering;
use rand::{self, Rng};  // currently only support wasm32-wasi target in nightly
use log::{debug, trace};


// use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name="guess_wasi")]
struct Opt {
    #[structopt(long="levels")]
    levels: Vec<u32>,
}

fn main() {
    env_logger::init();
    let opt = Opt::from_args();
    for &lv in &opt.levels {
        println!("given number range 0~{}", lv);
        guess_a_number((0, lv));
    }
}

// While a game has many levels, we guess a number in each level.
fn guess_a_number((lb, hb): (u32, u32)) {
    let mut guess_str = String::new();
    let secret = rand::thread_rng().gen_range(lb, hb + 1);
    trace!("secret number: {}", secret);

    loop {
        println!("Please input your guess.");

        io::stdin().read_line(&mut guess_str)
            .expect("Failed to read line");
        
        debug!("scaned string: {:?}", guess_str);

        let guess = guess_str.trim().parse::<u32>()
            .expect("Input not a number! please input a number");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("You get it!");
                break;
            }
        }

        guess_str.clear();
        debug!("cleared string: {:?}", guess_str);
    }
}
