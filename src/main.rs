use std::env;
use std::process::Command;

fn run_command(command: &str, args: Vec<&str>) {
    let status = Command::new(command)
        .args(args)
        .status()
        .unwrap_or_else(|e| panic!("Failed to execute {}: {}", command, e));

    if !status.success() {
        eprintln!("Command {} failed", command);
    }
}

fn main() {
    let oh_my_zsh_path = env::var("ZSH").expect("The $ZSH environment variable is not set.");
    let upgrade_script_path = format!("{}/tools/upgrade.sh", oh_my_zsh_path);
    run_command("zsh", vec!["-c", &upgrade_script_path]);

    let brew_commands = vec![
        vec!["update"],
        vec!["upgrade"],
        vec!["upgrade", "--cask"],
        vec!["cleanup"],
        vec!["autoremove"],
    ];

    for args in brew_commands {
        run_command("brew", args);
    }

    let nvm_dir = env::var("NVM_DIR").expect("The NVM_DIR environment variable is not set.");
    let nvm_command = format!("source {}/nvm.sh; nvm install node --reinstall-packages-from=node", nvm_dir);
    run_command("zsh", vec!["-c", &nvm_command]);

    run_command("ncu", vec!["-g"]);
}

