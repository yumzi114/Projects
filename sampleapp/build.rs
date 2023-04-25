fn main() {
    glib_build_tools::compile_resources(
        &["src/bin/resources"],
        "src/bin/resources/resources.gresource.xml",
        "todo_1.gresource",
    );
}