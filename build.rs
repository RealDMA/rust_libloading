#[cfg(target_os = "windows")]
fn main(){
    println!("cargo::rerun-if-changed=src/os/windows/MemoryModule.c");
    cc::Build::new()
        .file("src/os/windows/MemoryModule.c")
        .compile("MemoryModule");
}

#[cfg(not(target_os = "windows"))]
fn main() {
}