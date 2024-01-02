use std::fs;
use std::env;
use std::fmt;
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// A x,y coordinate on a 2D map
struct MapCoord {
    /// The X variable of this coordinate
    row: usize,
    /// The Y variable of this coordinate
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

    /// Returns the Direction that is MapCoord is relative to other
    /// 
    /// Returns None if other is not adjacent to this
    pub fn get_rel_pos_of( &self, other: &MapCoord ) -> Option<Direction> {
        if self.is_above(other) {
            return Some(Direction::Up);
        } else if self.is_below(other) {
            return Some(Direction::Down);
        } else if self.is_left_of(other) {
            return Some(Direction::Left);
        } else if self.is_right_of(other) {
            return Some(Direction::Right);
        }
        None
    }
}

#[derive(Debug)]
struct PipeMap {
    /// The raw map data represented in 1 dimension
    map: Vec<char>,
    /// The number of rows in this map
    rows: usize,
    /// The number of columns in this map
    cols: usize
}

/// A map that projects a PipePath onto a PipeMap
struct PathMap {
    map: Vec<char>,
    cols: usize,
}

impl PathMap {
    /// Char denoting a pipe piece is occupying the slot
    const PIPE_CHAR_ANY: char = 'X';
    const PIPE_CHAR_VERT: char = '|';
    const PIPE_CHAR_HORZ: char = '-';
    const PIPE_CHAR_BEND_UP: char = '^';
    const PIPE_CHAR_BEND_DOWN: char = 'V';

    /// Creates a PathMap from a PipePath and a PipeMap
    pub fn create( origin: &PipeMap, path: &PipePath ) -> PathMap {
        // fill the map to size with default variable
        let mut canvas: Vec<char> = vec!['.'; origin.map.len()];

        // mark all of the nodes that path lies on with an 'X'
        for cell in &path.pipe_locs {
            // calculate the position in 1D
            let spot: usize = (cell.row * origin.cols) + cell.col;
            let symbol: char = origin.get_cell(cell).unwrap();
            canvas[spot] = match symbol {
               Self::PIPE_CHAR_VERT => Self::PIPE_CHAR_VERT,
               'L' | 'J' => Self::PIPE_CHAR_BEND_UP,
               'F' | '7' => Self::PIPE_CHAR_BEND_DOWN,
               'S' => {
                    // Locate the first step and last step of the path as this path is a loop and these two will connect to the start position
                    // since this should be a loop, this path's first and last element is the starting position
                    let loc_of_unknown: &MapCoord = &path.pipe_locs[0];
                    let first_loc: &MapCoord = &path.pipe_locs[1];
                    let last_loc: &MapCoord = &path.pipe_locs[path.len() - 2];

                    // figure out the start position based on the two connecting pipe pieces
                    Self::assess_pipe_connection(loc_of_unknown, 
                                                 first_loc, 
                                                 last_loc).unwrap()
               },
               _ => Self::PIPE_CHAR_HORZ,
            };
        }
        PathMap { map: (canvas), cols: (origin.cols) }
    }

    /// Takes two MapCoord and a pipe map and decerns what the pipe part must be connecting the two MapCoords
    fn assess_pipe_connection( loc_of_unkown: &MapCoord, 
                              loc_first_piece: &MapCoord, 
                              loc_second_piece: &MapCoord ) -> Option<char> {
        let rel_dir_to_first: Direction = loc_first_piece.get_rel_pos_of(loc_of_unkown).unwrap();
        let rel_dir_to_second: Direction = loc_second_piece.get_rel_pos_of(loc_of_unkown).unwrap();

        match (rel_dir_to_first, rel_dir_to_second) {
            (Direction::Down, Direction::Up) | (Direction::Up, Direction::Down) => Some(Self::PIPE_CHAR_VERT),
            (Direction::Down, _) | (_, Direction::Down) => Some(Self::PIPE_CHAR_BEND_DOWN),
            (Direction::Up, _) | (_, Direction::Up) => Some(Self::PIPE_CHAR_BEND_UP),
            _ => Some(Self::PIPE_CHAR_HORZ)
        }
    }

