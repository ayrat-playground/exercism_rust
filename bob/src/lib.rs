pub fn reply(message: &str) -> &str {
    let message = message.trim();

    if message == "" {
        return "Fine. Be that way!"
    }

    if any_alphanumeric(message) {
        if all_uppercase(message) {
            if message.chars().last() == Some('?') {
              return "Calm down, I know what I'm doing!"
            } else {
                return "Whoa, chill out!"
            }
        }
    }

    match message.chars().last() {
        Some('?') => "Sure.",
        Some(_)   => "Whatever.",
        None => "Fine. Be that way!"
    }
}

fn any_alphanumeric(message: &str) -> bool {
    message.chars().any(
        |char| char.is_alphabetic()
    )
}

fn all_uppercase(message: &str) -> bool {
    message.chars().all(
        |char| (!char.is_alphabetic()) ||
            (char.is_uppercase() && char.is_alphabetic())
    )
}
