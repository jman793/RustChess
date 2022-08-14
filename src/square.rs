use crate::{common::constants, square_name};
use sdl2::rect::Point;
use std::collections::HashMap;

#[derive(Copy, Clone)]
pub struct Square {
    pub(crate) position: Point,
    pub(crate) is_occupied: bool,
}

//This is going from left to right
// TODO write some tests for this
pub(crate) fn square_name_to_position(square_name: square_name::SquareName) -> Point {
    let x: i32 = match square_name.file {
        'H' => constants::HORIZONTAL_OFFSET + constants::SQUARE_SIZE * 7,
        'G' => constants::HORIZONTAL_OFFSET + constants::SQUARE_SIZE * 6,
        'F' => constants::HORIZONTAL_OFFSET + constants::SQUARE_SIZE * 5,
        'E' => constants::HORIZONTAL_OFFSET + constants::SQUARE_SIZE * 4,
        'D' => constants::HORIZONTAL_OFFSET + constants::SQUARE_SIZE * 3,
        'C' => constants::HORIZONTAL_OFFSET + constants::SQUARE_SIZE * 2,
        'B' => constants::HORIZONTAL_OFFSET + constants::SQUARE_SIZE,
        'A' => constants::HORIZONTAL_OFFSET,
        _ => panic!("Incorrect file value, {}", square_name.file),
    };

    let y: i32 = match square_name.rank {
        8 => constants::VERTICLE_OFFSET,
        7 => constants::VERTICLE_OFFSET + constants::SQUARE_SIZE,
        6 => constants::VERTICLE_OFFSET + constants::SQUARE_SIZE * 2,
        5 => constants::VERTICLE_OFFSET + constants::SQUARE_SIZE * 3,
        4 => constants::VERTICLE_OFFSET + constants::SQUARE_SIZE * 4,
        3 => constants::VERTICLE_OFFSET + constants::SQUARE_SIZE * 5,
        2 => constants::VERTICLE_OFFSET + constants::SQUARE_SIZE * 6,
        1 => constants::VERTICLE_OFFSET + constants::SQUARE_SIZE * 7,
        _ => panic!("Incorrect Rank value, {}", square_name.rank),
    };

    Point::new(x, y)
}

// TODO write some tests for this
pub(crate) fn select_square_from_position(x: i32, y: i32) -> square_name::SquareName {
    let index_to_file_map: HashMap<i32, char> = HashMap::from([
        (0, 'A'),
        (1, 'B'),
        (2, 'C'),
        (3, 'D'),
        (4, 'E'),
        (5, 'F'),
        (6, 'G'),
        (7, 'H'),
    ]);

    let mut rank: u8 = 9;
    let mut file: char = 'Z';
    for i in 0..8 {
        if ((constants::SQUARE_SIZE * i)..(constants::SQUARE_SIZE * (i + 1))).contains(&y) {
            rank = (i + 1).try_into().unwrap();
        }
        if (constants::SQUARE_SIZE * i..constants::SQUARE_SIZE * (i + 1)).contains(&x) {
            file = index_to_file_map[&i];
        }
    }

    if (rank == 9) || (file == 'Z') {
        panic!("Square not found. This should not happen");
    }

    match rank {
        1 => rank = 8,
        2 => rank = 7,
        3 => rank = 6,
        4 => rank = 5,
        5 => rank = 4,
        6 => rank = 3,
        7 => rank = 2,
        8 => rank = 1,
        _ => panic!("This will not happen. Rank is between 1 and 8."),
    }

    square_name::SquareName {
        file: file,
        rank: rank,
    }
}

impl Square {
    // TODO write some tests for this
    pub(crate) fn initialize_squares() -> HashMap<square_name::SquareName, Square> {
        (0..64)
            .map(|i| {
                (
                    square_name::SquareName::new(i),
                    Square {
                        position: square_name_to_position(square_name::SquareName::new(i)),
                        is_occupied: if (0..15).contains(&i) || (47..63).contains(&i) {
                            true
                        } else {
                            false
                        },
                    },
                )
            })
            .collect()
    }
}
