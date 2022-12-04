use std::process::Command;
fn terminalInterface(mainArg: String, subArg: String) {
    Command::new("ls")
        .args([mainArg, subArg])
        .spawn()
        .expect("Failed to execute terminalInterface process");
}
