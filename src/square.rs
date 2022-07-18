use crate::{common, square_name};
use sdl2::rect::Point;
use std::{collections::HashMap};

#[derive(Copy, Clone)]
pub(crate) struct Square {
    pub(crate) position: Point,
    pub(crate) square_name: square_name::SquareName,
    pub(crate) is_occupied: bool,
}

// TODO write some tests for this
pub(crate) fn initialize_squares() -> Vec<Square> {
    (0..64)
        .map(|i| Square {
            position: square_name_to_position(square_name::SquareName::new(i)),
            square_name: square_name::SquareName::new(i),
            is_occupied: if (0..15).contains(&i) || (47..63).contains(&i) {
                true
            } else {
                false
            },
        })
        .collect::<Vec<Square>>()
}

//This is going from left to right
// TODO write some tests for this
pub(crate) fn square_name_to_position(square_name: square_name::SquareName) -> Point {
    let x: i32 = match square_name.file {
        'A' => common::HORIZONTAL_OFFSET + common::SQUARE_SIZE * 7,
        'B' => common::HORIZONTAL_OFFSET + common::SQUARE_SIZE * 6,
        'C' => common::HORIZONTAL_OFFSET + common::SQUARE_SIZE * 5,
        'D' => common::HORIZONTAL_OFFSET + common::SQUARE_SIZE * 4,
        'E' => common::HORIZONTAL_OFFSET + common::SQUARE_SIZE * 3,
        'F' => common::HORIZONTAL_OFFSET + common::SQUARE_SIZE * 2,
        'G' => common::HORIZONTAL_OFFSET + common::SQUARE_SIZE,
        'H' => common::HORIZONTAL_OFFSET,
        _ => panic!("Incorrect file value, {}", square_name.file),
    };

    let y: i32 = match square_name.rank {
        1 => common::VERTICLE_OFFSET,
        2 => common::VERTICLE_OFFSET + common::SQUARE_SIZE,
        3 => common::VERTICLE_OFFSET + common::SQUARE_SIZE * 2,
        4 => common::VERTICLE_OFFSET + common::SQUARE_SIZE * 3,
        5 => common::VERTICLE_OFFSET + common::SQUARE_SIZE * 4,
        6 => common::VERTICLE_OFFSET + common::SQUARE_SIZE * 5,
        7 => common::VERTICLE_OFFSET + common::SQUARE_SIZE * 6,
        8 => common::VERTICLE_OFFSET + common::SQUARE_SIZE * 7,
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
        if ((common::SQUARE_SIZE * i)..(common::SQUARE_SIZE * (i + 1))).contains(&y) {
            rank = (i + 1).try_into().unwrap();
        }
        if (common::SQUARE_SIZE * i..common::SQUARE_SIZE * (i + 1)).contains(&x) {
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
