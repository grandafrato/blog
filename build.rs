use std::process::Command;

fn main() {
    let output = Command::new("npx")
        .args([
            "tailwindcss",
            "-i",
            "tailwind_input.css",
            "-o",
            "assets/css/style.css",
        ])
        .output()
        .expect("failed to execute process");
    println!("{:?}", output);
}
