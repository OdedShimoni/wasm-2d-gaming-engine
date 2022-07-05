// TODO implement PartrialEq for point if needed

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Point(pub u32, pub u32); // (x, y)

#[derive(Clone, Debug, Copy)]
pub struct Rect(pub u32, pub u32); // (width, height)

pub fn get_index(width: u32, row: u32, column: u32) -> usize {
    (row * width + column) as usize
}