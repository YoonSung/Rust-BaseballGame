use std::io;
use std::io::Write;
extern crate rand;
extern crate baseball_game;

use rand::distributions::{IndependentSample, Range};
use baseball_game::inning;

const INNING_INPUT_LIMIT: usize= 3;

fn main() {

    println!("========================");
    println!("       숫자 야구게임       ");
    println!("========================");

    let between = Range::new(1i8, 10);
    let answers:Vec<i8> = gen_rand_array(INNING_INPUT_LIMIT, between); 
    let inputs = gen_array_from_user(INNING_INPUT_LIMIT);

    //Check input data
    let sbo = inning::get_sbo_result(inputs, &answers);

    println!("correct answer : {}. {}. {}", answers[0], answers[1],answers[2]);
    println!("\nGame result\n-----------  \ns : {}\nb : {}\no : {}", sbo.strike, sbo.ball, sbo.out);

}

fn gen_rand_array(len: usize, range: Range<i8>) -> Vec<i8> {
    let mut rng = rand::thread_rng();
    let mut rand_array = Vec::new();

    while rand_array.len() != len {
        let rand_num = range.ind_sample(&mut rng);
        if rand_array.contains(&rand_num) {
            continue;
        } else { 
           rand_array.push(rand_num);
        }
    }

    rand_array
}

#[allow(unused_must_use)]
fn gen_array_from_user(len: usize) -> Vec<i8> {
    let mut return_arr:Vec<i8> = vec![];

    let mut current_input_size:usize= 0;
    while current_input_size != len-1 {
        current_input_size = return_arr.len();

        let mut input = String::new();
        print!("{}번째 숫자를 입력해주세요: ", current_input_size+1);

        io::stdout().flush();

        io::stdin().read_line(&mut input);
        let int_value: Option<i8> = input.trim().parse().ok();

        match int_value {
            value @ Some(1...9) => {
                //TODO already exist value check
                return_arr.push(value.unwrap());
            },
            Some(_)|None => {
               println!("only Input Between 1 to 10 number!");
                continue;
            }
        }
    }

    return_arr 
}
