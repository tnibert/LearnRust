mod vectors;
mod strings;
mod hashmaps;

fn main() {
    vectors::vector_decls();
    vectors::vector_reads();
    vectors::vector_loop();

    strings::create();
    strings::update();
    strings::accessing_elements();

    hashmaps::scores();
    hashmaps::updating();
}
