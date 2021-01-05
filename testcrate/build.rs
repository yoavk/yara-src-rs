
fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    yara_src::build();
    yara_src::print_cargo_metadata();
}
