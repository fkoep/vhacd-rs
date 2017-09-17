extern crate cpp_build;
extern crate cmake;

fn main() {
    cpp_build::Config::new()
        .include("v-hacd/src/VHACD_Lib/public/")
        .build("src/lib.rs");

    let dest = cmake::Config::new("v-hacd/src")
        // .cxxflag("pthread")
        .define("NO_OPENCL", "1")
        .define("NO_OPENMP", "1")
        .define("CMAKE_CXX_FLAGS", " -pthread -fPIC ")
        .build();


    println!("cargo:rustc-link-search=native={}/lib", dest.display());
    println!("cargo:rustc-link-lib=static=vhacd");
}
