use std::fs;
use std::env;

#[derive(Debug, Clone, Copy)]
struct MapCoord {
    row: usize,
    col: usize
}

impl MapCoord {
    /// Checks to see if "other" is directly below self
    pub fn is_below( &self, other: &MapCoord ) -> bool {
        (other.row + 1) == self.row && 
        other.col == self.col
    }

    /// Checks to see if "other" is directly above self
    pub fn is_above( &self, other: &MapCoord ) -> bool {
        other.row == (self.row + 1) && 
        other.col == self.col
    }

    /// Checks to see if "other" is directly to the left of self
    pub fn is_left_of( &self, other: &MapCoord ) -> bool {
        other.row == self.row && 
        other.col == (self.col + 1)
    }

    /// Checks to see if "other" is directly to the right of self
    pub fn is_right_of( &self, other: &MapCoord ) -> bool {
        other.row == self.row && 
        (other.col + 1) == self.col
    }
}

#[derive(Debug)]
struct PipeMap {
    map: String,
    rows: usize,
    cols: usize
}

impl PipeMap {
    pub fn parse( input: &str ) -> PipeMap {
        let row_count: usize = input.chars().filter(|c| *c == '\n').count() + 1; // assumes no line-return at end of input
        let col_count: usize = input.find('\n').unwrap();

        let pipes_graph: String = input.chars().filter(|c| *c != '\n').collect();

        PipeMap{ map: ( pipes_graph ), rows: (row_count), cols: (col_count) }
    }

    /// Gets the MapCoord of the first char in the map that equals "find_me"
    pub fn get_map_coord( &self, find_me: char ) -> Option<MapCoord> {
        let location: usize = self.map.find(find_me)?;
        let row: usize = location / self.cols;
        let col: usize = location % self.cols;
        Some( MapCoord { row: (row), col: (col) } )
    }

    pub fn transit_pipe( &self, 
                         from_pos: MapCoord, 
                         cur_pos: MapCoord ) -> Direction {
        // TODO if from_pos == cur_pos then we're stuck, too
        // Maybe make this Result<Direction, MoveErr>?
        match self.get_cell(&cur_pos) {
            // is this "fast" Rust? correct "Rust"? idk.
            Some('|') => {
                if cur_pos.is_above(&from_pos) {
                    // Self::get_map_coord_above(&self, &cur_pos)
                    Direction::Up
                } else if cur_pos.is_below(&from_pos) {
                    // Self::get_map_coord_below(&self, &cur_pos)
                    Direction::Down
                } else {
                    // None
                    Direction::Stuck
                }
            },
            Some('-') => {
                if cur_pos.is_left_of(&from_pos) {
                    // Self::get_map_coord_left_of(&self, &cur_pos)
                    Direction::Left
                } else if cur_pos.is_right_of(&from_pos) {
                    // Self::get_map_coord_right_of(&self, &cur_pos)
                    Direction::Right
                } else {
                    // None
                    Direction::Stuck
                }
            },
            Some('L') => {
                if cur_pos.is_left_of(&from_pos) {
                    // Self::get_map_coord_above(&self, &cur_pos)
                    Direction::Up
                } else if cur_pos.is_below(&from_pos) {
                    // Self::get_map_coord_right_of(&self, &cur_pos)
                    Direction::Right
                } else {
                    // None
                    Direction::Stuck
                }
            },
            Some('J') => {
                if cur_pos.is_right_of(&from_pos) {
                    // Self::get_map_coord_above(&self, &cur_pos)
                    Direction::Up
                } else if cur_pos.is_below(&from_pos) {
                    // Self::get_map_coord_left_of(&self, &cur_pos)
                    Direction::Left
                } else {
                    // None
                    Direction::Stuck
                }
            },
            Some('7') => {
                if cur_pos.is_right_of(&from_pos) {
                    // Self::get_map_coord_below(&self, &cur_pos)
                    Direction::Down
                } else if cur_pos.is_above(&from_pos) {
                    // Self::get_map_coord_left_of(&self, &cur_pos)
                    Direction::Left
                } else {
                    // None
                    Direction::Stuck
                }
            },
            Some('F') => {
                if cur_pos.is_left_of(&from_pos) {
                    // Self::get_map_coord_below(&self, &cur_pos)
                    Direction::Down
                } else if cur_pos.is_above(&from_pos) {
                    // Self::get_map_coord_right_of(&self, &cur_pos)
                    Direction::Right
                } else {
                    // None
                    Direction::Stuck
                }
            }
            Some('.') => Direction::Stuck,
            Some('S') => Direction::Goal,
            _ => unreachable!(),
        }
    }

    /// Returns the character (pipe) contained at the MapCoord located in this map.
    /// 
    /// If the MapCoord falls outside of the bounds of this PipeMap, None is returned.
    pub fn get_cell( &self, loc: &MapCoord ) -> Option<char> {
        if loc.col >= self.cols {
            None
        } else if loc.row >= self.rows {
            None
        } else {
            let flat_pos: usize = loc.row * self.cols + loc.col;
            self.map.chars().nth(flat_pos)
        }
    }

    pub fn get_map_coord_above( &self, coord: &MapCoord ) -> Option<MapCoord> {
        if coord.row == 0 {
            None
        } else {
            Some( MapCoord { row: (coord.row - 1), col: (coord.col) } )
        }
    }

