#[cfg(feature = "unstable")]
mod inner {
    pub fn main() {}
}

#[cfg(not(feature = "unstable"))]
mod inner {
    extern crate serde_codegen;

    use std::env;
    use std::path::Path;
    use std::fs;

    pub fn main() {
        let outdir = env::var_os("OUT_DIR").unwrap();
        for src in fs::read_dir("src").unwrap()
            .filter_map(Result::ok)
            .filter(|s| s.file_type().unwrap().is_file())
            .map(|s| s.path())
            .filter(|s| s.extension().unwrap() == "in") {
                let dst = Path::new(&outdir).join(src.file_stem().unwrap());
                serde_codegen::expand(&src, &dst).unwrap();
        }
    }
}

fn main() {
    inner::main();
}
