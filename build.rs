use std::path::Path;
use std::process::Command;

fn main() {
    cargo_build_config();
    install_node_deps_if_necessary();
    compile_tailwindcss();
}

fn install_node_deps_if_necessary() {
    if !Path::new("node_modules/")
        .try_exists()
        .expect("Can't check for existence of node_modules")
    {
        let status = Command::new("npm")
            .arg("install")
            .status()
            .expect("failed to exectute npm install");

        assert!(status.success());
    }
}

fn compile_tailwindcss() {
    let status = Command::new("npx")
        .arg("tailwindcss")
        .args(["-i", "tailwind_input.css"])
        .args(["-o", "assets/css/style.css"])
        .status()
        .expect("failed to execute npx tailwindcss");

    assert!(status.success());
}

fn cargo_build_config() {
    println!("cargo:rerun-if-changed=tailwind_input.css");
    println!("cargo:rerun-if-changed=package.json");
}
