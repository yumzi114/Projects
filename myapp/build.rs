fn main() {
    glib_build_tools::compile_resources(
        &["src/bin/resources"],
        "src/bin/resources/resources.gresource.xml",
        "myapp.gresource",
    );
}