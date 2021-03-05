mod graphics;
mod gui;
mod piston;

fn main() {
    //let _graphics = graphics::init_gpu_graphics();
    // let _gui = gui::init_ui();
    let _piston = piston::init_piston();

    // show("Buffer contain: ".to_string() + &*graphics);
}

fn show(arg: String) {
    println!("What is {:?} ?", arg);
}