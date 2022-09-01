#[derive(PartialEq, Copy, Clone)]
pub struct Wall {
    pub set: bool,
    pub is_column: bool,
    pub column: i32,
    pub row: i32,
}
