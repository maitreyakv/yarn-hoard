fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
    sycamore::render(yarn_hoard::frontend::App);
}
