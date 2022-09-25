pub fn on(pin: u16) {
    println!("  digitalWrite({}, HIGH);", pin);
}

pub fn off(pin: u16) {
    println!("  digitalWrite({}, LOW);", pin);
}

pub fn def_pin(pin: u16, is_input: bool) {
    if is_input {
        println!("  pinMode({}, INPUT);", pin);
    } else {
        println!("  pinMode({}, OUTPUT);", pin);
    }
}

pub fn begin_setup() {
    println!("void setup() {{");
}

pub fn begin_loop() {
    println!("void loop() {{");
}

pub fn end_function() {
    println!("}}");
}

pub fn halt(seconds: f64) {
    println!("  delay({});", seconds * 1000.0);
}

pub fn analog_info(pin: u64) {
    println!("  analogRead({});", pin);
}

pub fn analog_info_var(var_name: String, variation: String, pin: u64) {
    println!("  {} {} = analogRead({});", var_name.to_string(), variation.to_string(), pin);
}

pub fn digital_ino(pin: u64) {
    println!("  digitalRead({});", pin);
}

pub fn digital_info_var(var_name: String, variation: String, pin: u64) {
    println!("  {} {} = digitalRead({});", var_name.to_string(), variation.to_string(), pin);
}

pub fn serial_print(printing: String) {
    println!("  Serial.print(\"{}\");", printing);
}

pub fn serial_println(printing: String) {
    println!("  Serial.println(\"{}\");", printing);
}

pub fn serial_analog(pin: u64) {
    println!("  Serial.print( analogRead({}) );", pin)
}

pub fn serial_digital(pin: u64) {
    println!("  Serial.print( digitalRead({}) );", pin)
}

pub fn serial_begin(baud: u64) {
    println!("  Serial.begin({});", baud);
}

pub fn pulse_in_var(var_name: String, variation: String, pin: u64) {
    println!("  {} {} = pulseIn({}, HIGH);", variation, var_name, pin);
}

pub fn pulse_in(pin: u64) {
    println!("  pulseIn({});", pin);
}

pub fn def_var(var_name: String, variation: String, set_to: u64) {
    println!("  {} {} = {};", variation, var_name, set_to);
}

pub fn if_cond(condition: String, cmd: String) {
    println!("  if ({}) {{\n
        {}\n
    }}", condition, cmd);
}