    /// Counts the number of cells surrounded by the path in this PathMap
    pub fn count_inner_cells( &self ) -> usize {
        let mut inside_counter: usize = 0;
        let mut inside_cache: usize = 0;
        let mut is_inside: bool = false;
        let mut is_slidding: char = Self::PIPE_CHAR_ANY;

        for row_slice in self.map.chunks(self.cols) {
            for c in row_slice {

                // this could just be a `match` block but idk what's prefered
                if *c == Self::PIPE_CHAR_VERT {
                    // toggle the inside activator!
                    if is_inside {
                        inside_counter += inside_cache;
                        inside_cache = 0;
                     }
                     is_inside = !is_inside;
                  } else if *c == Self::PIPE_CHAR_HORZ {
                     continue;
                  } else if *c == Self::PIPE_CHAR_BEND_UP {
                     if is_slidding == Self::PIPE_CHAR_ANY {
                        // nothing to up means we started a slide
                        is_slidding = Self::PIPE_CHAR_BEND_UP;
                     } else if is_slidding == Self::PIPE_CHAR_BEND_UP {
                        // up to up means we want to leave the 'is_inside' variable and reset the slidding marker
                        is_slidding = Self::PIPE_CHAR_ANY;
                     }
                     else {
                        //up to down means we crossed the pipe threshold and want to take action.
                        is_slidding = Self::PIPE_CHAR_ANY;
                        if is_inside {
                           inside_counter += inside_cache;
                           inside_cache = 0;
                        }
                        is_inside = !is_inside;
                     }
                  } else if *c == Self::PIPE_CHAR_BEND_DOWN {
                     // a "slide" is a stretch of horizontal pip parts that begin and end with a bend.
                     // if the next char is not a pipe, then it's safe to assume this is a end of the slide as we cannot have one bend
                     if is_slidding == Self::PIPE_CHAR_ANY {
                        // nothing to up means we started a slide
                        is_slidding = Self::PIPE_CHAR_BEND_DOWN;
                     } else if is_slidding == Self::PIPE_CHAR_BEND_DOWN {
                        // up to up means we want to leave the 'is_inside' variable and reset the slidding marker
                        is_slidding = Self::PIPE_CHAR_ANY;
                     }
                     else {
                        //down to up means we crossed the pipe threshold and want to take action.
                        is_slidding = Self::PIPE_CHAR_ANY;
                        if is_inside {
                           inside_counter += inside_cache;
                           inside_cache = 0;
                        }
                        is_inside = !is_inside;
                     }
                  } else {
                    if is_inside {
                        // count this cell as inside
                        inside_cache += 1;
                        // color it for debugging
                        // *c = 'I';
                     }
                  }
            }

            // reset inside counter
            is_inside = false;
            inside_cache = 0;
            is_slidding = Self::PIPE_CHAR_ANY;
        }
        
        inside_counter
    }
}

impl fmt::Display for PathMap {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        for row in self.map.chunks(self.cols) {
            for char in row{
                write!(f, "{}", char)?;
            }
            write!(f, "{}", "\n")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
enum PipeMapError {
   InvalidPipe,
   BadTransit,
}

impl fmt::Display for PipeMapError {
   fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
      match self {
         PipeMapError::InvalidPipe => write!(f, "Invalid pipe junction detected."),
         PipeMapError::BadTransit => write!(f, "Transit has no decernable direction."),
      }
   }
}

impl std::error::Error for PipeMapError {}

impl PipeMap {
    pub fn parse( input: &str ) -> PipeMap {
        let row_count: usize = input.chars().filter(|c| *c == '\n').count() + 1; // assumes no line-return at end of input
        let col_count: usize = input.find('\n').unwrap();

        let pipes_graph: Vec<char> = input.chars().filter(|c| *c != '\n').collect();

        PipeMap{ map: ( pipes_graph ), rows: (row_count), cols: (col_count) }
    }

