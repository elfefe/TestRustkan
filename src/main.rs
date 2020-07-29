// use structopt::StructOpt;

fn main() {
	show("la youte".to_string());
}

fn show(arg: String) {
	println!("What is {:?} ?", arg);
}
