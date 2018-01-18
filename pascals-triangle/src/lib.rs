pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let rows = (0..row_count)
            .map(|i|
                 (0..i+1)
                 .map(|j| element(i, j))
                 .collect())
            .collect();

        PascalsTriangle{ rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}

fn element(i: u32, j: u32) -> u32 {
    if (i == 0) || (j == 0) || (i == j) { return 1 }

    binomial_coef(i-1, j-1) + binomial_coef(i-1, j)
}

fn binomial_coef(i: u32, j: u32) -> u32 {
    factorial(i) / (factorial(j) * factorial(i-j))
}

fn factorial(n: u32) -> u32 {
    if (n == 0) || (n == 1) { return 1 }

    (1..n+1).fold(1, |acc, i| acc * i)
}
