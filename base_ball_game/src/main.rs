use std::io;
use std::io::Write;
extern crate rand;
use rand::distributions::{IndependentSample, Range};

const INNING_INPUT_LIMIT: usize= 3;

struct SBO {
    strike: i8,
    ball  : i8,
    out   : i8
}

fn main() {

    println!("========================");
    println!("       숫자 야구게임       ");
    println!("========================");

    let mut answers:Vec<i8> = vec![]; 

    let between = Range::new(1i8, 10);
    gen_rand_array(&mut answers, INNING_INPUT_LIMIT, between); 
    let inputs = gen_array_from_user(INNING_INPUT_LIMIT);

    //Check input data
    let sbo = get_sbo_result(inputs, &answers);

    println!("correct answer : {}. {}. {}", answers[0], answers[1],answers[2]);
    println!("\nGame result\n-----------  \ns : {}\nb : {}\no : {}", sbo.strike, sbo.ball, sbo.out);

}

fn gen_rand_array(arr: &mut Vec<i8>, len: usize, range: Range<i8>) {
    let mut rng = rand::thread_rng();

    while arr.len() != len {
        let rand_num = range.ind_sample(&mut rng);
        if (*arr).contains(&rand_num) {
            continue;
        } else { 
            (*arr).push(rand_num);
        }
    }
}

#[allow(unused_must_use)]
fn gen_array_from_user(len: usize) -> Vec<i8> {
    let mut return_arr:Vec<i8> = vec![];

    let mut current_input_size:usize= 0;
    while current_input_size != len {
        current_input_size = return_arr.len();

        let mut input = String::new();
        print!("{}번째 숫자를 입력해주세요: ", current_input_size+1);

        io::stdout().flush();

        io::stdin().read_line(&mut input);
        let int_value: Option<i8> = input.trim().parse().ok();

        match int_value {
            value @ Some(1...9) => {
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

//TODO use closure
fn get_sbo_result(inputs :Vec<i8>, answers :&Vec<i8>) -> SBO {
    let mut sbo = SBO{strike:0, ball:0, out:0};
    let mut is_heat = false;
    for (answer_value, answer_index) in answers.iter().enumerate() {
        for (input_value, input_index) in inputs.iter().enumerate() {
            if answer_value == input_value {
                if answer_index == input_index {
                    sbo.strike += 1;
                    is_heat = true;
                } else {
                    sbo.ball += 1;
                    is_heat = true;
                } 
            }
        }
        
        if is_heat == false {
            sbo.out += 1;
        }
    }
    
    sbo
}
