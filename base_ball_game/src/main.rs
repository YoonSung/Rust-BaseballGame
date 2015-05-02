use std::io;
use std::io::Write;
extern crate rand;
use rand::distributions::{IndependentSample, Range};

struct SBO {
    strike: i8,
    ball  : i8,
    out   : i8
}

fn main() {

    println!("========================");
    println!("       숫자 야구게임       ");
    println!("========================");

    let mut inputs:Vec<i8> = vec![];
    let mut answers:Vec<i8> = vec![]; 

    let between = Range::new(1i8, 10);
    gen_rand_array(&mut answers, 3, between); 
    inputs = gen_array_from_user(3);

    //Check input data
    let sbo = get_sbo_result(inputs, &answers);

    println!("correct answer : {}. {}. {}", answers[0], answers[1],answers[2]);
    println!("\nGame result\n-----------  \ns : {}\nb : {}\no : {}", sbo.strike, sbo.ball, sbo.out);

}

fn gen_rand_array(arr: &mut Vec<i8>, len: i8, range: Range<i8>) {
    let mut rng = rand::thread_rng();

    while (arr.len() != 3) {
        let rand_num = range.ind_sample(&mut rng);
        if (*arr).contains(&rand_num) {
            continue;
        } else { 
            (*arr).push(rand_num);
        }
    }
}

//TODO input data validation
fn gen_array_from_user(len: usize) -> Vec<i8> {
    let mut return_arr:Vec<i8> = vec![];

    loop {
        let current_input_size = return_arr.len();

        let mut input = String::new();
        print!("{}번째 숫자를 입력해주세요: ", current_input_size+1);
        io::stdout().flush();

        io::stdin().read_line(&mut input);
        let int_value: Option<i8> = input.trim().parse().ok();

        match int_value {
            Some(value) => {
                return_arr.push(value);
            },
            None => continue
        }

        if current_input_size == len-1 {
            break;
        }
    }

    return_arr 
}

//TODO Modulization
fn get_sbo_result(inputs :Vec<i8>, answers :&Vec<i8>) -> SBO {
    let mut sbo = SBO{strike:0, ball:0, out:0};
    let mut is_heat = false;
    let mut answer_value: i8 = 0;
    let mut input_value: i8 = 0;
    for answer_index in 0..(answers).len() {
        answer_value = (answers)[answer_index];
        
        for input_index in 0..(inputs).len() {
            input_value = (inputs)[input_index];
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
