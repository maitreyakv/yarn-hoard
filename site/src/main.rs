fn main() {
    console_error_panic_hook::set_once();
    sycamore::render(site::App);
}
