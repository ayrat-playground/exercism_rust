pub fn map_function<F: Fn(i32) -> i32>(input: Vec<i32>, f: F) -> Vec<i32> {
    let mut output = Vec::new();

    for el in input {
        output.push(f(el));
    }

    output
}

pub fn map_closure<F: Fn(i32) -> i32>(input: Vec<i32>, f: F) -> Vec<i32> {
    map_function(input, f)
}