    /// Gets the MapCoord of the first char in the map that equals "find_me"
    pub fn get_map_coord( &self, find_me: char ) -> Option<MapCoord> {
        let location: usize = self.map.iter().position(|c: &char| *c == find_me)?;
        let row: usize = location / self.cols;
        let col: usize = location % self.cols;
        Some( MapCoord { row: (row), col: (col) } )
    }

    pub fn transit_pipe( &self, 
                         from_pos: MapCoord, 
                         cur_pos: MapCoord ) -> Result<Direction, PipeMapError> {
        if from_pos == cur_pos {
            return Err(PipeMapError::BadTransit);
        }

        // Maybe make this Result<Direction, MoveErr>?
        match self.get_cell(&cur_pos) {
            // is this "fast" Rust? correct "Rust"? idk.
            Some('|') => {
                if cur_pos.is_above(&from_pos) {
                    Ok(Direction::Up)
                } else if cur_pos.is_below(&from_pos) {
                    Ok(Direction::Down)
                } else {
                    Err(PipeMapError::InvalidPipe)
                }
            },
            Some('-') => {
                if cur_pos.is_left_of(&from_pos) {
                    Ok(Direction::Left)
                } else if cur_pos.is_right_of(&from_pos) {
                    Ok(Direction::Right)
                } else {
                    Err(PipeMapError::InvalidPipe)
                }
            },
            Some('L') => {
                if cur_pos.is_left_of(&from_pos) {
                    Ok(Direction::Up)
                } else if cur_pos.is_below(&from_pos) {
                    Ok(Direction::Right)
                } else {
                    Err(PipeMapError::InvalidPipe)
                }
            },
            Some('J') => {
                if cur_pos.is_right_of(&from_pos) {
                    Ok(Direction::Up)
                } else if cur_pos.is_below(&from_pos) {
                    Ok(Direction::Left)
                } else {
                    Err(PipeMapError::InvalidPipe)
                }
            },
            Some('7') => {
                if cur_pos.is_right_of(&from_pos) {
                    Ok(Direction::Down)
                } else if cur_pos.is_above(&from_pos) {
                    Ok(Direction::Left)
                } else {
                    Err(PipeMapError::InvalidPipe)
                }
            },
            Some('F') => {
                if cur_pos.is_left_of(&from_pos) {
                    Ok(Direction::Down)
                } else if cur_pos.is_above(&from_pos) {
                    Ok(Direction::Right)
                } else {
                    Err(PipeMapError::InvalidPipe)
                }
            }
            Some('.') => Ok(Direction::Stuck),
            Some('S') => Ok(Direction::Goal),
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
            Some( self.map[flat_pos] )
        }
    }

    /// Returns the MapCoord above the provided MapCoord, or None if we walked off the map
    pub fn get_map_coord_above( &self, coord: &MapCoord ) -> Option<MapCoord> {
        if coord.row == 0 {
            None
        } else {
            Some( MapCoord { row: (coord.row - 1), col: (coord.col) } )
        }
    }

    /// Returns the MapCoord below the provided MapCoord, or None if we walked off the map
    pub fn get_map_coord_below( &self, coord: &MapCoord ) -> Option<MapCoord> {
        if coord.row == self.rows {
            None
        } else {
            Some( MapCoord { row: (coord.row + 1), col: (coord.col) } )
        }
    }

    /// Returns the MapCoord to the left of the provided MapCoord, or None if we walked off the map
    pub fn get_map_coord_left_of( &self, coord: &MapCoord ) -> Option<MapCoord> {
        if coord.col == 0 {
            None
        } else {
            Some( MapCoord { row: (coord.row), col: (coord.col - 1) } )
        }
    }

