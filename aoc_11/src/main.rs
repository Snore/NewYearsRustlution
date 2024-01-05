use std::fs;
use std::env;
use std::fmt;

/// A x,y coordinate on a 2D map
#[derive(Clone, Copy)]
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

        if self.rows() >= &row {
            None
        } else {
            Some( MapCoord { row: (row), col: (col) } )
        }
    }

    fn get_location( &self, location: MapCoord ) -> Option<usize> {
        let too_big: usize = self.rows() * self.cols();
        let position: usize = location.row * location.col;
        if position >= too_big {
            None
        } else {
            Some( position )
        }
    }
}

struct StarField {
   field : Vec<char>,
   cols: usize,
   rows: usize
}

impl fmt::Display for StarField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.field.chunks(self.cols) {
            for char in row {
                write!(f, "{}", char)?;
            }
            write!(f, "{}", "\n")?;
        }
        Ok(())
    }
}

impl StarField {
   const EMPTY_CHAR: char = '.';

   /// Takes a string representation of a star field and creates a StarField
    pub fn parse( input: String ) -> StarField {
        let row_count: usize = input.chars()
                                    .filter(|c| *c == '\n')
                                    .count() + 1; // assumes no line-return at end of input
        let col_count: usize = input.find('\n')
                                    .unwrap();
        let field : Vec<char> = input.chars()
                                     .filter(|c| *c != '\n')
                                     .collect(); // we don't want the line returns

        let compressed_field: StarField = StarField{ field: ( field ), 
                                                     cols: ( col_count ), 
                                                     rows: ( row_count ) };
        Self::expand( compressed_field )
    }

    /// Accounts for gravitational distortion and adds an extra row/col for every
    /// row and column that have no galaxies
    fn expand( field: StarField ) -> StarField {
        // first check the columns, then check the rows.
        // it does not really matter which order we do it in.

        // iterate through all of the columns
        let mut empty_columns: Vec<usize> = Vec::new();
        for col in 0..field.cols {
            // produce a range that is just the glyphs in the column in field
            // and check it for the absence of galaxies
            let is_empty: bool = field.iter_for_column(col)
                                      .all(|c| *c == Self::EMPTY_CHAR);
            if is_empty {
                empty_columns.push(col);
            }
        }

        // iterate through all of the rows and mark the empty ones
        let mut empty_rows: Vec<usize> = Vec::new();
        for (row_idx, row) in field.field.chunks(field.cols).enumerate() {
            let is_empty: bool = row.iter().all(|c| *c == Self::EMPTY_CHAR );
            if is_empty {
                empty_rows.push(row_idx);
            }
        }

        let expanded_cols: usize = field.cols + empty_columns.len();
        let expanded_rows: usize = field.rows + empty_rows.len();
        let mut expanded_field: StarField = StarField { field: (Vec::new()), 
                                                        cols: (expanded_cols), 
                                                        rows: (expanded_rows) };

        // inject the extra elements while repopulating the field
        for (row_idx, row) in field.field.chunks(field.cols).enumerate() {
            for (char_idx, char) in row.iter().enumerate() {
                // populate the expanded field
                expanded_field.field.push(*char);
                //
            }
        }

        expanded_field
    }

    fn iter_for_column( &self, col: usize ) -> impl Iterator<Item = &char> {
        self.field.iter()
                  .enumerate()
                  .filter_map(move |(i, c)| {
                      if i % col == 0 {
                          Some(c)
                      } else {
                          None
                      }
                  } )
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
}
