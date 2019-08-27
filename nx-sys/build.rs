extern crate bindgen;
extern crate cfg_if;
use bindgen::callbacks::{ EnumVariantCustomBehavior, EnumVariantValue, IntKind, MacroParsingBehavior, ParseCallbacks };
use std::fs::OpenOptions;
use std::io::Write;
use cfg_if::cfg_if;

#[derive(Debug)]
struct CustomCallbacks;

impl ParseCallbacks for CustomCallbacks {
    fn will_parse_macro(&self, _name: &str) -> MacroParsingBehavior {
        MacroParsingBehavior::Default
    }

    fn int_macro(&self, _name: &str, _value: i64) -> Option<IntKind> {
        if _name.starts_with("POLL") && _value < i16::max_value() as i64 && _value > i16::min_value() as i64 {
            Some(IntKind::I16)
        }
        else if _name.starts_with("DT_") && _value > 0 && _value < u8::max_value() as i64 {
            Some(IntKind::U8)
        }
        else if _name.starts_with("S_IF") && _value > 0 && _value < u32::max_value() as i64 {
            Some(IntKind::U32)
        }
        else if _value < i32::max_value() as i64 && _value > i32::min_value() as i64 {
            Some(IntKind::I32)
        }
        else {
            None
        }
    }

    fn enum_variant_behavior(&self, _enum_name: Option<&str>, _original_variant_name: &str, _variant_value: EnumVariantValue,) -> Option<EnumVariantCustomBehavior> {
        None
    }

    fn enum_variant_name(&self, _enum_name: Option<&str>, _original_variant_name: &str, _variant_value: EnumVariantValue,) -> Option<String> {
        None
    }
}

pub fn get_devkitpro() -> Option<String> {
    match std::env::var("DEVKITPRO") {
        Ok(_var) => {
            match cfg!(windows) {
                true => {
                    let path = std::env::var("PATH").unwrap();
                    let dummy_path = format!("{}devkitA64{}bin", std::path::MAIN_SEPARATOR, std::path::MAIN_SEPARATOR).to_owned();
                    let pathvars : Vec<&str> = path.split(';').collect();
                    for var in &pathvars {
                        if var.ends_with(&dummy_path) {
                            let dkp = &var[0..var.len() - dummy_path.len()];
                            return Some(dkp.to_string());
                        }
                    }
                    None
                },
                false => Some(String::from("/opt/devkitpro"))
            }

        },
        _ => None
    }
}

pub fn regen_bindings(input: &str, output: &str, whitelist: Option<Vec<String>>) -> Result<bindgen::Bindings, std::io::Error> {

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let out_p = out_path.join(output);
    let gen_path = out_p.to_str().unwrap();

    let in_path = std::path::PathBuf::from("bindgen");
    let in_p = in_path.join(input);
    let header_wrapper = in_p.to_str().unwrap();

    // we don't care if deletion succeeds, as long as the file is gone
    let _ = std::fs::remove_file(gen_path);
    assert!(!std::path::Path::new(gen_path).exists());

    let dkp = match get_devkitpro() {
        Some(path) => path,
        _ => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Oof"))
    };

    let mut builder = bindgen::Builder::default().trust_clang_mangling(false).use_core().rust_target(bindgen::RustTarget::Nightly).ctypes_prefix("ctypes").generate_inline_functions(true).parse_callbacks(Box::new(CustomCallbacks{})).header(header_wrapper)
    .clang_arg(format!("-I{}", std::path::Path::new(&dkp).join("libnx").join("include").to_str().unwrap()))
    .clang_arg(format!("-I{}", std::path::Path::new(&dkp).join("portlibs").join("switch").join("include").to_str().unwrap()))
    .clang_arg(format!("-I{}", std::path::Path::new(&dkp).join("devkitA64").join("aarch64-none-elf").join("include").to_str().unwrap()))
    .clang_arg(format!("-I{}", std::path::Path::new(&dkp).join("devkitA64").join("lib").join("gcc").join("aarch64-none-elf").join("8.3.0").join("include").to_str().unwrap()))
    .blacklist_type("u8").blacklist_type("u16").blacklist_type("u32").blacklist_type("u64");
    
    if let Some(whitelist) = whitelist {
        for func in whitelist {
            builder = builder.whitelist_function(func);
        }
    }

    builder.generate().map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Could not create file!")).and_then(|bnd| {
        let mut file = OpenOptions::new().write(true).create(true).open(gen_path)?;
        file.write_all(br#"
        mod ctypes {
            pub type c_void = core::ffi::c_void;
            pub type c_char = u8;
            pub type c_int = i32;
            pub type c_long = i64;
            pub type c_longlong = i64;
            pub type c_schar = i8;
            pub type c_short = i16;
            pub type c_uchar = u8;
            pub type c_uint = u32;
            pub type c_ulong = u64;
            pub type c_ulonglong = u64;
            pub type c_ushort = u16;
            pub type size_t = u64;
            pub type ssize_t = i64;
            pub type c_float = f32;
            pub type c_double = f64;
        }
        "#)?;
        bnd.write(Box::new(file)).map(|_| bnd)
    })
}

pub fn process_bindgen(input: &str, output: &str, name: &str, whitelist: Option<Vec<String>>) {
    regen_bindings(input, output, whitelist).expect(&format!("Error generating {}'s bindings!", name));
}

cfg_if! {
    if #[cfg(feature = "twili")] {
        pub fn process_twili() {
            process_bindgen("twili.h", "twili.rs", "twili", Some(vec!["twiliWriteNamedPipe".to_string(), "twiliCreateNamedOutputPipe".to_string(), "twiliCreateNamedOutputPipe".to_string(), "twiliInitialize".to_string(), "twiliExit".to_string()]));
        }
    }
    else {
        pub fn process_twili() {
        }
    }
}

pub fn main() {
    process_bindgen("libnx.h", "libnx.rs", "libnx", None);
    process_twili();
}