use std::process::Command;

#[test]
fn test_hello_world_binary() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "hello_world"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Hello, world!"));
}

#[test]
fn test_main_binary() {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "just-rust"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Hello, world!"));
}
