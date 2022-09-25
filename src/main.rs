mod ino_rs;

fn main() {
	let red = 3;
	let green = 5;
	let blue = 6;

	ino_rs::begin_setup();
	ino_rs::def_pin(red, false);
	ino_rs::def_pin(green, false);
	ino_rs::def_pin(blue, false);
	ino_rs::end_function();

	ino_rs::begin_loop();
	ino_rs::on(red);
	ino_rs::off(green);
	ino_rs::off(blue);
	ino_rs::halt(1.0);
	
	ino_rs::on(green);
	ino_rs::off(red);
	ino_rs::off(blue);
	ino_rs::halt(1.0);
	
	ino_rs::on(blue);
	ino_rs::off(green);
	ino_rs::off(red);
	ino_rs::halt(1.0);
	ino_rs::end_function();
}