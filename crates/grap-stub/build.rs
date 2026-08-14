//! Hand the module-definition file to the MSVC linker so exports land on the
//! exact ordinals the original `grap64.dll` used. Ordinal #9 in particular must
//! match or `MapleStory.exe` will fail to bind its static import.

fn main() {
    let def = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("grap64.def");
    println!("cargo:rerun-if-changed={}", def.display());
    println!("cargo:rustc-cdylib-link-arg=/DEF:{}", def.display());
}
