use regex::Regex;

pub fn is_valid_user_name(user_name: &str) -> bool {
    let re = Regex::new(r".{2,}").unwrap(); // Minimum length of 2 characters
    re.is_match(user_name)
}

pub fn is_valid_mail_addr(mail_addr: &str) -> bool {
    let re = Regex::new(r"^([a-zA-Z0-9_\-\.]+)@([a-zA-Z0-9_\-\.]+)\.([a-zA-Z]{2,5})$").unwrap();
    re.is_match(mail_addr)
}

pub fn is_valid_pass_word(pass_word: &str) -> bool {
    let re_1 = Regex::new(r".{8,}").unwrap(); // Minimum length of 8 characters
    let re_2 = Regex::new(r"[A-Z]").unwrap(); // At least one uppercase letter
    let re_3 = Regex::new(r"[a-z]").unwrap(); // At least one lowercase letter
    let re_4 = Regex::new(r"\d").unwrap(); //    At least one digit
    let re_5 = Regex::new(r"[$&+,:;=?@#|'<>.^*()%!-]").unwrap(); // At least one special character

    re_1.is_match(pass_word)
        && re_2.is_match(pass_word)
        && re_3.is_match(pass_word)
        && re_4.is_match(pass_word)
        && re_5.is_match(pass_word)
}
