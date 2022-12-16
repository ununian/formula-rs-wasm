pub enum Operator {
    // Math
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,

    // Load
    LoadNumber,
    LoadString,
    LoadIdentifier,

    // func 
    Call
}
