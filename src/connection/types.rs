/// Motors
pub enum Motor {
    A,
    B,
    C,
    D,
}
/// All ev3's values
pub enum Value {
    Speed(usize),
    Deg(usize),
}
/// Ports
pub enum Port {
    P1,
    P2,
    P3,
    P4,
}
