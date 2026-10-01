//! Hand the module-definition file to the MSVC linker so exports land on the
//! exact ordinals the original `grap64.dll` used. Ordinal #9 in particular must
//! match or `MapleStory.exe` will fail to bind its static import.

fn main() {
    let def = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("grap64.def");
    println!("cargo:rerun-if-changed={}", def.display());
    println!("cargo:rustc-cdylib-link-arg=/DEF:{}", def.display());
    // A reproducible link, because this DLL is compiled INTO the launcher (`crates/launcher/
    // build.rs` `embed_stub`) and the launcher updates itself whenever its bytes differ from
    // the server's. With the link time in the PE header, relinking this for any reason made a
    // new launcher for every player. `/Brepro` puts a content hash there instead (2026-09-25).
    println!("cargo:rustc-cdylib-link-arg=/Brepro");
}
