pub fn square_of_sum(n: usize) -> usize {
    (1..=n).sum::<usize>().pow(2)
}

pub fn sum_of_squares(n: usize) -> usize {
    (1..=n).map(|_n| _n.pow(2)).sum()
}

pub fn difference(n: usize) -> usize {
    square_of_sum(n) - sum_of_squares(n)
}
