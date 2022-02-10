use std::collections::{HashMap, HashSet};

use crate::ir::{IRCommand, IRInstruction, IRLine, IRParamType, IR, IRParameter};

type InstructionSignature = (IRCommand, IRParamType, IRParamType);

struct AssemblyTranslation {
    instructions: HashMap<InstructionSignature, u8>,

    /// Indicates which instructions have byte sized immediates
    /// as parameters (instead of 32 bit immediates).
    byte_immediates: HashSet<InstructionSignature>,
}

struct AssembleInstruction<'a> {
    instruction: &'a IRInstruction,
    assembled: Vec<u8>,
    next_register: usize,
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

        let byte_immediates = HashSet::from([
            (IRCommand::Shl, IRParamType::Register, IRParamType::Immediate),
            (IRCommand::Shr, IRParamType::Register, IRParamType::Immediate)
        ]);

        AssemblyTranslation {
            instructions,
            byte_immediates,
        }
    }

    fn assemble_instruction(&self, instruction: &IRInstruction) -> Vec<u8> {
        let mut assemble = AssembleInstruction::new(instruction);

        // Encode instruction type byte.
        let instruction_signature = instruction_signature(instruction);
        let encoding = self.instructions.get(&instruction_signature);
        let encoding = match encoding {
            Some(&instruction_byte) => {
                instruction_byte
            }
            None => {
                panic!("Instruction {:?} has no variation with parameters {:?},{:?} (on line {})", instruction_signature.0, instruction_signature.1, instruction_signature.2, instruction.line_number);
            }
        };

        let byte_immediates = self.byte_immediates.contains(&instruction_signature);
        assemble.assemble(encoding, byte_immediates);

        assemble.assembled
    }
}

impl<'a> AssembleInstruction<'a> {
    fn new(instruction: &'a IRInstruction) -> AssembleInstruction<'a> {
        AssembleInstruction {
            instruction,
            assembled: vec![0u8; 4],
            next_register: 2,
        }
    }

    fn assemble(&mut self, encoding: u8, byte_immediates: bool) {
        self.assembled[3] = encoding;
        self.assemble_parameter(&self.instruction.param1, byte_immediates);
        self.assemble_parameter(&self.instruction.param2, byte_immediates);
    }

    fn assemble_parameter(&mut self, param: &Option<IRParameter>, byte_immediates: bool) {
        match param {
            Some(IRParameter::Reg(register)) | Some(IRParameter::MemReg(register)) => {
                self.add_register_value(*register as u8);
            }
            Some(IRParameter::Imm(value)) | Some(IRParameter::MemImm(value)) => {
                let value = *value;
                if !byte_immediates {
                    self.assembled.extend_from_slice(&value.to_be_bytes());
                } else {
                    if value < u8::MIN.into() || value > u8::MAX.into() {
                        eprintln!("[Warning] Instruction {:?} has has parameter {} that eceeds byte range of [0..256) on line {}", self.instruction.command, value, self.instruction.line_number);
                    }

                    self.add_register_value(value as u8);
                }
            }
            _ => {}
        }
    }

    fn add_register_value(&mut self, value: u8) {
        self.assembled[self.next_register] = value;
        self.next_register -= 1;
    }
}

fn param_type(param: &Option<IRParameter>) -> IRParamType {
    match param {
        Some(p) => p.param_type(),
        None => IRParamType::None,
    }
}

fn instruction_signature(instruction: &IRInstruction) -> InstructionSignature {
    (instruction.command.clone(), param_type(&instruction.param1), param_type(&instruction.param2))
}

pub fn assemble(ir: IR) -> Vec<u8> {
    let translation = AssemblyTranslation::new();
    let mut assembled = Vec::with_capacity(ir.instructions.len());
    let mut label_locations = HashMap::new();

    // First scan to figure out size and assemble all but labels.
    let mut location = 0;
    for instruction in &ir.instructions {
        match instruction {
            IRLine::Ins(ins) => {
                let translated = translation.assemble_instruction(ins);
                location += translated.len() / 4;
                assembled.push(translated);
            }
            IRLine::Label(label) => {
                if label_locations.insert(label, location).is_some() {
                    panic!("Found duplicate label: '{}'", label);
                }
            }
        }
    }

    // Add HALT to the end of the result if it is not present.
    match &ir.instructions.iter().last() {
        Some(IRLine::Ins(ins)) if ins.command == IRCommand::Halt => {}
        _ => assembled.push(vec![0x00, 0x00, 0x00, HALT_INSTRUCTION])
    }

    // Second scan to add location to jump instructions.
    let mut location = 0;
    let mut assembled_index = 0;
    for instruction in &ir.instructions {
        match instruction {
            IRLine::Ins(ins) => {
                if let Some(IRParameter::Label(target_label)) = &ins.param1 {
                    let label_location = match label_locations.get(target_label) {
                        Some(location) => *location,
                        None => panic!("Did not find target label '{}'", target_label),
                    };

                    let location_difference = (label_location as i32) - (location as i32);
                    let encoded_location = location_difference.to_be_bytes();
                    for i in 0..3 {
                        assembled[assembled_index][i] = encoded_location[i+1];
                    }
                }

                location += assembled[assembled_index].len() / 4;
                assembled_index += 1;
            }
            _ => {}
        }
    }


    assembled.into_iter().flatten().collect()
}
