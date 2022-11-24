pub struct Cell {
    pub x: i32,          // Cell x position
    pub y: i32,          // Cel
    pub alive: bool,     // Cell alive state
    pub will_live: bool, // Cell alive state in the next generation
}

impl Cell {
    pub fn new(x: i32, y: i32, alive: bool, will_live: bool) -> Cell {
        return Cell {
            x: x,
            y: y,
            alive: alive,
            will_live: will_live,
        };
    }
}
