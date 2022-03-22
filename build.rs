use std::fs;

fn main() {
    pyo3_build_config::add_extension_module_link_args();
    let pyo3_interpreter = pyo3_build_config::get();
    let output = pyo3_interpreter
        .run_python_script(
            &fs::read_to_string("./sample_python_build_script.py")
                .expect("failed to read sample_python_build_script.py"),
        )
        .expect("failed to run sample_python_build_script.py");
    println!("{}", output);
}