    /// Returns the MapCoord to the right of the provided MapCoord, or None if we walked off the map
    pub fn get_map_coord_right_of( &self, coord: &MapCoord ) -> Option<MapCoord> {
        if coord.col == self.cols {
            None
        } else {
            Some( MapCoord { row: (coord.row), col: (coord.col + 1) } )
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
   Up,
   Down,
   Left,
   Right,
   Stuck,
   Goal
}

#[derive(Clone)]
/// A vector of coordinates denoting the path along a map.
/// 
/// The first element is the starting block, the last is where the walker finished.
struct PipePath {
   pipe_locs: Vec<MapCoord>
}

impl PipePath {
    fn push( &mut self, location: &MapCoord ) {
        self.pipe_locs.push(*location);
    }

    fn len( &self ) -> usize {
        self.pipe_locs.len()
    }
}

#[derive(Clone)]
struct MapWalker<'a> {
   map: &'a PipeMap,
   last_pos: MapCoord,
   cur_pos: MapCoord,
   path: PipePath
}

impl<'a> MapWalker<'a> {
    pub fn new( map: &'a PipeMap, start_pos: MapCoord, dir: Direction ) -> MapWalker {
        let mut mw = MapWalker { map: (map), 
                                                last_pos: (start_pos), 
                                                cur_pos: (start_pos), 
                                                path: ( PipePath { pipe_locs: ( vec![start_pos] ) } ) };
        mw.shove(dir);
        mw
    }

    fn relocate( &mut self, pos: Option<MapCoord> ) {
        if pos.is_some() {
        
        self.last_pos = self.cur_pos;
        self.cur_pos = pos.unwrap();
         
        // append the next step to our path
        self.path.push(&self.cur_pos);
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
        let potential_direction = self.map.transit_pipe(self.last_pos, self.cur_pos);
        let next_direction: Direction = match potential_direction {
            Ok(direction) => direction,
            Err(_) => Direction::Stuck,
        };

        Self::shove(self, next_direction);
        next_direction
    }

    /// Runs this MapWalker until it runs out of moves
    pub fn explore( &mut self ) -> Direction {
        let mut final_direction: Direction = Direction::Up;

        // make a quick lambda that is only relevent to this function
        let is_done = |d: Direction| d == Direction::Stuck || d == Direction::Goal;
        // run until this walker is out of moves
        while !is_done(final_direction) {
            final_direction = self.step();
        }

        final_direction
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

    let starting_loc: MapCoord = pipes.get_map_coord('S').unwrap();
    println!("Starting location for S [{starting_loc:?}]");

    // make a walker for each of the cardinal directions
    let mut walkers: Vec<MapWalker> = vec![MapWalker::new(&pipes, starting_loc, Direction::Up),
                                           MapWalker::new(&pipes, starting_loc, Direction::Down),
                                           MapWalker::new(&pipes, starting_loc, Direction::Left),
                                           MapWalker::new(&pipes, starting_loc, Direction::Right)];

    // run until one of the walkers reaches the the 'S' again
    let timing_start_1: Instant = Instant::now();

    let mut best_walker: Option<&MapWalker> = None;
    for walker in &mut walkers {
        if walker.explore() == Direction::Goal {
            best_walker = Some(walker);
            break;
        }
    }

    let furthest: usize = best_walker.unwrap().path.len() / 2;
    let elapsed_1: std::time::Duration = timing_start_1.elapsed();

    // start part 2
    let timing_start_2: Instant = Instant::now();
    // create a new map that has the pipe cells 'X'ed out
    let pipe_drawing: PathMap = PathMap::create(&pipes, &best_walker.unwrap().path);

    let inner_node_count: usize = pipe_drawing.count_inner_cells();
    let elapsed_2: std::time::Duration = timing_start_2.elapsed();

    // print answers
    println!("{pipe_drawing}");
    println!("The furthest spot from the start is [{furthest}] taking [{elapsed_1:?}]");
    println!("The number of nodes surrounded by the pipe is [{inner_node_count}] taking [{elapsed_2:?}]");
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
