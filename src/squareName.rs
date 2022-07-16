
#[derive(Copy, Clone)]
pub struct SquareName {
    file: char,
    rank: u8
}

impl SquareName {
    //NEED TO IMPLEMENT THIS
    pub fn calc_square_name(num: i32) -> SquareName {
        let mut file: String = match num%8 {
            0 => String::from("A"),
            1 => String::from("B"),
            2 => String::from("C"),
            3 => String::from("D"),
            4 => String::from("E"),
            5 => String::from("F"),
            6 => String::from("G"),
            7 => String::from("H"),
            _ => String::from("ERROR")
        };
 
        let rank: &str = &(num/8).to_string();
 
        file.push_str(rank);

        SquareName { file: 'Z', rank: 0 }
    }
}