use anyhow::Result;
use std::fs;
use std::process::Command;
use tempfile::tempdir;

fn run_cli_command(args: &[&str]) -> Result<String> {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .args(args)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Command failed: {}", stderr);
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[test]
fn test_signature_verification() -> Result<()> {
    // Create a temporary directory for test files
    let temp_dir = tempdir()?;
    let temp_path = temp_dir.path();

    // Generate a test key pair
    let key_name = "test_key";
    run_cli_command(&["generate-key", "--name", key_name])?;

    // Create test files
    let proof_content = "test proof content";
    let elf_content = "test elf content";

    let proof_path = temp_path.join("test.proof");
    let elf_path = temp_path.join("test.elf");

    fs::write(&proof_path, proof_content)?;
    fs::write(&elf_path, elf_content)?;

    // Send the files to the server
    let output = run_cli_command(&[
        "send",
        "--proof-file",
        proof_path.to_str().unwrap(),
        "--elf-file",
        elf_path.to_str().unwrap(),
        "--key-name",
        key_name,
    ])?;

    // Verify the response contains success message
    assert!(
        output.contains("Successfully sent files"),
        "Expected success message, got: {}",
        output
    );

    Ok(())
}
