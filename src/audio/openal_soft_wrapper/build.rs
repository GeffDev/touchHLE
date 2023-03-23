/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
use std::path::Path;

/*fn rerun_if_changed(path: &Path) {
    println!("cargo:rerun-if-changed={}", path.to_str().unwrap());
}*/
fn link_search(path: &Path) {
    println!("cargo:rustc-link-search=native={}", path.to_str().unwrap());
}
fn link_lib(lib: &str) {
    // Default to static linking
    if cfg!(feature = "dynamic") {
        println!("cargo:rustc-link-lib=dylib={}", lib);
    } else {
        println!("cargo:rustc-link-lib=static={}", lib);
    }
}

fn link_framework(framework: &str) {
    println!("cargo:rustc-link-lib=framework={}", framework);
}

fn main() {
    let package_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = package_root.join("../../..");

    let mut build = cmake::Config::new(workspace_root.join("vendor/openal-soft"));
    if cfg!(feature = "dynamic") {
        build.define("LIBTYPE", "DYNAMIC");
    } else {
        build.define("LIBTYPE", "STATIC");
    }
    let openal_soft_out = build.build();

    // Some dependencies of OpenAL Soft.
    if cfg!(target_os = "macos") {
        link_framework("AudioToolbox");
        link_framework("CoreAudio");
        link_framework("CoreFoundation");
    }

    link_search(&openal_soft_out.join("lib"));
    // some Linux systems
    link_search(&openal_soft_out.join("lib64"));
    // see also src/audio/openal.rs
    link_lib(if cfg!(target_os = "windows") {
        "OpenAL32"
    } else {
        "openal"
    });

    if cfg!(target_os = "linux") {
        // OpenAL on Linux depends on sndio, needs to be dynamically linked
        println!("cargo:rustc-link-lib=dylib=sndio");
    }
    // rerun-if-changed seems to not work if pointed to a directory :(
    //rerun_if_changed(&workspace_root.join("vendor/openal-soft"));
}
