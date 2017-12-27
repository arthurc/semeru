extern crate semeru;

fn run() -> semeru::Result<()> {
    let mut vm = semeru::VM::build()
        .expect("Failed to initialize")
        .with_module(semeru::java8::Module::new())?
        .build()?;

    Ok(vm.run())
}

fn main() {
    if let Err(e) = run() {
        println!("Failed to execute VM: {}", e);
        ::std::process::exit(1);
    }
}
