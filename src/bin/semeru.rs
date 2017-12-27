extern crate semeru;

fn main() {
    semeru::VM::build()
        .expect("Failed to initialize")
        #[cfg(feature = "java8")]
        .with_module(semeru::java8::Java8Module::new())
        .build()
        .expect("Failed to build VM")
        .run();
}
