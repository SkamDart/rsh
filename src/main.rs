mod shell;
mod job;

fn main() {
    loop {
        shell::print_prompt();
        let user_input = shell::wait_for_command();
        shell::handle(user_input);
    }
}
