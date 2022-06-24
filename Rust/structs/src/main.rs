struct user {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> user {
    user {
        email: email, //If the name matches the struct field name, you can omit the field name. example "email,"
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let mut xanthus58 = user {
        username: "xanthus58".to_string(),
        email: "xanthus58@protonmail.com".to_string(),
        sign_in_count: 1,
        active: true,
    };
    let mut wolfen = build_user("wolfen@protonmail.com".to_string(), "Wolfen_XVII".to_string());

    let mut java = user {
        username: "Java".to_string(),
        email: "java@gmail.com".to_string(),
        ..wolfen //Assignes the rest values of the structs to match the value of the other struct(wolfen)
    };

    println!("Xanthus58's information");
    println!("----------------------------");
    println!("Username: {}", xanthus58.username);
    println!("Email: {}", xanthus58.email);
    println!("Sign in count: {}", xanthus58.sign_in_count);
    println!("Active: {}", xanthus58.active);
    println!("----------------------------");
    println!("Wolfen's information");
    println!("----------------------------");
    println!("Username: {}", wolfen.username);
    println!("Email: {}", wolfen.email);
    println!("Sign in count: {}", wolfen.sign_in_count);
    println!("Active: {}", wolfen.active);
    println!("----------------------------");
    println!("Java's information");
    println!("----------------------------");
    println!("Username: {}", java.username);
    println!("Email: {}", java.email);
    println!("Sign in count: {}", java.sign_in_count);
    println!("Active: {}", java.active);
}
