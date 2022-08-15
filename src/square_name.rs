use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
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

    fn file_left(&self) -> char {
        let left_file_mapping = HashMap::from([
            ('B', 'A'),
            ('C', 'B'),
            ('D', 'C'),
            ('E', 'D'),
            ('F', 'E'),
            ('G', 'F'),
            ('H', 'G')
        ]);

        left_file_mapping[&self.file]
    }

    fn file_right(&self) -> char {
        let right_file_mapping: HashMap<char, char> = HashMap::from([
            ('A', 'B'),
            ('B', 'C'),
            ('C', 'D'),
            ('D', 'E'),
            ('E', 'F'),
            ('F', 'G'),
            ('G', 'H')
        ]);

        right_file_mapping[&self.file]
    }

    // return none if square is off the board
    // THIS IS INCREASING THE RANK BY ONE NO MATTER WHAT SIDE
    pub fn diagonal_up_left(&self) ->  Option<SquareName> {
        if self.file == 'A' || self.rank == 8  {
            return None;
        }
        return Some(SquareName {file: self.file_left(), rank: self.rank-1 });
    }

    pub fn diagonal_up_right(&self) -> Option<SquareName> {
        if self.file == 'H' || self.rank == 8 {
            return None;
        }
        return Some(SquareName {file: self.file_right(), rank: self.rank-1});
    }

    pub fn diagonal_down_left(&self) -> Option<SquareName> {
        if self.file == 'A' || self.rank == 1 {
            return None;
        }
        return Some(SquareName {file: self.file_left(), rank: self.rank+1});
    }

    pub fn diagonal_down_right(&self) -> Option<SquareName> {
        if self.file == 'H' || self.rank == 1 {
            return None;
        }
        return Some(SquareName {file: self.file_right(), rank: self.rank+1});
    }


}
