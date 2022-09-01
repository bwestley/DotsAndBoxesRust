use crate::wall::Wall;
#[derive(PartialEq, Copy, Clone)]
pub struct SquareWalls {
    pub top: Wall,
    pub right: Wall,
    pub bottom: Wall,
    pub left: Wall,
}

impl SquareWalls {
    pub fn get_walls(&self, set: bool) -> Vec<Wall> {
        let mut walls = Vec::new();
        if self.top.set == set {
            walls.push(self.top)
        };
        if self.right.set == set {
            walls.push(self.right)
        };
        if self.bottom.set == set {
            walls.push(self.bottom)
        };
        if self.left.set == set {
            walls.push(self.left)
        };
        return walls;
    }

    pub fn get_first_wall(&self, set: bool) -> Option<Wall> {
        if self.top.set == set {
            return Some(self.top);
        };
        if self.right.set == set {
            return Some(self.right);
        };
        if self.bottom.set == set {
            return Some(self.bottom);
        };
        if self.left.set == set {
            return Some(self.left);
        };
        return None;
    }
}
