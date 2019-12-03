/// Motors
pub enum Motor {
    A, B, C, D
}
/// Values
pub enum Value {
    Speed(usize),
    Deg(usize),
}
/// Ports
pub enum Port {
    P1, P2, P3, P4,
}
/// All ev3's types
pub enum Ev3Type {
    Port(Port),
    Value(Value),
    Motor(Motor),
}

// pub trait Define {
//     fn define(&self) -> Vec<u8>
// }

// impl Define for Ev3Type {
//     fn define(&self) -> Vec<u8> {
//         let mut res = Vec::new();
//         match *self {
//            Ev3Type::Port(t) => res = t.define(),
//            Ev3Type::Value(t) => res = t.define(),
//            Ev3Type::Motor(t) => res = t.define(),
//         }
//         res
//     }
// } 