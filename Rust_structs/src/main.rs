struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


fn main() {
    let mut my_user: User = User {
        active: true,
        username: String::from("shvmpz"),
        email: String::from("shvmpz@gmail.com"),
        sign_in_count: 2
    };

    let user_email = my_user.email;

    println!("email {user_email}");

    my_user.email = String::from("newemail@gmail.com");

    let updated_email = my_user.email;



    println!("new email = {updated_email}");

    let user2 = short_hand(String::from("martin"), String::from("martin@gmail.com"));


    //let user2_email = user2.email;

    // println!("user 2 email: {user2_email}");

    let user3 = User {
        username: String::from("Mkuu"),
        ..user2
    };

    let user3_email = user3.email;

    println!("User3 email: {user3_email}");



}

fn short_hand(username: String, email: String) -> User {

    User {
        active: true,
        username,
        email,
        sign_in_count: 1
    }

}
