mod cmd_arg;
fn main() {
    let cmd = cmd_arg::cmd_str();
    let data = cmd_arg::init();
    println!("{}:\n{}", cmd, data);
}
