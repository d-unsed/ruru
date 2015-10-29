fn main() {
    // Rbenv-specific path
    println!("cargo:rustc-link-search=native=~/.rbenv/versions/2.2.3/lib");
}
