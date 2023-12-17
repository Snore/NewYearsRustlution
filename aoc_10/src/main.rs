use std::fs;
use std::env;

#[derive(Debug)]
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
                         cur_pos: MapCoord ) -> Option<MapCoord> {
        match self.get_cell(&cur_pos) {
            // is this "fast" Rust? correct "Rust"? idk.
            Some('|') => {
                if cur_pos.is_above(&from_pos) {
                    Self::get_map_coord_above(&self, &cur_pos)
                } else if cur_pos.is_below(&from_pos) {
                    Self::get_map_coord_below(&self, &cur_pos)
                } else {
                    None
                }
            },
            Some('-') => {
                if cur_pos.is_left_of(&from_pos) {
                    Self::get_map_coord_left_of(&self, &cur_pos)
                } else if cur_pos.is_right_of(&from_pos) {
                    Self::get_map_coord_right_of(&self, &cur_pos)
                } else {
                    None
                }
            },
            Some('L') => {
                if cur_pos.is_left_of(&from_pos) {
                    Self::get_map_coord_above(&self, &cur_pos)
                } else if cur_pos.is_below(&from_pos) {
                    Self::get_map_coord_right_of(&self, &cur_pos)
                } else {
                    None
                }
            },
            Some('J') => {
                if cur_pos.is_right_of(&from_pos) {
                    Self::get_map_coord_above(&self, &cur_pos)
                } else if cur_pos.is_below(&from_pos) {
                    Self::get_map_coord_left_of(&self, &cur_pos)
                } else {
                    None
                }
            },
            Some('7') => {
                if cur_pos.is_right_of(&from_pos) {
                    Self::get_map_coord_below(&self, &cur_pos)
                } else if cur_pos.is_above(&from_pos) {
                    Self::get_map_coord_left_of(&self, &cur_pos)
                } else {
                    None
                }
            },
            Some('F') => {
                if cur_pos.is_left_of(&from_pos) {
                    Self::get_map_coord_below(&self, &cur_pos)
                } else if cur_pos.is_above(&from_pos) {
                    Self::get_map_coord_right_of(&self, &cur_pos)
                } else {
                    None
                }
            }
            Some('.') => None,
            Some('S') => None,
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

    fn get_map_coord_above( &self, coord: &MapCoord ) -> Option<MapCoord> {
        if coord.row == 0 {
            None
        } else {
            Some( MapCoord { row: (coord.row - 1), col: (coord.col) } )
        }
    }

    fn get_map_coord_below( &self, coord: &MapCoord ) -> Option<MapCoord> {
        if coord.row == self.rows {
            None
        } else {
            Some( MapCoord { row: (coord.row + 1), col: (coord.col) } )
        }
    }

    fn get_map_coord_left_of( &self, coord: &MapCoord ) -> Option<MapCoord> {
        if coord.col == 0 {
            None
        } else {
            Some( MapCoord { row: (coord.row), col: (coord.col - 1) } )
        }
    }

    fn get_map_coord_right_of( &self, coord: &MapCoord ) -> Option<MapCoord> {
        if coord.col == self.cols {
            None
        } else {
            Some( MapCoord { row: (coord.row), col: (coord.col + 1) } )
        }
    }

    // IDEA: make function that returns iter() for all cardinal directions around 'S'?
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

    // ok, game plan
    // grab width and height of the input.
    // make psudo 2d vector over the input.
    // use the input as the graph.
    // REMEMBER TO IGNORE THE '\n', or strip them!!!
    // can use switch statement in order to navigate based on the character.
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
