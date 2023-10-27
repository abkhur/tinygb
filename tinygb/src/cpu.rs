impl CPU {
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => {
                match target {
                    ArithmeticTarget::C => {
                        //todo: implement add on reg. C
                    }
                    _ => {/*todo: support more targets*/}
                }
            }
            _ => {/*todo: support more instructions*/}
        }
    }
}