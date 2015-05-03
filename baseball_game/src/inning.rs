pub struct SBO {
   pub strike: i8,
   pub ball  : i8,
   pub out   : i8
}

pub fn get_sbo_result(inputs :Vec<i8>, answers :&Vec<i8>) -> SBO {
    let mut sbo = SBO{strike:0, ball:0, out:0};

    each_iterator_do(&inputs, |value, index| {
        if answers.contains(&value) == true {
            if answers[index] == value {
                sbo.strike += 1;
            } else {
                sbo.ball += 1;
            }

        } else {
            sbo.out += 1;
        }
    });

    sbo
}

//@see https://github.com/csherratt/gl-rs/commit/fcedd846c40ad6759b4a9cca69ff8f72b62a07b2
fn each_iterator_do<F>(arr: &Vec<i8>, mut execute: F) where F: FnMut(i8, usize) {
    for (index, value) in arr.iter().enumerate() {
        execute(*value, index);
    }
}
