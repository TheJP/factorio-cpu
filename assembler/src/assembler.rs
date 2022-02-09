use std::collections::HashMap;

use crate::ir::{IRCommand, IRInstruction, IRLine, IRParamType, IR, IRParameter};

struct AssemblyTranslation {
    instructions: HashMap<(IRCommand, IRParamType, IRParamType), u8>,
}

pub const HALT_INSTRUCTION: u8 = 0xee;

impl AssemblyTranslation {
    fn new() -> AssemblyTranslation {
        let instructions = HashMap::from([
            // MOV
            ((IRCommand::Mov, IRParamType::Register, IRParamType::Immediate), 0x01),
            ((IRCommand::Mov, IRParamType::Register, IRParamType::Register), 0x02),
            ((IRCommand::Mov, IRParamType::Register, IRParamType::MemoryAtImmediate), 0x03),
            ((IRCommand::Mov, IRParamType::Register, IRParamType::MemoryAtRegister), 0x04),
            ((IRCommand::Mov, IRParamType::MemoryAtImmediate, IRParamType::Immediate), 0x05),
            ((IRCommand::Mov, IRParamType::MemoryAtRegister, IRParamType::Immediate), 0x06),
            ((IRCommand::Mov, IRParamType::MemoryAtImmediate, IRParamType::Register), 0x07),
            ((IRCommand::Mov, IRParamType::MemoryAtRegister, IRParamType::Register), 0x08),

            // Arithmetic
            ((IRCommand::Add, IRParamType::Register, IRParamType::Immediate), 0x10),
            ((IRCommand::Add, IRParamType::Register, IRParamType::Register), 0x20),
            ((IRCommand::Sub, IRParamType::Register, IRParamType::Immediate), 0x11),
            ((IRCommand::Sub, IRParamType::Register, IRParamType::Register), 0x21),
            ((IRCommand::Mul, IRParamType::Register, IRParamType::Immediate), 0x12),
            ((IRCommand::Mul, IRParamType::Register, IRParamType::Register), 0x22),
            ((IRCommand::Div, IRParamType::Register, IRParamType::Immediate), 0x13),
            ((IRCommand::Div, IRParamType::Register, IRParamType::Register), 0x23),
            ((IRCommand::Mod, IRParamType::Register, IRParamType::Immediate), 0x14),
            ((IRCommand::Mod, IRParamType::Register, IRParamType::Register), 0x24),
            ((IRCommand::Pow, IRParamType::Register, IRParamType::Immediate), 0x15),
            ((IRCommand::Pow, IRParamType::Register, IRParamType::Register), 0x25),
            ((IRCommand::Inc, IRParamType::Register, IRParamType::None), 0x17),
            ((IRCommand::Dec, IRParamType::Register, IRParamType::None), 0x18),

            // Bit Operations
            ((IRCommand::And, IRParamType::Register, IRParamType::Immediate), 0x1a),
            ((IRCommand::And, IRParamType::Register, IRParamType::Register), 0x2a),
            ((IRCommand::Or, IRParamType::Register, IRParamType::Immediate), 0x1b),
            ((IRCommand::Or, IRParamType::Register, IRParamType::Register), 0x2b),
            ((IRCommand::Xor, IRParamType::Register, IRParamType::Immediate), 0x1c),
            ((IRCommand::Xor, IRParamType::Register, IRParamType::Register), 0x2c),
            ((IRCommand::Shl, IRParamType::Register, IRParamType::Immediate), 0x1d),
            ((IRCommand::Shl, IRParamType::Register, IRParamType::Register), 0x2d),
            ((IRCommand::Shr, IRParamType::Register, IRParamType::Immediate), 0x1e),
            ((IRCommand::Shr, IRParamType::Register, IRParamType::Register), 0x2e),
            ((IRCommand::Not, IRParamType::Register, IRParamType::None), 0x1f),

            // CMP
            ((IRCommand::Cmp, IRParamType::Register, IRParamType::Immediate), 0x16),
            ((IRCommand::Cmp, IRParamType::Register, IRParamType::Register), 0x26),

            // Jump
            ((IRCommand::Jmp, IRParamType::Label, IRParamType::None), 0x50),
            ((IRCommand::Jz, IRParamType::Label, IRParamType::None), 0x51),
            ((IRCommand::Jnz, IRParamType::Label, IRParamType::None), 0x52),
            ((IRCommand::Js, IRParamType::Label, IRParamType::None), 0x53),
            ((IRCommand::Jns, IRParamType::Label, IRParamType::None), 0x54),
            ((IRCommand::Jle, IRParamType::Label, IRParamType::None), 0x55),
            ((IRCommand::Jgt, IRParamType::Label, IRParamType::None), 0x56),

            // Stack
            ((IRCommand::Push, IRParamType::Immediate, IRParamType::None), 0x60),
            ((IRCommand::Push, IRParamType::Register, IRParamType::None), 0x61),
            ((IRCommand::Pop, IRParamType::Register, IRParamType::None), 0x62),

            // Call
            ((IRCommand::Call, IRParamType::Label, IRParamType::None), 0x70),
            ((IRCommand::Int, IRParamType::Register, IRParamType::None), 0x72),
            ((IRCommand::Ret, IRParamType::None, IRParamType::None), 0x71),

            // Miscellaneous
            ((IRCommand::Halt, IRParamType::None, IRParamType::None), HALT_INSTRUCTION),
            ((IRCommand::Nop, IRParamType::None, IRParamType::None), 0xff),
        ]);

        AssemblyTranslation {
            instructions
        }
    }

