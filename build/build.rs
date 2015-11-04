use std::process::Command;

fn main() {
    let rbenv = Command::new("rbenv")
                        .arg("prefix")
                        .output()
                        .unwrap_or_else(|e| panic!("Rbenv not found: {}", e));

    let ruby_path =  String::from_utf8_lossy(&rbenv.stdout);

    println!("cargo:rustc-flags=-L {}/lib", ruby_path.trim());
}
