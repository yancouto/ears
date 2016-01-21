#[cfg(unix)]
fn main() {
    println!("cargo:rustc-link-search=native=/usr/local/opt/openal-soft/lib");
    println!("cargo:rustc-link-search=native=/usr/local/lib");
}

#[cfg(not(unix))]
fn main() {}
