use crate::*;

pub fn read_file(file: &str) -> String {
    fs::read_to_string(file).unwrap()
}

pub fn test_adjacent<T>(
    array: &[Vec<T>],
    row: usize,
    col: usize,
    test: fn(&T) -> bool,
    default: bool,
) -> usize {
    [
        if row == 0 || col == 0 {
            default
        } else {
            test(&array[row - 1][col - 1])
        },
        if row == 0 {
            default
        } else {
            test(&array[row - 1][col])
        },
        if row == 0 || col == array[0].len() - 1 {
            default
        } else {
            test(&array[row - 1][col + 1])
        },
        if col == 0 {
            default
        } else {
            test(&array[row][col - 1])
        },
        if col == array[0].len() - 1 {
            default
        } else {
            test(&array[row][col + 1])
        },
        if row == array.len() - 1 || col == 0 {
            default
        } else {
            test(&array[row + 1][col - 1])
        },
        if row == array.len() - 1 {
            default
        } else {
            test(&array[row + 1][col])
        },
        if row == array.len() - 1 || col == array[0].len() - 1 {
            default
        } else {
            test(&array[row + 1][col + 1])
        },
    ]
    .into_iter()
    .filter(|x| *x)
    .count()
}
