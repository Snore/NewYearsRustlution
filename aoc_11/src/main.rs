use std::fs;
use std::env;
use std::fmt;

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
        for col in 0..field.cols {
            // produce a range that is just the glyphs in the column in field
            // and check it for the absence of galaxies
            let is_empty: bool = field.field.iter()
                                            .enumerate()
                                            .filter_map(|(i, c)| {
                                                if i % col == 0 {
                                                    Some(c)
                                                } else {
                                                    None
                                                }
                                            } )
                                            .all(|c| *c == '.');

            // TODO: return a list of all columns that are empty
        }
        field
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
