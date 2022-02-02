#[derive(Debug)]
struct IR {
    instructions: Vec<IRLine>,
}

#[derive(Debug)]
enum IRLine {
    Instruction(IRInstruction),
    Label(String),
}

#[derive(Debug)]
struct IRInstruction {
    cmd: IRCommand,
    param1: Option<IRParameter>,
    param2: Option<IRParameter>,
}

#[derive(Debug)]
enum IRCommand {
    Mov,
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Cmp,
    Inc,
    Dec,
    // Bit Operations
    And,
    Or,
    Xor,
    Shl,
    Shr,
    Not,
    // Jump
    Jmp,
    Jz,
    Jnz,
    Js,
    Jns,
    Jle,
    Jgt,
    // Stack
    Push,
    Pop,
    // Call
    Call,
    Int,
    Ret,
    // Misc
    Halt,
    Nop,
}

#[derive(Debug)]
enum IRParameter {
    Reg(IRRegister),
    Imm(i32),
    Label(String),
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
enum IRRegister {
    A = 0x01,
    B = 0x02,
    C = 0x03,
    D = 0x04,
    IP = 0x05,
    SP = 0x06,
}

fn main() {
    println!("Hello, world!");
}
