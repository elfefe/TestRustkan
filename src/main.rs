mod graphics;
mod gui;

fn main() {
    let graphics = graphics::init_gpu_graphics();
    let gui = gui::init_ui();

    show("Buffer contain: ".to_string() + &*graphics);
}

fn show(arg: String) {
    println!("What is {:?} ?", arg);
}