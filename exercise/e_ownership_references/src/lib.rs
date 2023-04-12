pub fn is_plural(s: &String) -> bool {
    if s.ends_with("s") {
        return true;
    }
    return false;
}

pub fn change(s: &mut String) {
    if !is_plural(s) {
        s.push_str("s");
    } else {
        println!("Already plural");
    }
}

pub fn eat(s: String) -> bool {
    if s.starts_with("b") && s.contains("a") {
        return true;
    }
    return false;
}

pub fn bedazzle(s: &mut String) {
    *s = String::from("sparkly");
}
