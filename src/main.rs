mod ino_rs;

fn main() {
    let trig = 10;
    let echo = 11;
    let red = 3;

    ino_rs::begin_setup();
    ino_rs::def_pin(trig, false);
    ino_rs::def_pin(echo, true);
    ino_rs::def_pin(red, false);
    ino_rs::end_function();

    ino_rs::begin_loop();
    ino_rs::off(trig);
    ino_rs::halt(0.01);
    ino_rs::on(trig);
    ino_rs::halt(0.01);
    ino_rs::off(trig);

    ino_rs::pulse_in_var("time_taken".to_string(), "int".to_string(), echo.into());

    ino_rs::if_cond("time_taken < 500".to_string(), "digitalWrite(3, HIGH)".to_string());
    ino_rs::if_cond("time_taken > 500".to_string(), "digitalWrite(3, LOW)".to_string());

    ino_rs::end_function();

}