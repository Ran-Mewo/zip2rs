use std::env;
use std::path::PathBuf;
use std::fs;

fn main() {
    // Check if bundled feature is enabled
    let bundled = env::var("CARGO_FEATURE_BUNDLED").is_ok();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();

    // Determine the platform-specific directory
    let platform_dir = get_platform_dir(&target_os, &target_arch, &target_env);
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    println!("cargo:warning=Building for platform: {} ({})", platform_dir, target_os);

    // Try multiple locations for native libraries in priority order
    let mut native_lib_paths = vec![
        // 1. Default to general build output (when building locally)
        PathBuf::from(&manifest_dir).join("zip4j-abi").join("build").join("native").join("nativeCompile"),
        // 2. Default to prebuilt location (platform-specific)
        PathBuf::from(&manifest_dir).join("zip4j-abi").join("build").join("native").join("nativeCompile").join(&platform_dir),
    ];

    // 3. For musl targets, also try the corresponding glibc directory as fallback
    if platform_dir.contains("musl") {
        let glibc_platform_dir = platform_dir.replace("-musl", "");
        native_lib_paths.push(
            PathBuf::from(&manifest_dir).join("zip4j-abi").join("build").join("native").join("nativeCompile").join(&glibc_platform_dir)
        );
    }

    // 4. Legacy lib directory (fallback)
    native_lib_paths.push(PathBuf::from(&manifest_dir).join("lib"));

    let mut found_lib_dir = None;
    for path in &native_lib_paths {
        if path.exists() && has_native_library(path, &target_os) {
            println!("cargo:warning=Found native libraries at: {}", path.display());
            found_lib_dir = Some(path.clone());
            break;
        }
    }

    // Libraries should be pre-built by GitHub Actions workflow

    let lib_dir = found_lib_dir.unwrap_or_else(|| {
        panic!("Native library not found for platform {}. Please build it first or check the zip4j-abi/build/native/nativeCompile/ directory.", platform_dir);
    });

    // Set up linking based on target OS
    setup_linking(&lib_dir, &target_os);

    // Copy libraries to target directory for runtime
    copy_runtime_libraries(&lib_dir, &target_os);

    // Find and use header file for bindings
    let header_path = find_header_file(&lib_dir, &manifest_dir);
    generate_bindings(&header_path);

    // Generate embedded libraries if bundled feature is enabled
    if bundled {
        generate_embedded_libraries(&manifest_dir);
    }

    // Tell cargo to rerun this build script if files change
    println!("cargo:rerun-if-changed=zip4j-abi/build/native/nativeCompile/");
    println!("cargo:rerun-if-changed=zip4j-abi/src/");
    println!("cargo:rerun-if-changed=lib/");
}

fn get_platform_dir(target_os: &str, target_arch: &str, target_env: &str) -> String {
    match (target_os, target_arch, target_env) {
        ("windows", "x86_64", _) => "windows-x86_64".to_string(),
        ("windows", "aarch64", _) => "windows-aarch64".to_string(),
        ("macos", "x86_64", _) => "darwin-x86_64".to_string(),
        ("macos", "aarch64", _) => "darwin-aarch64".to_string(),
        ("linux", "x86_64", "musl") => "linux-x86_64-musl".to_string(),
        ("linux", "aarch64", "musl") => "linux-aarch64-musl".to_string(),
        ("linux", "x86_64", _) => "linux-x86_64".to_string(),
        ("linux", "aarch64", _) => "linux-aarch64".to_string(),
        _ => format!("{}-{}", target_os, target_arch),
    }
}

fn has_native_library(path: &PathBuf, target_os: &str) -> bool {
    match target_os {
        "windows" => {
            path.join("zip4j-abi.dll").exists() && path.join("zip4j-abi.lib").exists()
        }
        "macos" => {
            path.join("libzip4j-abi.dylib").exists() || path.join("zip4j-abi.dylib").exists()
        }
        "linux" => {
            path.join("libzip4j-abi.so").exists() || path.join("zip4j-abi.so").exists()
        }
        _ => false,
    }
}



