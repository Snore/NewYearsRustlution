use std::fs;
use std::env;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// A x,y coordinate on a 2D map
struct MapCoord {
    /// The X variable of this coordinate
    row: usize,
    /// The Y variable of this coordinate
    col: usize
}

trait Mappable {
    fn cols( &self ) -> &usize;
    fn rows( &self ) -> &usize;

    fn get_map_coord( &self, location: usize ) -> Option<MapCoord> {
        let row: usize = location / self.cols();
        let col: usize = location % self.cols();

        if self.rows() <= &row {
            None
        } else {
            Some( MapCoord { row: (row), col: (col) } )
        }
    }

    fn get_location( &self, location: MapCoord ) -> Option<usize> {
        let too_big: usize = self.rows() * self.cols();
        let position: usize = (location.row * self.cols()) + location.col;
        if position >= too_big {
            None
        } else {
            Some( position )
        }
    }
}

struct Dilation{
    left_right: usize,
    top_bottom: usize
}

impl Dilation {
    pub fn new( left_right: usize, top_bottom: usize ) -> Dilation {
        Dilation{ left_right: ( left_right ), 
                  top_bottom: ( top_bottom )}
    }
}

struct StarField {
   field : Vec<u32>,
   cols: usize,
   rows: usize
}

#[derive(Debug)]
enum StarFieldError {
    OutOfBounds
}

impl fmt::Display for StarFieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StarFieldError::OutOfBounds => write!(f, "Point out of bounds."),
        }
    }
}

impl std::error::Error for StarFieldError {}

impl fmt::Display for StarField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.field.chunks(self.cols) {
            for element in row {
                write!(f, "{}", element)?;
            }
            write!(f, "{}", "\n")?;
        }
        Ok(())
    }
}

impl Mappable for StarField {
   fn cols( &self ) -> &usize {
       &self.cols
   }

   fn rows( &self ) -> &usize {
       &self.rows
   }
}

impl StarField {
    const GALAXY_VALUE: u32 = 0u32;
    const SPACE_VALUE: u32 = 1u32;

    /// Takes a string representation of a star field and creates a StarField
    pub fn parse( input: String ) -> StarField {
        let row_count: usize = input.chars()
                                    .filter(|c| *c == '\n')
                                    .count() + 1; // assumes no line-return at end of input
        let col_count: usize = input.find('\n')
                                    .unwrap();
        let field : Vec<u32> = input.chars()
                                     .filter(|c| *c != '\n')
                                     .map(|c| match c {
                                        '#' => Self::GALAXY_VALUE,
                                        '.' => Self::SPACE_VALUE,
                                        _ => unreachable!(),
                                     })
                                     .collect(); // we don't want the line returns

        Self::apply_distortion( StarField{ field: ( field ), cols: ( col_count ), rows: ( row_count ) } )
    }

    /// Returns the relative distance between two coordinates on the StarField
    /// 
    /// Will return an Error if one of the points is outside of the StarField
    pub fn distance_between( &self, point_one: MapCoord, point_two: MapCoord ) -> Result<u64, StarFieldError> {
        if !self.is_in(point_one) || !self.is_in(point_two) {
            return Err(StarFieldError::OutOfBounds)
        }

        let distance_row = usize::abs_diff(point_one.row, point_two.row);
        let distance_col = usize::abs_diff(point_one.col, point_two.col);
        Ok((distance_row + distance_col) as u64)
    }