    pub fn get_map_coord_below( &self, coord: &MapCoord ) -> Option<MapCoord> {
        if coord.row == self.rows {
            None
        } else {
            Some( MapCoord { row: (coord.row + 1), col: (coord.col) } )
        }
    }

    pub fn get_map_coord_left_of( &self, coord: &MapCoord ) -> Option<MapCoord> {
        if coord.col == 0 {
            None
        } else {
            Some( MapCoord { row: (coord.row), col: (coord.col - 1) } )
        }
    }

    pub fn get_map_coord_right_of( &self, coord: &MapCoord ) -> Option<MapCoord> {
        if coord.col == self.cols {
            None
        } else {
            Some( MapCoord { row: (coord.row), col: (coord.col + 1) } )
        }
    }

    // IDEA: make function that returns iter() for all cardinal directions around 'S'?
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Direction {
   Up,
   Down,
   Left,
   Right,
   Stuck,
   Goal
}

struct MapWalker<'a> {
   map: &'a PipeMap,
   last_pos: MapCoord,
   cur_pos: MapCoord,
   walk_counter: u32
}

impl<'a> MapWalker<'a> {
   pub fn new( map: &'a PipeMap, start_pos: MapCoord, dir: Direction ) -> MapWalker {
      let mut mw = MapWalker { map: (map), last_pos: (start_pos), cur_pos: (start_pos), walk_counter: (0) };
      mw.shove(dir);
      mw
   }

   fn relocate( &mut self, pos: Option<MapCoord> ) {
      if pos.is_some() {
         self.last_pos = self.cur_pos;
         self.cur_pos = pos.unwrap();
         self.walk_counter += 1; 
      }
   }

   fn shove( &mut self, dir: Direction ) {
      match dir {
         Direction::Up => {
            Self::relocate(self, self.map.get_map_coord_above(&self.cur_pos) );
         },
         Direction::Down => {
            Self::relocate(self, self.map.get_map_coord_below(&self.cur_pos) );
         },
         Direction::Left => {
            Self::relocate(self, self.map.get_map_coord_left_of(&self.cur_pos) );
         },
         Direction::Right => {
            Self::relocate(self, self.map.get_map_coord_right_of(&self.cur_pos) );
         },
         Direction::Stuck => {},
         Direction::Goal => {},
      }
   }

   pub fn step( &mut self ) -> Direction {
      let next_direction = self.map.transit_pipe(self.last_pos, self.cur_pos);
      Self::shove(self, next_direction);
      next_direction
   }
}

fn main() {
    // get input
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];

    // read
    let pipes_raw: String = fs::read_to_string(file_path).unwrap();
    println!("{pipes_raw}");

    let pipes: PipeMap = PipeMap::parse(&pipes_raw);

    // println!("{pipes:?}"); // DEBUG

    let starting_loc: MapCoord = pipes.get_map_coord('S').unwrap();
    println!("Starting location for S [{starting_loc:?}]");

    let mut walkers: Vec<MapWalker> = vec![MapWalker::new(&pipes, starting_loc, Direction::Up),
                                           MapWalker::new(&pipes, starting_loc, Direction::Down),
                                           MapWalker::new(&pipes, starting_loc, Direction::Left),
                                           MapWalker::new(&pipes, starting_loc, Direction::Right)];

    loop {
       if walkers.iter_mut().any(|mw| mw.step() == Direction::Goal ) {
         break;
       }
    }

    for mw in walkers {
      println!("steps: [{}]", mw.walk_counter);
    }
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_above() {
      let primary = MapCoord{ row: (0), col: (0) };
      let other = MapCoord{ row: (1), col: (0) };
      assert_eq!(primary.is_above(&other), true);

      let primary = MapCoord{ row: (1), col: (0) };
      let other = MapCoord{ row: (0), col: (0) };
      assert_eq!(primary.is_above(&other), false);
   }

   #[test]
   fn test_below() {
      let primary = MapCoord{ row: (1), col: (0) };
      let other = MapCoord{ row: (0), col: (0) };
      assert_eq!(primary.is_below(&other), true);

      let primary = MapCoord{ row: (1), col: (0) };
      let other = MapCoord{ row: (0), col: (1) };
      assert_eq!(primary.is_below(&other), false);

      let primary = MapCoord{ row: (0), col: (0) };
      let other = MapCoord{ row: (1), col: (0) };
      assert_eq!(primary.is_below(&other), false);
   }

   #[test]
   fn test_left() {
      let primary = MapCoord{ row: (0), col: (0) };
      let other = MapCoord{ row: (0), col: (1) };
      assert_eq!(primary.is_left_of(&other), true);

      let primary = MapCoord{ row: (0), col: (1) };
      let other = MapCoord{ row: (0), col: (0) };
      assert_eq!(primary.is_left_of(&other), false);
   }

   #[test]
   fn test_right() {
      let primary = MapCoord{ row: (0), col: (1) };
      let other = MapCoord{ row: (0), col: (0) };
      assert_eq!(primary.is_right_of(&other), true);

      let primary = MapCoord{ row: (0), col: (0) };
      let other = MapCoord{ row: (0), col: (1) };
      assert_eq!(primary.is_right_of(&other), false);
   }
}
