extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let mut include_paths: Vec<String> = vec![];
    match pkg_config::Config::new()
	.probe("tkrzw") {
	    Ok(lib) => {
		lib.include_paths.iter().for_each(|p| {
		    include_paths.push(p.to_str().unwrap().to_string());
		});
	    },
	    Err(_) => {
		panic!("Cannot find tkrzw by pkg-config");
	    }
	};
    println!("cargo:rerun-if-changed=wrapper.hpp");
    let mut clang_args: Vec<String> = include_paths.iter()
	.map(|p| format!("-I{}", p)).collect();
    clang_args.push(String::from("-std=c++17"));
    let bindings = bindgen::Builder::default()
	.header("wrapper.hpp")
	.clang_args(clang_args)
     	.parse_callbacks(Box::new(bindgen::CargoCallbacks))
	.opaque_type("std::*")
	.whitelist_type("tkrzw::HashDBM")
//	.blacklist_type("size_type")
//	.blacklist_type("int_type")
//	.blacklist_type("char_type")
     	.generate()
    	.expect("Unable to generate bindings");    
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
     	.write_to_file(out_path.join("bindings.rs"))
     	.expect("Couldn't write bindings!");
}
