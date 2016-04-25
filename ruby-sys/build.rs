use std::process::Command;

fn main() {
    let ruby = Command::new("ruby")
                   .arg("-e")
                   .arg("puts File.join(File.dirname(File.dirname(RbConfig.ruby)), 'lib')")
                   .output()
                   .unwrap_or_else(|e| panic!("ruby not found: {}", e));

    let ruby_libdir = String::from_utf8_lossy(&ruby.stdout);

    println!("cargo:rustc-link-search={}", ruby_libdir.trim());
}
