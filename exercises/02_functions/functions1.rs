const LOGGER_NAME: &str = file!();

fn call_me() -> String
{
    let msg: &str = "Do the thing";
    println!("{LOGGER_NAME}: {msg}");
    msg.to_string()
}


fn main() {
    call_me(); // Don't change this line
}