fn setup_linking(lib_dir: &PathBuf, target_os: &str) {
    println!("cargo:rustc-link-search=native={}", lib_dir.display());

    match target_os {
        "windows" => {
            // On Windows, link to the import library
            println!("cargo:rustc-link-lib=dylib=zip4j-abi");
        }
        "macos" => {
            // On macOS, check which naming convention is used
            if lib_dir.join("libzip4j-abi.dylib").exists() {
                println!("cargo:rustc-link-lib=dylib=zip4j-abi");
            } else if lib_dir.join("zip4j-abi.dylib").exists() {
                // Create a symlink with the expected lib prefix or copy the file
                let src = lib_dir.join("zip4j-abi.dylib");
                let dst = lib_dir.join("libzip4j-abi.dylib");
                if !dst.exists() {
                    if std::fs::hard_link(&src, &dst).is_err() {
                        // If hard link fails, try copying
                        let _ = std::fs::copy(&src, &dst);
                    }
                }
                println!("cargo:rustc-link-lib=dylib=zip4j-abi");
            }
            // Add rpath for runtime library loading
            println!("cargo:rustc-link-arg=-Wl,-rpath,@loader_path");
        }
        "linux" => {
            // On Linux, check which naming convention is used
            if lib_dir.join("libzip4j-abi.so").exists() {
                println!("cargo:rustc-link-lib=dylib=zip4j-abi");
            } else if lib_dir.join("zip4j-abi.so").exists() {
                // Create a symlink with the expected lib prefix or copy the file
                let src = lib_dir.join("zip4j-abi.so");
                let dst = lib_dir.join("libzip4j-abi.so");
                if !dst.exists() {
                    if std::fs::hard_link(&src, &dst).is_err() {
                        // If hard link fails, try copying
                        let _ = std::fs::copy(&src, &dst);
                    }
                }
                println!("cargo:rustc-link-lib=dylib=zip4j-abi");
            }
            // Add rpath for runtime library loading
            println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");
        }
        _ => {
            println!("cargo:rustc-link-lib=dylib=zip4j-abi");
        }
    }
}

fn copy_runtime_libraries(lib_dir: &PathBuf, target_os: &str) {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let target_dir = out_dir.parent().unwrap().parent().unwrap().parent().unwrap();

    match target_os {
        "windows" => {
            let dll_path = lib_dir.join("zip4j-abi.dll");
            if dll_path.exists() {
                let target_dll = target_dir.join("zip4j-abi.dll");
                if std::fs::copy(&dll_path, &target_dll).is_ok() {
                    println!("cargo:warning=Copied DLL to {}", target_dll.display());
                }

                // Copy to examples directory
                let examples_dir = target_dir.join("examples");
                if examples_dir.exists() || std::fs::create_dir_all(&examples_dir).is_ok() {
                    let examples_dll = examples_dir.join("zip4j-abi.dll");
                    if std::fs::copy(&dll_path, &examples_dll).is_ok() {
                        println!("cargo:warning=Copied DLL to {}", examples_dll.display());
                    }
                }
            }
        }
        "macos" => {
            for dylib_name in &["libzip4j-abi.dylib", "zip4j-abi.dylib"] {
                let dylib_path = lib_dir.join(dylib_name);
                if dylib_path.exists() {
                    // Copy with both naming conventions to ensure compatibility
                    let target_dylib = target_dir.join(dylib_name);
                    if std::fs::copy(&dylib_path, &target_dylib).is_ok() {
                        println!("cargo:warning=Copied dylib to {}", target_dylib.display());
                    }

                    // Also copy with the other naming convention if it doesn't exist
                    let alt_name = if dylib_name.starts_with("lib") {
                        "zip4j-abi.dylib"
                    } else {
                        "libzip4j-abi.dylib"
                    };
                    let alt_target = target_dir.join(alt_name);
                    if !alt_target.exists() {
                        let _ = std::fs::copy(&dylib_path, &alt_target);
                    }
                    break;
                }
            }
        }
        "linux" => {
            for so_name in &["libzip4j-abi.so", "zip4j-abi.so"] {
                let so_path = lib_dir.join(so_name);
                if so_path.exists() {
                    // Copy with both naming conventions to ensure compatibility
                    let target_so = target_dir.join(so_name);
                    if std::fs::copy(&so_path, &target_so).is_ok() {
                        println!("cargo:warning=Copied shared library to {}", target_so.display());
                    }

                    // Also copy with the other naming convention if it doesn't exist
                    let alt_name = if so_name.starts_with("lib") {
                        "zip4j-abi.so"
                    } else {
                        "libzip4j-abi.so"
                    };
                    let alt_target = target_dir.join(alt_name);
                    if !alt_target.exists() {
                        let _ = std::fs::copy(&so_path, &alt_target);
                    }
                    break;
                }
            }
        }
        _ => {}
    }
}

