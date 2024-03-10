fn main() {
    let user_agent: &'static str = real_ua::get_ua();
    println!("{}", user_agent);
}