    /// Returns an iterator that iterates over all galaxies in the StarField
    pub fn galaxies<'a>( &'a self ) -> impl Iterator<Item = MapCoord> + 'a {
        self.field.iter()
                  .enumerate()
                  .filter(|(_i, c)| **c == Self::GALAXY_VALUE )
                  .filter_map(|(i, _c)| self.get_map_coord(i))
    }

    /// Checks to see if a MapCoord is within this StarField
    fn is_in( &self, coord: MapCoord ) -> bool {
        coord.row < self.rows && coord.col < self.cols
    }

    /// Accounts for the gravitational distortion and adds an extra row and column to 
    /// rows and columns with no galaxies
    fn apply_distortion( input: StarField ) -> StarField {
        // iterate through all of the rows and mark the empty ones
        let mut empty_rows: Vec<usize> = Vec::new();
        for (row_idx, row) in input.field.chunks(input.cols).enumerate() {
            let is_empty: bool = row.iter().all(|c| *c == Self::SPACE_VALUE );
            if is_empty {
                empty_rows.push(row_idx);
            }
        }

        // iterate through all of the columns
        let mut empty_columns: Vec<usize> = Vec::new();
        for col in 0..input.cols {
            // produce a range that is just the glyphs in the column in field
            // and check it for the absence of galaxies
            let is_empty: bool = input.iter_for_column(col)
                                      .all(|c| *c == Self::SPACE_VALUE);
            if is_empty {
                empty_columns.push(col);
            }
        }

        // set all of the cells in empty rows or columns to 2
        let mut extended_field = input.field.clone();
        for ( row_idx, row) in extended_field.chunks_mut(input.cols).enumerate() {
            for idx in 0..input.cols {
                if empty_rows.contains(&row_idx) {
                    row[idx] = 2;
                }
                else if empty_columns.contains(&idx) {
                    row[idx] = 2;
                }
            }
            
        }

        StarField{ field: extended_field, 
                   cols: (input.cols), 
                   rows: (input.rows) }

    }

    fn iter_for_column( &self, col: usize ) -> impl Iterator<Item = &u32> {
        self.field.iter()
                  .enumerate()
                  .filter_map(move |(location, c)| {
                    if let Some(mapped_loc) = Mappable::get_map_coord(self, location) {
                        if mapped_loc.col == col {
                            return Some(c);
                        }
                    }
                    None
                  })
    }
}

fn main() {
    // get input
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];

    // read
    let space_raw: String = fs::read_to_string(file_path).unwrap();
    println!("{space_raw}");
    println!("-----------");

    let stars: StarField = StarField::parse( space_raw );
    println!("{stars}");

    // TODO split distance field and char field
    // WANT function that returns pairs of items

    let galaxies: Vec<MapCoord> = stars.galaxies().collect();
    let mut total_distance: u64 = 0u64;
    for outer_galaxy_idx in 0..galaxies.len() {
        for inner_galaxy_idx in (outer_galaxy_idx + 1)..galaxies.len() {
            total_distance += stars.distance_between(galaxies[outer_galaxy_idx], galaxies[inner_galaxy_idx]).unwrap();
        }
    }

    println!("The total distance between all galaxies part 1 is [{total_distance}]"); // 374 : stars_1
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestMap {
        rows: usize,
        cols: usize,
    }

    impl Mappable for TestMap {
        fn rows(&self) -> &usize {
            &self.rows
        }

        fn cols(&self) -> &usize {
            &self.cols
        }
    }

    #[test]
    fn test_get_map_coord() {
        let map = TestMap { rows: 10, cols: 10 };
        assert_eq!(map.get_map_coord(0), Some(MapCoord { row: 0, col: 0 }));
        assert_eq!(map.get_map_coord(5), Some(MapCoord { row: 0, col: 5 }));
        assert_eq!(map.get_map_coord(15), Some(MapCoord { row: 1, col: 5 }));
        assert_eq!(map.get_map_coord(99), Some(MapCoord { row: 9, col: 9 }));
        assert_eq!(map.get_map_coord(105), None);
    }

    #[test]
    fn test_get_location() {
        let map = TestMap { rows: 10, cols: 10 };
        assert_eq!(map.get_location(MapCoord { row: 0, col: 5 }), Some(5));
        assert_eq!(map.get_location(MapCoord { row: 1, col: 5 }), Some(15));
        assert_eq!(map.get_location(MapCoord { row: 10, col: 10 }), None);
    }
}