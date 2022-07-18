#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SquareName {
    pub file: char,
    pub rank: u8,
}

impl SquareName {
    // TODO write some tests for this
    pub fn new(num: i32) -> SquareName {
        let file: char = match num % 8 {
            0 => 'H',
            1 => 'G',
            2 => 'F',
            3 => 'E',
            4 => 'D',
            5 => 'C',
            6 => 'B',
            7 => 'A',
            _ => panic!("This will never happen"),
        };

        SquareName {
            file: file,
            rank: (num / 8 + 1).try_into().unwrap(),
        }
    }
}
