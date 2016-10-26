static STATUS_FLAG: u8 = 1 << 0;

struct Register {
    A: u8,
    X: u8,
    Y: u8,
    S: u8,
    P: u8,
    PC: u16
}


fn implied() {
}

fn accumulator() {
}

fn immediate() {
}

fn zeropage() {
}

fn zeropage_x() {
}

fn zeropage_y() {
}

fn relative() {
}

fn absolute() {
}

fn absolute_x() {
}

fn absolute_y() {
}

fn indirect() {
}

fn indirect_x() {
}

fn indirect_y() {
}
