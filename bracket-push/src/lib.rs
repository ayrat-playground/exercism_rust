pub struct Brackets<'a> {
    data: &'a str
}

impl<'a> From<&'a str> for Brackets<'a> {
    fn from(string: &'a str) -> Brackets<'a> {
        Brackets { data: string }
    }
}

impl<'a> Brackets<'a> {
    pub fn are_balanced(&self) -> bool {
        let mut stack = Vec::<char>::new();

        for ch in self.data.chars() {
            match ch {
                '(' | '[' | '{' => stack.push(ch),
                ')' | ']' | '}' => if !matches_bracket(&mut stack, ch) { return false },
                _               => ()
            }
        }

        stack.is_empty()
    }
}

fn matches_bracket(stack: &mut Vec<char>, closing_bracket: char) -> bool {
    let opening_bracket = match closing_bracket {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        _   => 'e'
    };

    let prev_bracket = match stack.pop() {
        Some(bracket) => bracket,
        None          => ' '
    };

    prev_bracket == opening_bracket
}
