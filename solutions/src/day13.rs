use libs::read_input::InputData;

use nalgebra::SMatrix;
use regex::Regex;

// Simply to make types less cumbersome.
#[derive(Clone, Copy)]
struct Matrix {
    m: nalgebra::Matrix<f64, nalgebra::Const<2>, nalgebra::Const<3>, nalgebra::ArrayStorage<f64, 2, 3>>,
    rows: usize,
    cols: usize,
}

/*
    I am defining a system of equations as follows:

    1: X1a + Y1b = X         [ X1 Y1 | X ]
    2: X2a + Y2b = Y   ===>  [ X2 Y2 | Y ]

    Then solve using Gaussian elimination and backwards substitution.
*/

// Generating the matrices.
fn parse(input: InputData) -> Vec<Matrix> {
    let re: Regex = Regex::new(r"(?<num>\d{1,20})").unwrap(); // Regex for grabbing numbers. 1-20 digits because of the example, not needed.
    let mut i: usize = 0; // Starting index.
    let mut matrices: Vec<Matrix> = Vec::new(); // Vector of matrices.

    // Loop through the list of text inputted.
    while i < input.list.len() {
        // If we're at an empty line, skip to next.
        if input.list[i].len() == 0 {
            i += 1
        // If we reach the first line of a machine description, continue.
        } else if input.list[i].len() == 4 {
            // Grabbing each value with regex. Could definitely be tidied up.
            let c_1 = re.captures(&input.list[i][2]).unwrap();
            let x_1: f64 = c_1["num"].parse().unwrap();
            let c_2 = re.captures(&input.list[i+1][2]).unwrap();
            let x_2: f64 = c_2["num"].parse().unwrap();
            let c_3 = re.captures(&input.list[i+2][1]).unwrap();
            let x_3: f64 = c_3["num"].parse::<f64>().unwrap() + 10000000000000.0; // Remove 10 trillion for part 1.

            // Grab the second row.
            let c_1 = re.captures(&input.list[i][3]).unwrap();
            let y_1: f64 = c_1["num"].parse().unwrap();
            let c_2 = re.captures(&input.list[i+1][3]).unwrap();
            let y_2: f64 = c_2["num"].parse().unwrap();
            let c_3 = re.captures(&input.list[i+2][2]).unwrap();
            let y_3: f64 = c_3["num"].parse::<f64>().unwrap() + 10000000000000.0; // Remove 10 trillion for part 1.

            // Generate a new matrix and store it.
            matrices.push( Matrix{ m: (SMatrix::<f64, 2, 3>::new(
                x_1, x_2, x_3,
                y_1, y_2, y_3
                )), rows: 2, cols: 3
            });

            i += 3 // Skip to the next empty line.
        }
    }
    matrices // Vector of matrices.
}

// Argmax function just to tidy up reduction function. Gets the highest value in the specified collumn.
fn argmax(matrix: &Matrix, h: usize, k: usize) -> usize {
    let mut current: (f64, usize) = (0.0,0);

    for i in h..matrix.rows {
        if matrix.m[(i, k)].abs() >= current.0 {
            current = (matrix.m[(i, k)].abs(), i)
        }
    }

    current.1
}

// Gaussian elimination function. Reduces matrix to row echelon form.
// Based on pseudocode here https://en.wikipedia.org/wiki/Gaussian_elimination.
fn reduction(matrix: &mut Matrix) -> &mut Matrix {
    let mut h: usize = 0; // Pivot row.
    let mut k: usize = 0; // Pivot col.

    while h <= matrix.rows - 1 && k <= matrix.cols - 1 {
        // Find kth pivot.
        let i_max: usize = argmax(&matrix, h, k);
        
        if matrix.m[(i_max, k)] == 0.0 {
            // Skip to next collumn.
            k += 1
        } else {
            matrix.m.swap_rows(h, i_max);
            // Repeat below pivot.
            for i in (h+1)..matrix.rows {
                let f: f64 = matrix.m[(i, k)] / matrix.m[(h, k)];
                // Fill lower part with zeroes.
                matrix.m[(i, k)] = 0.0;

                // Repeat as needed.
                for j in (k+1)..matrix.cols {
                    matrix.m[(i, j)] = matrix.m[(i, j)] - matrix.m[(h, j)] * f;
                }
            }
            // Continue.
            h += 1;
            k += 1;
        }
    }

    //println!("After: \n{}", matrix.m);

    matrix
}

fn substitution(matrix: Matrix, original: Matrix) -> f64{
    // Grab b from the reduced matrix. Set to 0 if matrix isn't reduced.
    let mut b: f64 = if matrix.m[(1, 0)] == 0.0 {
        matrix.m[(1, 2)] / matrix.m[(1, 1)]
    } else {
        0.0
    };

    // Grab a from the matrix using b, again, zero if it isn't reduced.
    let mut a: f64 = if b > 0.0 {
        (matrix.m[(0, 2)] - (matrix.m[(0, 1)] * b)) / matrix.m[(0, 0)]
    } else {
        0.0
    };

    // Both these account for floating point accuracy problems. It's probably my fault they occur somehow, but this works.
    if b.fract() >= 0.5 {
        b = b.ceil()
    } else {
        b = b.floor()
    }

    if a.fract() >= 0.5 {
        a = a.ceil();
    } else {
        a = a.floor()
    }

    // Check if the values exist.
    if a > 0.0 {
        // Check a and b with the original matrix.
        let check_1: bool = original.m[(0, 2)] == (original.m[(0, 0)] * a) + (original.m[(0, 1)] * b);
        let check_2: bool = original.m[(1, 2)] == (original.m[(1, 0)] * a) + (original.m[(1, 1)] * b);

        // If the values are correct in the original matrix, then return the token cost.
        if check_1 && check_2 {
            return a * 3.0 + b
        // If not, then just return 0.
        } else {
            return 0.0
        }
    } else {
        0.0
    }
}

// Just repeats all processing for all matrices.
fn count_tokens(matrices: &mut Vec<Matrix>) -> f64 {
    let mut result: f64 = 0.0;
    let originals: Vec<Matrix> = matrices.clone();

    for i in 0..matrices.len() {
        reduction(&mut matrices[i]);
        let current = substitution(matrices[i], originals[i]);
        result += current;
    }

    println!("Total cost: {:?}", result);

    result
}

pub fn wrapper(input: InputData) {
    count_tokens(&mut parse(input));
}