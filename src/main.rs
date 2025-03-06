use std::thread::spawn;

fn main() {
    // let rows = 20_000;
    // let cols = 20_000;

    // let large_matrix = generate_large_matrix(rows, cols);

    // println!("{:?}", spiral_traversal(&vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12], vec![13, 14, 15, 16]]));
    // println!("{:?}", spiral_traversal(&large_matrix));
    // spiral_traversal(&large_matrix);
    // spiral_traversal(&large_matrix);
    // spiral_traversal(&large_matrix);
    spawn_thread();
}

fn spawn_thread() {
    let thread_fn = || {
        let rows = 20_000;
        let cols = 20_000;

        let large_matrix = generate_large_matrix(rows, cols);

        spiral_traversal(&large_matrix);
    };

    let handle = spawn(thread_fn);
    let handle2 = spawn(thread_fn);

    // test_thread();

    handle.join().expect("The thread being joined has panicked");
    handle2.join().expect("The thread being joined has panicked");
}

fn generate_large_matrix(rows: usize, cols: usize) -> Vec<Vec<usize>> {
    let mut matrix: Vec<Vec<usize>> = Vec::with_capacity(rows * cols);
    let mut count = 1;

    for _ in 0..rows {
        let mut row = Vec::with_capacity(cols);
        for _ in 0..cols {
            row.push(count);
            count += 1;
        }
        matrix.push(row);
    }

    matrix
}

/// O(m * n) time complexity
/// O(m * n) space complexity
// fn spiral_traversal(matrix: &Vec<Vec<usize>>) -> Vec<usize> {
//     let m = matrix.len();
//     let n = matrix[0].len();

//     // result vector to store the spiral traversal
//     let mut res: Vec<usize> = vec![];

//     // initialize a 2D vector to keep track of visited cells
//     let mut visited = vec![vec![false; n]; m];

//     // use direction arrays dr and dc to represent right, down, left, and up movements
//     // move right (0, +1), move down (+1, 0)
//     // move left (0, -1), move up (-1, 0)

//     // change in the row index for each direction
//     let dr: [i32; 4] = [0, 1, 0, -1];
//     // change in the column index for each direction
//     let dc: [i32; 4] = [1, 0, -1, 0];

//     // inital positions in the matrix
//     let mut r: usize = 0;
//     let mut c: usize = 0;

//     // initialize direction index (0 = right)
//     let mut di: usize = 0;

//     // iterate over all cells in the matrix
//     for _ in 0..m * n {
//         res.push(matrix[r][c]);
//         visited[r][c] = true;

//         // calculate the next cell based on current direction
//         let next_r = r as i32 + dr[di];
//         let next_c = c as i32 + dc[di];

//         // check if the next position is within the matrix and not visited
//         if next_r >= 0 && next_r < m as i32 && next_c >= 0 && next_c < n as i32 && !visited[next_r as usize][next_c as usize] {
//             r = next_r as usize;
//             c = next_c as usize;
//         } else {
//             // change direction if the next cell is out of bounds or visited
//             di = (di + 1) % 4;
//             r = (r as i32 + dr[di]) as usize;
//             c = (c as i32 + dc[di]) as usize;
//         }
//     }

//     res
// }

/// boundary traversal
/// O(m * n) time complexity
/// O(1) space complexity
fn spiral_traversal(matrix: &Vec<Vec<usize>>) -> Vec<usize> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut res = Vec::with_capacity(rows * cols);

    // initialize boundaries
    let mut top = 0;
    let mut bottom = rows - 1;
    let mut left = 0;
    let mut right = cols - 1;

    // iterate over all cells in the matrix
    while top <= bottom && left <= right {
        // print right column from top to bottom
        for i in left..=right {
            res.push(matrix[top][i]);
        }
        top += 1;

        // print right column from top to bottom
        for i in top..=bottom {
            res.push(matrix[i][right]);
        }
        right -= 1;

        // print bottom row from right to left (if exists)
        if top <= bottom {
            for i in (left..=right).rev() {
                res.push(matrix[bottom][i]);
            }
            bottom -= 1;
        }

        // print left column from bottom to top (if exists)
        if left <= right {
            for i in (top..=bottom).rev() {
                res.push(matrix[i][left]);
            }
            left += 1;
        }
    }

    res
}
