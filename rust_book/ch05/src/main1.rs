fn main() {
    // &strではなく意図的に所有権のあるString型を使用する
    {
        let user = User {
            name: String::from("someusername1234"),
            email: String::from("someusername1234@example.com"),
            active: true,
            sign_in_count: 10,
        };
        println!("user name is: {}", user.name);
        println!("user email is: {}", user.email);
        println!("user active is: {}", user.active);
        println!("user sign_in_count is: {}", user.sign_in_count);
    }
    println!("----------------------");

    {
        let mut user = User {
            name: String::from("someusername1234"),
            email: String::from("someusername1234@example.com"),
            active: true,
            sign_in_count: 10,
        };
        user.name = String::from("someusername1234_renamed");
        println!("user name is: {}", user.name);
        println!("user email is: {}", user.email);
        println!("user active is: {}", user.active);
        println!("user sign_in_count is: {}", user.sign_in_count);
    }
    println!("----------------------");

    {
        let user = build_user(
            String::from("built-user@example.com"),
            String::from("build-user-name-1234"),
        );
        println!("user name is: {}", user.name);
        println!("user email is: {}", user.email);
        println!("user active is: {}", user.active);
        println!("user sign_in_count is: {}", user.sign_in_count);
    }
    println!("----------------------");

    {
        let user = build_user(
            String::from("built-user@example.com"),
            String::from("build-user-name-1234"),
        );
        let user2 = User {
            name: String::from("user2-name"),
            email: String::from("user2-email@example.com"),
            ..user
        };
        println!("user2 name is: {}", user2.name);
        println!("user2 email is: {}", user2.email);
        println!("user2 active is: {}", user2.active);
        println!("user2 sign_in_count is: {}", user2.sign_in_count);
    }
    println!("----------------------");
}

struct User {
    name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, name: String) -> User {
    User {
        email,
        name,
        active: true,
        sign_in_count: 1,
    }
}
