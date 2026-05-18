fn main() {
    cc::Build::new()
        .file("extern/seamoptimizer.cpp")
        .include("extern")
        .opt_level(3)
        .compile("seamoptimizer");
}
