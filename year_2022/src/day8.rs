use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use strum_macros::EnumIter;

pub struct Day8;

#[derive(EnumIter, Hash, Eq, PartialEq)]
enum ViewDirection {
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
}

impl ViewDirection {
    fn iterator(&self, left: usize, right: usize) -> Box<dyn Iterator<Item = usize>> {
        match *self {
            Self::LeftToRight | Self::TopToBottom => Box::new(left..right),
            Self::RightToLeft | Self::BottomToTop => Box::new((left..right).rev()),
        }
    }
}

impl Day8 {
    fn read_input(path: &str) -> Vec<Vec<i8>> {
        let file = File::open(path).unwrap();
        let buffer_reader = BufReader::new(file);

        buffer_reader
            .lines()
            .into_iter()
            .map(|line| line.unwrap().chars().map(|c| c as i8 - '0' as i8).collect())
            .collect()
    }

    fn process_rows(matrix: &Vec<Vec<i8>>, processed_trees: &mut HashSet<(usize, usize)>) {
        for column_direction in [ViewDirection::LeftToRight, ViewDirection::RightToLeft] {
            for row in ViewDirection::TopToBottom.iterator(0, matrix.len()) {
                let mut max_height = -1;

                for column in column_direction.iterator(0, matrix[0].len()) {
                    if processed_trees.contains(&(row, column)) {
                        max_height = max_height.max(matrix[row][column]);
                        continue;
                    }

                    if matrix[row][column] > max_height {
                        max_height = matrix[row][column];
                        processed_trees.insert((row, column));
                    }
                }
            }
        }
    }

    fn process_columns(matrix: &Vec<Vec<i8>>, processed_trees: &mut HashSet<(usize, usize)>) {
        for row_direction in [ViewDirection::TopToBottom, ViewDirection::BottomToTop] {
            for column in ViewDirection::LeftToRight.iterator(0, matrix[0].len()) {
                let mut max_height = -1;

                for row in row_direction.iterator(0, matrix.len()) {
                    if processed_trees.contains(&(row, column)) {
                        max_height = max_height.max(matrix[row][column]);
                        continue;
                    }

                    if matrix[row][column] > max_height {
                        max_height = matrix[row][column];
                        processed_trees.insert((row, column));
                    }
                }
            }
        }
    }

    pub fn part_one() -> usize {
        let matrix = Self::read_input("src/day8/input");
        let mut processed_trees = HashSet::new();

        Self::process_rows(&matrix, &mut processed_trees);
        Self::process_columns(&matrix, &mut processed_trees);

        processed_trees.len()
    }

    fn scores_left_to_right(matrix: &Vec<Vec<i8>>) -> Vec<Vec<usize>> {
        let mut result = vec![];

        (0..matrix.len()).for_each(|row| {
            let mut decrease_and_humerus_points = vec![];
            let mut view_scores_row = vec![0];

            (1..matrix[0].len()).for_each(|column| {
                if matrix[row][column] <= matrix[row][column - 1] {
                    decrease_and_humerus_points.push(column - 1);
                    view_scores_row.push(1);
                } else {
                    if let Some(higher_tree_pos) = decrease_and_humerus_points
                        .iter()
                        .rev()
                        .find(|pos| matrix[row][**pos] >= matrix[row][column])
                    {
                        view_scores_row.push(column - *higher_tree_pos);
                    } else {
                        view_scores_row.push(column);
                    }
                }
            });

            result.push(view_scores_row)
        });

        result
    }

    fn scores_right_to_left(matrix: &Vec<Vec<i8>>) -> Vec<Vec<usize>> {
        let mut result = vec![];

        (0..matrix.len()).for_each(|row| {
            let mut decrease_and_humerus_points = Vec::with_capacity(matrix[0].len());
            let mut view_scores_row = Vec::with_capacity(matrix[0].len());

            (0..matrix[0].len() - 1).rev().for_each(|column| {
                if matrix[row][column] <= matrix[row][column + 1] {
                    decrease_and_humerus_points.push(column + 1);
                    unsafe {
                        (view_scores_row.as_mut_ptr() as *mut usize)
                            .offset(column as isize)
                            .write(1);
                    }
                } else {
                    if let Some(higher_tree_pos) = decrease_and_humerus_points
                        .iter()
                        .rev()
                        .find(|pos| matrix[row][**pos] >= matrix[row][column])
                    {
                        unsafe {
                            (view_scores_row.as_mut_ptr() as *mut usize)
                                .offset(column as isize)
                                .write(*higher_tree_pos - column);
                        }
                    } else {
                        unsafe {
                            (view_scores_row.as_mut_ptr() as *mut usize)
                                .offset(column as isize)
                                .write(matrix[0].len() - 1 - column);
                        }
                    }
                }
            });

            // Unsafe adding elements will not increase length
            unsafe {
                view_scores_row.set_len(matrix[0].len() - 1);
            }

            view_scores_row.push(0);
            result.push(view_scores_row)
        });

        result
    }

