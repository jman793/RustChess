use sdl2::rect::Point;

enum Color {
    WHITE,
    BLACK,
}

enum PieceType {
    KNIGHT,
    QUEEN,
    PAWN,
    BISHOP,
    ROOK,
    KING,
}

//position is where a piece goes (center of square)
pub struct Square {
    position: Point,
    color: Color,
    name: String,
    is_occupied: bool
}

pub struct Piece {
    square: Square,
    color: Color,
    piece_type: PieceType,
    is_taken: bool,
}

fn get_pieces() {

}

pub fn initialize_squares() -> Vec<Square> {
    (0..64)
    .map(|i| Square {
        position: Point::new(6+13*i, 5+9*(i/8)),
        color: if i%2==0 {Color::BLACK} else {Color::WHITE},
        name: calc_square_name(i),
        is_occupied: if (0..15).contains(&i) || (47..63).contains(&i) {true} else {false}       
    })
    .collect::<Vec<Square>>()
}

fn calc_square_name(num: i32) -> String {
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

    file
}