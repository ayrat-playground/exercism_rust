#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient
}

pub fn classify(num: u64) -> Result<Classification, & 'static str> {
    if num < 1 {
      return Err("Number must be positive")
    }

    let factors = factors(num);
    let factors_sum: u64 = factors.iter().sum();

    if (factors_sum == num) && factors.len() == 1 {
        return Ok(Classification::Deficient)
    }

    if factors_sum == num {
        Ok(Classification::Perfect)
    } else if factors_sum > num {
        Ok(Classification::Abundant)
    } else if factors_sum < num {
        Ok(Classification::Deficient)
    } else {
        Err("oops")
    }
}

fn factors(num: u64) -> Vec<u64> {
    let half = num / 2 + 1;

    (1..half)
        .filter(|f| num % f == 0)
        .collect()
}
