#[cfg(unix)]
fn main() {
    println!("cargo:rustc-link-search=native=/usr/local/opt/openal-soft/lib");
    println!("cargo:rustc-link-search=native=/usr/local/lib");
}

#[cfg(all(windows, target_arch = "x86"))]
fn main () {
    println!("cargo:rustc-link-search=native=C:\\msys32\\mingw64\\lib");
}

#[cfg(all(windows, target_arch = "x86_64"))]
fn main () {
    println!("cargo:rustc-link-search=native=C:\\msys64\\mingw64\\lib");
}

#[cfg(all(not(windows), not(unix)))]
fn main() {}
