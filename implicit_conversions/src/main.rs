// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut ret_matrix: [[i32; 3]; 3] = [[0; 3]; 3];
    for row_ind in 0..matrix.len()
    {
        for col_ind in 0..matrix[0].len()
        {
            // let ref_x: &mut i32 = &mut ret_matrix[col_ind][row_ind];
            // *ref_x = matrix[row_ind][col_ind];
            ret_matrix[col_ind][row_ind] = matrix[row_ind][col_ind];
        }
    }
    return ret_matrix;
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for row in matrix{
        println!("| {row:?} |")
    }
}

fn main() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);
}