struct UrlComponents {
    username: Option<String>,
    password: Option<String>,
}

impl UrlComponents {
    fn to_url(&self) -> String {
        match (&self.username, &self.password) {
            (Some(usr), Some(pwd)) => format!("{usr}:{pwd}@"),
            (Some(usr), None) => format!("{usr}@"),
            _ => String::new(),
        }
    }
}
fn main() {
    let user_and_password = UrlComponents {
        username: Some(String::from("auca")),
        password: Some(String::from("badpassword")),
    };
    let only_user = UrlComponents {
        username: Some(String::from("auca")),
        password: None,
    };

    let result1 = user_and_password.to_url();
    let result2 = only_user.to_url();

    println!("user and password\n {result1}\nuser but no password\n {result2}");
}