    fn assemble_instruction(&self, instruction: &IRInstruction) -> Vec<u8> {
        let mut assembled = vec![0u8; 4];

        // Encode instruction type byte.
        let instruction_type = instruction_type(instruction);
        let encoding = self.instructions.get(&instruction_type);
        match encoding {
            Some(&instruction_byte) => {
                assembled[3] = instruction_byte
            }
            None => {
                eprintln!("Instruction {:?} has no variation with parameters {:?},{:?} (on line {})", instruction_type.0, instruction_type.1, instruction_type.2, instruction.line_number);
                panic!()
            }
        }

        // Encode parameters.
        let mut next_register = 2;
        Self::assemble_parameter(&instruction.param1, &mut assembled, &mut next_register);
        Self::assemble_parameter(&instruction.param2, &mut assembled, &mut next_register);

        assembled
    }

    fn assemble_parameter(param: &Option<IRParameter>, assembled: &mut Vec<u8>, next_register: &mut usize) {
        match param {
            Some(IRParameter::Reg(register)) | Some(IRParameter::MemReg(register)) => {
                assembled[*next_register] = *register as u8;
                *next_register -= 1;
            }
            Some(IRParameter::Imm(value)) | Some(IRParameter::MemImm(value)) => {
                assembled.extend_from_slice(&value.to_be_bytes());
            }
            _ => {}
        }
    }
}

fn param_type(param: &Option<IRParameter>) -> IRParamType {
    match param {
        Some(p) => p.param_type(),
        None => IRParamType::None,
    }
}

fn instruction_type(instruction: &IRInstruction) -> (IRCommand, IRParamType, IRParamType) {
    (instruction.command.clone(), param_type(&instruction.param1), param_type(&instruction.param2))
}

pub fn assemble(ir: IR) -> Vec<u8> {
    let translation = AssemblyTranslation::new();
    let mut assembled = Vec::with_capacity(ir.instructions.len());

    // First scan to figure out size and assemble all but labels.
    for instruction in &ir.instructions {
        match instruction {
            IRLine::Ins(ins) => {
                assembled.push(translation.assemble_instruction(ins));
            }
            IRLine::Label(_) => {
                assembled.push(Vec::new());
            }
        }
    }

    // Add HALT to the end of the result if it is not present.
    match &ir.instructions.iter().last() {
        Some(IRLine::Ins(ins)) if ins.command == IRCommand::Halt => {}
        _ => assembled.push(vec![0x00, 0x00, 0x00, HALT_INSTRUCTION])
    }

    // TODO: Handle labels (use a second scan to do so)

    assembled.into_iter().flatten().collect()
}
