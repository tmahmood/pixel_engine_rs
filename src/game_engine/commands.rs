pub trait Commands {
    fn run_command(cmd_str: &str) -> bool;
}
