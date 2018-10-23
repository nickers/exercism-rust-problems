pub fn reply(message: &str) -> &str {
// 'Sure.' if you ask him a question.
// 'Whoa, chill out!' if you yell at him.
// 'Calm down, I know what I'm doing!' if you yell a question at him.
    let q : String = message.trim().to_string();
    if q.is_empty() {
        return "Fine. Be that way!"
    }

    let yelling = q.to_ascii_uppercase() == q && q.chars().any(|x| x.is_alphabetic());
    let question = q.ends_with("?");

    if question && yelling {
        return "Calm down, I know what I'm doing!"
    }
    if question {
        return "Sure."
    }
    if yelling {
        return "Whoa, chill out!"
    }
    return "Whatever."
}
