use std::env;
use std::path::PathBuf;
use bindgen::callbacks::{EnumVariantValue, ItemInfo, ItemKind};


#[derive(Debug)]
struct CargoCallbacks;

impl bindgen::callbacks::ParseCallbacks for CargoCallbacks{
    fn process_comment(&self, comment: &str) -> Option<String> {
        Some(format!("````ignore\n{}\n````", comment))
    }

    fn enum_variant_name(
        &self,
        _enum_name: Option<&str>,
        original_variant_name: &str,
        _variant_value: EnumVariantValue
    ) -> Option<String>{
        Some(format!("{}", heck::AsUpperCamelCase(original_variant_name)))
    }

    fn generated_name_override(&self, item_info: ItemInfo<'_>) -> Option<String> {
        match item_info.kind {
            ItemKind::Var => {
                let stripped = match item_info.name.strip_prefix("ts") {
                    Some(x) => x,
                    None => item_info.name
                };
                Some(format!("{}", heck::AsSnakeCase(stripped)))
            },
            ItemKind::Function => {
                let stripped = match item_info.name.strip_prefix("ts") {
                    Some(x) => x,
                    None => item_info.name
                };
                Some(format!("{}", heck::AsSnakeCase(stripped)))
            },
            _ => Some(item_info.name.to_string())
        }
    }
}

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");

    let project_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let mut ts_path = project_path;
    ts_path.push("src");

    let files = [
        "tinyspline.c",
    ];

    let mut ts_build = cc::Build::new();

    ts_build
        .include(&ts_path)
        .files(files.iter().map(|file| {
            let mut path = ts_path.clone();
            path.push(file);
            path
        }))
        .warnings(false)
        .opt_level(3);

    ts_build.compile("tinyspline");

    println!("cargo:rustc-link-lib=tinyspline");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(CargoCallbacks))
        .rustified_enum(".*")
        .generate()
        .expect("Unable to generate tinyspline bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}