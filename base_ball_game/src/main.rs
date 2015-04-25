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

    let mut inputs = vec![];

    loop {

        let current_input_size = inputs.len();

        let mut input = String::new();
        print!("{}번째 숫자를 입력해주세요: ", current_input_size+1);
        io::stdout().flush();

        io::stdin().read_line(&mut input);
        print!("input data : {}", input);
//TODO input data validation
        inputs.push(input);

        if current_input_size == 2{
            break;
        }
    }

    print!("빠져나왔습니다");

}
