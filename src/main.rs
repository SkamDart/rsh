pub mod shell;

fn main() {
    loop {
        shell::print_prompt();
        let command = shell::wait_for_command();
        shell::execute(command);
    }
}
