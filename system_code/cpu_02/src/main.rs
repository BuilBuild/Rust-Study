/*
 * @Author: LeiJiulong
 * @Date: 2025-03-27 18:36:14
 * @LastEditors: LeiJiulong && lei15557570906@outlook.com
 * @LastEditTime: 2025-03-27 18:45:33
 * @Description: 
 */

struct CPU {
    registers: [u8; 16],
    position_in_memory: usize,
    memory: [u8; 0x1000];
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p+1] as u16;

        
    }
}

fn main() {
    println!("Hello, world!");
}