fn find_header_file(lib_dir: &PathBuf, manifest_dir: &str) -> PathBuf {
    let header_candidates = vec![
        // 1. In the same directory as the found library
        lib_dir.join("zip4j-abi.h"),
        // 2. In the general build output directory
        PathBuf::from(manifest_dir).join("zip4j-abi").join("build").join("native").join("nativeCompile").join("zip4j-abi.h"),
        // 3. Legacy lib directory
        PathBuf::from(manifest_dir).join("lib").join("zip4j-abi.h"),
    ];

    for candidate in header_candidates {
        if candidate.exists() {
            return candidate;
        }
    }

    // Create a minimal header if none found
    let fallback_header = PathBuf::from(manifest_dir).join("lib").join("zip4j-abi.h");
    std::fs::create_dir_all(fallback_header.parent().unwrap()).unwrap_or_default();
    std::fs::write(&fallback_header, "// Minimal header file\n").unwrap_or_default();
    fallback_header
}

fn generate_bindings(header_path: &PathBuf) {
    let header_dir = header_path.parent().unwrap();

    let bindings = bindgen::Builder::default()
        .header(header_path.to_string_lossy())
        .clang_arg(format!("-I{}", header_dir.to_string_lossy()))
        // Ensure consistent char handling across platforms
        .default_alias_style(bindgen::AliasVariation::TypeAlias)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:warning=Generated bindings at {}", out_dir.join("bindings.rs").display());
}

fn generate_embedded_libraries(manifest_dir: &str) {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let native_dir = PathBuf::from(manifest_dir).join("zip4j-abi").join("build").join("native").join("nativeCompile");

    // Platform-specific library mappings
    let platforms = vec![
        ("windows-x86_64", "zip4j-abi.dll"),
        ("linux-x86_64", "libzip4j-abi.so"),
        ("linux-x86_64-musl", "libzip4j-abi.so"),
        ("linux-aarch64", "libzip4j-abi.so"),
        ("darwin-x86_64", "libzip4j-abi.dylib"),
        ("darwin-aarch64", "libzip4j-abi.dylib"),
    ];

    let mut embedded_code = String::new();
    embedded_code.push_str("// Auto-generated embedded libraries\n");
    embedded_code.push_str("use std::collections::HashMap;\n\n");
    embedded_code.push_str("pub struct EmbeddedLibrary {\n");
    embedded_code.push_str("    pub data: &'static [u8],\n");
    embedded_code.push_str("    pub filename: &'static str,\n");
    embedded_code.push_str("}\n\n");
    embedded_code.push_str("pub fn get_embedded_libraries() -> HashMap<&'static str, EmbeddedLibrary> {\n");
    embedded_code.push_str("    let mut libs = HashMap::new();\n\n");

    for (platform, filename) in platforms {
        let lib_path = native_dir.join(platform).join(filename);
        if lib_path.exists() {
            embedded_code.push_str(&format!(
                "    libs.insert(\"{}\", EmbeddedLibrary {{\n",
                platform
            ));
            embedded_code.push_str(&format!(
                "        data: include_bytes!(\"{}\"),\n",
                lib_path.to_string_lossy().replace("\\", "/")
            ));
            embedded_code.push_str(&format!(
                "        filename: \"{}\",\n",
                filename
            ));
            embedded_code.push_str("    });\n\n");

            println!("cargo:warning=Embedding library for {}: {}", platform, lib_path.display());
        } else {
            println!("cargo:warning=Library not found for {}: {}", platform, lib_path.display());
        }
    }

    embedded_code.push_str("    libs\n");
    embedded_code.push_str("}\n");

    // Write the embedded libraries code
    let embedded_file = out_dir.join("embedded_libs.rs");
    fs::write(&embedded_file, embedded_code).expect("Failed to write embedded libraries");

    println!("cargo:warning=Generated embedded libraries at {}", embedded_file.display());
}
