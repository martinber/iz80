use super::decoder::*;
use super::memory_io::*;
use super::state::*;

pub struct Cpu {
    pub state: State,
    pub decoder: Decoder,
}

impl Cpu {
    pub fn new(memory: Box<dyn Memory>, io: Box<dyn Io>) -> Cpu {
        Cpu {
            state: State::new(memory, io),
            decoder: Decoder::new()
        }
    }

    pub fn new_plain() -> Cpu {
        let memory = Box::new(PlainMemoryIo::new());
        let io = Box::new(PlainMemoryIo::new());
        Cpu {
            state: State::new(
                memory,
                io),
            decoder: Decoder::new()
        }
    }

    pub fn execute_instruction(&mut self) {
        let pc = self.state.reg.get_pc();
        let opcode_index = self.state.peek_pc();
        print!("==== {:04x}: {:02x} ", pc, opcode_index);
        let opcode = self.decoder.decode(&mut self.state);
        println!("{}", opcode.disasm(&self.state));
        opcode.execute(&mut self.state)
    }
}


