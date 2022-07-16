use sdl2::rect::Point;
use crate::piece::Color;
use crate::squareName;

pub(crate) const SQUARE_WIDTH: i32 = 13;
pub(crate) const SQUARE_HEIGHT: i32 = 9;
const HORIZ_MIDDLE: i32 = 6;
const VER_MIDDLE: i32 = 5;

pub struct Square {
    pub(crate) position: Point,
    color: Color,
    pub(crate) square_name: squareName::SquareName,
    is_occupied: bool
}

//THE POSITION OF THE SQUARES IS NOT BEING CALCULATED PROPERLY AND NEEDS TO BE FIXED
pub fn initialize_squares() -> Vec<Square> {
    (0..64)
    .map(|i| Square {
        position: Point::new(HORIZ_MIDDLE+SQUARE_WIDTH*i, VER_MIDDLE+SQUARE_HEIGHT*(i/8)),
        color: if i%2==0 {Color::BLACK} else {Color::WHITE},
        square_name: squareName::SquareName::calc_square_name(i),
        is_occupied: if (0..15).contains(&i) || (47..63).contains(&i) {true} else {false}       
    })
    .collect::<Vec<Square>>()
}


// Consider just making square name a struct of rank and file that way it is much more specific on how move calculation works 
// That will allow you to hide more data and also allow for this to be less literal

