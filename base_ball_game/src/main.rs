use std::io;
use std::io::Write;


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
//TODO generate random data
    let answers:Vec<i8> = vec![1,2,3]; 
    let mut sbo = SBO{strike:0, ball:0, out:0};

    loop {
//TODO Modulization
        let current_input_size = inputs.len();

        let mut input = String::new();
        print!("{}번째 숫자를 입력해주세요: ", current_input_size+1);
        io::stdout().flush();

        io::stdin().read_line(&mut input);
        let int_value: Option<i8> = input.trim().parse().ok();

//TODO input data validation
        match int_value {
            Some(value) => {
                inputs.push(value);
                print!("int value = {}", value);
                io::stdout().flush();
            },
            None => continue
        }
        print!("input data : {}", input);

        if current_input_size == 2{
            break;
        }
    }

    print!("빠져나왔습니다");

//Check input data
//TODO Modulization
    let mut is_heat = false;
    let mut answer_value: i8 = 0;
    let mut input_value: i8 = 0;
    for answer_index in 0..answers.len() {
        answer_value = answers[answer_index];
        
        for input_index in 0..inputs.len() {
            input_value = inputs[input_index];
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

    print!("s : {}, b : {}, o : {}", sbo.strike, sbo.ball, sbo.out);

}