    fn scores_top_to_bottom(matrix: &Vec<Vec<i8>>) -> Vec<Vec<usize>> {
        let mut result = vec![vec![0; matrix[0].len()]];
        let mut decrease_and_humerus_points = HashMap::new();

        (1..matrix.len()).for_each(|row| {
            let mut view_scores_row = vec![];

            (0..matrix[0].len()).for_each(|column| {
                if matrix[row - 1][column] >= matrix[row][column] {
                    decrease_and_humerus_points
                        .entry(column)
                        .or_insert_with(Vec::new)
                        .push(row - 1);
                    view_scores_row.push(1);
                } else {
                    if let Some(higher_tree_pos) = decrease_and_humerus_points
                        .get(&column)
                        .and_then(|column_decrease_and_humerus_points| {
                            column_decrease_and_humerus_points
                                .iter()
                                .rev()
                                .find(|pos| matrix[**pos][column] >= matrix[row][column])
                        })
                    {
                        view_scores_row.push(row - *higher_tree_pos);
                    } else {
                        view_scores_row.push(row);
                    }
                }
            });

            result.push(view_scores_row)
        });

        result
    }

    fn scores_bottom_to_top(matrix: &Vec<Vec<i8>>) -> Vec<Vec<usize>> {
        let mut result = Vec::with_capacity(matrix.len());
        let mut decrease_and_humerus_points = HashMap::new();

        (0..matrix.len() - 1).rev().for_each(|row| {
            let mut view_scores_row = vec![];

            (0..matrix[0].len()).for_each(|column| {
                if matrix[row + 1][column] >= matrix[row][column] {
                    decrease_and_humerus_points
                        .entry(column)
                        .or_insert_with(Vec::new)
                        .push(row + 1);
                    view_scores_row.push(1);
                } else {
                    if let Some(higher_tree_pos) = decrease_and_humerus_points
                        .get(&column)
                        .and_then(|column_decrease_and_humerus_points| {
                            column_decrease_and_humerus_points
                                .iter()
                                .rev()
                                .find(|pos| matrix[**pos][column] >= matrix[row][column])
                        })
                    {
                        view_scores_row.push(*higher_tree_pos - row);
                    } else {
                        view_scores_row.push(matrix.len() - 1 - row);
                    }
                }
            });

            unsafe {
                (result.as_mut_ptr() as *mut Vec<usize>)
                    .offset(row as isize)
                    .write(view_scores_row)
            }
        });

        unsafe {
            result.set_len(matrix.len() - 1);
        }

        result.push(vec![0; matrix[0].len()]);
        result
    }

    // fn score_bottom_to_top(matrix: &Vec<Vec<i8>>) -> Vec<Vec<usize>> {
    //     let mut result = vec![];
    //
    //     (0..matrix.len()).for_each(|row| {
    //         let mut result_row = vec![0];
    //
    //         (1..matrix[0].len()).for_each(|column| {
    //             if let Some(higher_tree_pos) =
    //                 (row + 1..matrix.len()).find(|pos| matrix[*pos][column] >= matrix[row][column])
    //             {
    //                 result_row.push(higher_tree_pos - row);
    //             } else {
    //                 result_row.push(matrix.len() - 1 - row);
    //             }
    //         });
    //
    //         result.push(result_row)
    //     });
    //
    //     result
    // }

    pub fn part_two() -> usize {
        let matrix = Self::read_input("src/day8/input");
        let view_scores = [
            Self::scores_left_to_right(&matrix),
            Self::scores_right_to_left(&matrix),
            Self::scores_top_to_bottom(&matrix),
            Self::scores_bottom_to_top(&matrix),
        ];
        let mut result = 0;

        (0..matrix.len()).for_each(|row| {
            (0..matrix[0].len()).for_each(|column| {
                result = result.max(
                    view_scores
                        .iter()
                        .fold(1, |acc, vec_scores| acc * vec_scores[row][column]),
                );
            });
        });

        result
    }
}
