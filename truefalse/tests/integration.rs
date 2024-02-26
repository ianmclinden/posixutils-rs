use std::process::Command;

#[test]
fn test_false_exit_code() {
    let workspace_target = std::env::current_dir()
        .unwrap()
        .parent()
        .unwrap() // Move up to the workspace root from the current package directory
        .join("target/debug/false"); // Adjust the path to the binary

    let output = Command::new(workspace_target)
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    assert_eq!(output.status.code(), Some(1));
}

#[test]
fn test_true_exit_code() {
    let workspace_target = std::env::current_dir()
        .unwrap()
        .parent()
        .unwrap() // Move up to the workspace root from the current package directory
        .join("target/debug/true"); // Adjust the path to the binary

    let output = Command::new(workspace_target)
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    assert_eq!(output.status.code(), Some(0));
}