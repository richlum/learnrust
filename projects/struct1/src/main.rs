#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.height
    }
}


/// alternative method to allow display of struct
/// implement Display method instead of adding Debug trait
/// # example
///
/// ```
/// impl fmt::Display for User {
///    #[inline]
///    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
///  //      fmt::Display::fmt(&**self,f)
///        writeln!(f,"email: {}",self.email)?;
///        writeln!(f,"name: {}",self.username)?;
///        writeln!(f,"active: {}",self.active)?;
///        writeln!(f,"count: {} ",self.sign_in_count)
///    }
/// }
/// ```
///*/
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    println!("Hello, world!");
    let user2 = build_user(String::from("jametkirk@mailinator.com"), String::from("James T Kirk"));
    println!("user: {:?}",user2);
    dbg!(&user2);

    let rect1= Rectangle{
        width: 30,
        height: 50,
    };

    println!("area of the rectangle is {} ",
             rect1.area()
             );

}
