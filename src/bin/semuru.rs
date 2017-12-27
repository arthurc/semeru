extern crate semuru;

fn main() {
    semuru::VM::build()
        .expect("Failed to initialize")
        .build()
        .expect("Failed to build VM")
        .run();
}
