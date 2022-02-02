use std::collections::HashMap;

const COMMENT_CHAR: char = ';';

#[derive(Debug)]
pub struct IR {
    pub instructions: Vec<IRLine>,
}

#[derive(Debug)]
pub enum IRLine {
    Ins(IRInstruction),
    Label(String),
}

#[derive(Debug)]
pub struct IRInstruction {
    pub command: IRCommand,
    pub param1: Option<IRParameter>,
    pub param2: Option<IRParameter>,
    pub line_number: usize,
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum IRCommand {
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
pub enum IRParameter {
    Reg(IRRegister),
    Imm(i32),
    Label(String),
    MemReg(IRRegister),
    MemImm(i32),
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum IRRegister {
    A = 0x01,
    B = 0x02,
    C = 0x03,
    D = 0x04,
    IP = 0x05,
    SP = 0x06,
}

impl IRInstruction {
    fn with_cmd_and_params_string(command: IRCommand, params: &str, line_number: usize) -> Option<IRInstruction> {
        if params == "" {
            return Some(IRInstruction { command, param1: None, param2: None, line_number });
        }

        let param_count = params.matches(',').count() + 1;
        match param_count {
            1 => Some(IRInstruction { command, param1: Some(IRParameter::from(params, line_number)?), param2: None, line_number }),
            2 => {
                // param_count == 2 implies there is one ','
                let (param1, param2) = params.split_once(',').unwrap();
                Some(IRInstruction {
                    command, line_number,
                    param1: Some(IRParameter::from(param1, line_number)?),
                    param2: Some(IRParameter::from(param2, line_number)?),
                })
            }
            _ => {
                eprintln!("Instructions with more than two arguments are not supported. (line {})", line_number);
                None
            }
        }
    }
}

impl IRCommand {
    fn translation_table() -> HashMap<&'static str, IRCommand> {
        HashMap::from([
            ("mov", Self::Mov),
            ("add", Self::Add),
            ("sub", Self::Sub),
            ("mul", Self::Mul),
            ("div", Self::Div),
            ("mod", Self::Mod),
            ("pow", Self::Pow),
            ("inc", Self::Inc),
            ("dec", Self::Dec),
            ("and", Self::And),
            ("or", Self::Or),
            ("xor", Self::Xor),
            ("shl", Self::Shl),
            ("shr", Self::Shr),
            ("not", Self::Not),
            ("cmp", Self::Cmp),
            ("jmp", Self::Jmp),
            ("jz", Self::Jz),
            ("jnz", Self::Jnz),
            ("js", Self::Js),
            ("jns", Self::Jns),
            ("je", Self::Jz),
            ("jne", Self::Jnz),
            ("jlt", Self::Js),
            ("jge", Self::Jns),
            ("jle", Self::Jle),
            ("jgt", Self::Jgt),
            ("push", Self::Push),
            ("pop", Self::Pop),
            ("call", Self::Call),
            ("int", Self::Int),
            ("ret", Self::Ret),
            ("halt", Self::Halt),
            ("nop", Self::Nop),
        ])
    }
}

impl IRParameter {
    fn from(param: &str, line_number: usize) -> Option<IRParameter> {
        let param = param.trim();
        if param == "" {
            eprintln!("Invalid empty parameter on line {}", line_number);
            return None;
        }

        if param.starts_with('[') && param.ends_with(']') {
            // Remove both '[' prefix and ']' suffix.
            let mut chars = param.chars();
            chars.next();
            chars.next_back();
            let inner = IRParameter::from(chars.as_str(), line_number);
            return match inner {
                Some(IRParameter::Reg(register)) => Some(IRParameter::MemReg(register)),
                Some(IRParameter::Imm(value)) => Some(IRParameter::MemImm(value)),
                None => None, // Already printed an error message.
                Some(_) => {
                    eprintln!("Invalid parameter '{}' on line {}", param, line_number);
                    None
                }
            }
        }

        if param.starts_with(|c: char| c.is_digit(10) || c == '-' || c == '+') {
            return Some(IRParameter::Imm(Self::get_immediate_value(param, line_number)?));
        }

        if let Some(register) = IRRegister::from(param) {
            return Some(IRParameter::Reg(register));
        }

        if param.chars().all(|c|c.is_ascii_alphabetic()) {
            return Some(IRParameter::Label(param.into()));
        }

        eprintln!("Invalid parameter '{}' on line {}", param, line_number);
        None
    }

    fn get_immediate_value(mut param: &str, line_number: usize) -> Option<i32> {
        let original = param;
        let negative = param.starts_with('-');
        let negative_factor = if negative { -1 } else { 1 };

        if negative || param.starts_with('+') {
            // Remove '-' or '+' prefix.
            let mut chars = param.chars();
            chars.next();
            param = chars.as_str();
        }

        let conversion = if param.starts_with("0x") {
            // Remove the '0x'.
            let mut chars = param.chars();
            chars.next();
            chars.next();
            i32::from_str_radix(chars.as_str(), 16)
        } else {
            param.parse::<i32>()
        };

        match conversion {
            Ok(number) => Some(negative_factor * number),
            Err(e) => {
                eprintln!("Invalid number '{}' on line {}: {}", original, line_number, e);
                None
            }
        }
    }
}

impl IRRegister {
    fn from(param: &str) -> Option<IRRegister> {
        match param {
            "A" | "a" => Some(IRRegister::A),
            "B" | "b" => Some(IRRegister::B),
            "C" | "c" => Some(IRRegister::C),
            "D" | "d" => Some(IRRegister::D),
            "IP" | "ip" => Some(IRRegister::IP),
            "SP" | "sp" => Some(IRRegister::SP),
            _ => None,
        }
    }
}

pub struct TranslationTable {
    command: HashMap<&'static str, IRCommand>,
}

impl TranslationTable {
    pub fn new() -> TranslationTable {
        TranslationTable {
            command: IRCommand::translation_table(),
        }
    }

    pub fn create_intermediate(&self, line: &str, line_number: usize) -> Option<IRLine> {
        // Remove any comments from the line.
        let split = line.split_once(COMMENT_CHAR);
        let line = match split {
            Some((left, _)) => left,
            None => line,
        };

        let split = line.split_once(char::is_whitespace);
        let (left, right) = match split {
            Some((left, right)) => (left, right),
            None => (line, ""),
        };

        let left = left.trim();
        let right = right.trim();

        // Handle empty lines.
        if left.len() == 0 {
            return None;
        }

        // Handle jump labels.
        if left.ends_with(':') {
            let mut label = left.chars();
            label.next_back(); // Remove the ':'
            return Some(IRLine::Label(label.as_str().into()))
        }

        // Handle instructions.
        let left =left.to_ascii_lowercase();
        match self.command.get(&left as &str) {
            Some(command) => Some(IRLine::Ins(IRInstruction::with_cmd_and_params_string(command.clone(), right, line_number)?)),
            None => {
                eprintln!("Invalid command '{}' on line {}", left, line_number); // TODO: Proper error handling.
                None
            }
        }
    }
}
