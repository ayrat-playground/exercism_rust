pub fn raindrops(n: usize) -> String {
    let mut result = "".to_string();
    let pling = pling(n);
    let plang = plang(n);
    let plong = plong(n);

    result.push_str(&pling);
    result.push_str(&plang);
    result.push_str(&plong);

    if result == "".to_string() {
        n.to_string()
    } else {
        result
    }
}

fn pling(n: usize) -> String {
    matcher(n, 3, "Pling")
}

fn plang(n: usize) -> String {
    matcher(n, 5, "Plang")
}

fn plong(n: usize) -> String {
    matcher(n, 7, "Plong")
}

fn matcher(n: usize, divider: usize, res: &str) -> String {
    match n % divider {
        0 => res.to_string(),
        _ => "".to_string()
    }
}
