// A - address register, D - data register, PC - program counter register, harvard architecture
pub struct Hack {
    a: i16,
    d: i16,
    pc: u16,
    rom: [i16; 65536],
    pub ram: [i16; 65536],
}

#[derive(PartialEq, Debug)]
struct DecoderOutput {
    negate_output: bool,
    function: bool,
    negate_x: bool,
    zero_x: bool,
    negate_y: bool,
    zero_y: bool,
    load_a: bool,
    load_d: bool,
    a_or_ram: bool,
    alu_or_rom: bool,
    ram_storage: bool,
}

impl Hack {
    pub fn new() -> Self {
        Hack {
            a: 0,
            d: 0,
            pc: 0,
            rom: [0; 65536],
            ram: [0; 65536],
        }
    }

    // evil bit manipulation
    fn decoder(instruction: i16) -> DecoderOutput {
        DecoderOutput {
            negate_output: ((0b0000_0000_0100_0000 & instruction) != 0),
            function: ((0b0000_0000_1000_0000 & instruction) != 0),
            negate_y: ((0b0000_0001_0000_0000 & instruction) != 0),
            zero_y: ((0b0000_0010_0000_0000 & instruction) != 0),
            negate_x: ((0b0000_0100_0000_0000 & instruction) != 0),
            zero_x: ((0b0000_1000_0000_0000 & instruction) != 0),
            load_a: (-(!((-((instruction < 0) as i16))
                & (!(-((0b0000_0000_0010_0000 & instruction) >> 5)))))
                != 0),
            load_d: (((instruction < 0) as i16) & ((0b0000_0000_0001_0000 & instruction) >> 4))
                != 0,
            a_or_ram: ((((instruction < 0) as i16)
                & ((0b0001_0000_0000_0000 & instruction) >> 12))
                == 0),
            alu_or_rom: instruction < 0,
            ram_storage: (((instruction < 0) as i16) & ((0b000000000001000 & instruction) >> 3))
                != 0,
        }
    }

    fn alu(x: i16, y: i16, control: &DecoderOutput) -> i16 {
        // negate or zero inputs if requested
        let x1 = (x & (!(-(control.zero_x as i16)))) ^ (-(control.negate_x as i16));
        let y1 = (y & (!(-(control.zero_y as i16)))) ^ (-(control.negate_y as i16));

        //add or and
        let out = if control.function {
            x1.overflowing_add(y1).0
        } else {
            x1 & y1
        };

        //negating output if requested
        out ^ (-(control.negate_output as i16))
    }

    pub fn tick(&mut self) {
        // get current instruction at address taken from Program Counter register
        let current_instruction = self.rom[self.pc as usize];

        // decoding current instruction
        let decoder = Hack::decoder(current_instruction);

        // first alu input is Data register
        let x = self.d;

        // choosing second alu input; Address register or RAM at Address register
        let y = if decoder.a_or_ram {
            self.a
        } else {
            self.ram[self.a as usize]
        };

        // do the math and get ALU output
        let alu_output = Hack::alu(x, y, &decoder);

        // write to RAM at Address register if requested
        if decoder.ram_storage {
            self.ram[self.a as usize] = alu_output;
        }

        // load Data register with ALU output if requested
        if decoder.load_d {
            self.d = alu_output;
        }

        // jump logic, based of alu output, jump to Address register value or increment Program Counter register
        if ((current_instruction as u16) & 0b1000000000000111) > 0 {
            if (((0b1000000000000001 & (current_instruction as u16)) == 0b1000000000000001)
                && (alu_output > 0))
                || (((0b1000000000000010 & (current_instruction as u16)) == 0b1000000000000010)
                    && (alu_output == 0))
                || (((0b1000000000000100 & (current_instruction as u16)) == 0b1000000000000100)
                    && (alu_output < 0))
            {
                self.pc = self.a as u16;
            } else {
                self.pc += 1;
            }
        } else {
            self.pc += 1;
        }

        // load Address register with ALU output or, if requested to load some data from ROM into Address register, load Address register with current instruction (not the next)
        if decoder.load_a {
            if decoder.alu_or_rom {
                self.a = alu_output;
            } else {
                self.a = current_instruction;
            }
        }
    }

    ///Reset computer state
    pub fn reset(&mut self) {
        self.a = 0;
        self.d = 0;
        self.pc = 0;
        self.ram.iter_mut().for_each(|m| *m = 0);
    }

    /// Load ROM into computer. Should be backed by reset
    pub fn load_rom(&mut self, file: String) {
        file.lines()
            .enumerate()
            .for_each(|line| self.rom[line.0] = u16::from_str_radix(line.1, 16).unwrap() as i16);
    }

    // run computer for 1/60 second, then return how many ticks roughly has been done in one second
    pub fn update(&mut self) -> i32 {
        // handle frequency
        use std::time::Instant;
        let t0 = Instant::now();

        let mut counter = 0;

        // ticking for ~ 1/60 second
        while (Instant::now() - t0).subsec_millis() < 16 {
            self.tick();

            // count ticks
            counter += 1;
        }

        // return frequency
        counter * 1000 / 16
    }

    // draw screen memory map to physical screen buffer
    pub fn draw(&self, frame: &mut [u8]) {
        // hack screen is memory mapped; each bit in RAM 16384..24576 range encode color of one pixel (black/white), thus Hack screen is 512x256, because (24576-16384)*16 = 512*256
        self.ram[16384..24576]
            .iter()
            .flat_map(
                |memcell| {
                    (0..16)
                        .map(|x| ((1_i16 << x) & *memcell) != 0)
                        .collect::<Vec<bool>>()
                }, //apply mask of consequent bits with current memory cell and aggregate
            )
            .enumerate()
            .for_each(|pixel| {
                if pixel.1 {
                    // if current pixel has "true" value, then put black color in buffer
                    for i in 0..3 {
                        frame[pixel.0 * 4 + i] = 0x0; //
                    }
                } else {
                    for i in 0..3 {
                        frame[pixel.0 * 4 + i] = 0xFF;
                    }
                }

                frame[pixel.0 * 4 + 3] = 0xFF;
            });
    }
}

/////////////////////////////////////////// tests

#[cfg(test)]
mod tests {
    use super::*;

    fn decoder_output_construct(
        zero_x: bool,
        negate_x: bool,
        zero_y: bool,
        negate_y: bool,
        function: bool,
        negate_output: bool,
    ) -> DecoderOutput {
        DecoderOutput {
            negate_output,
            function,
            negate_y,
            zero_y,
            negate_x,
            zero_x,
            load_a: false,
            load_d: false,
            a_or_ram: false,
            alu_or_rom: false,
            ram_storage: false,
        }
    }

    #[test]
    fn alu_test() {
        assert_eq!(
            Hack::alu(
                123,
                456,
                &decoder_output_construct(false, false, false, false, false, false)
            ),
            123 & 456
        );
        assert_eq!(
            Hack::alu(
                123,
                456,
                &decoder_output_construct(true, false, true, false, true, false)
            ),
            0
        );
        assert_eq!(
            Hack::alu(
                123,
                456,
                &decoder_output_construct(true, true, true, true, true, true)
            ),
            1
        );
        assert_eq!(
            Hack::alu(
                123,
                456,
                &decoder_output_construct(true, true, true, false, true, false)
            ),
            -1
        );
        assert_eq!(
            Hack::alu(
                123,
                456,
                &decoder_output_construct(false, false, true, true, false, false)
            ),
            123
        );
        assert_eq!(
            Hack::alu(
                123,
                456,
                &decoder_output_construct(true, true, false, false, false, false)
            ),
            456
        );
        assert_eq!(
            Hack::alu(
                123,
                456,
                &decoder_output_construct(false, false, true, true, false, true)
            ),
            !123
        );
        assert_eq!(
            Hack::alu(
                123,
                456,
                &decoder_output_construct(true, true, false, false, false, true)
            ),
            !456
        );
        assert_eq!(
            Hack::alu(
                123,
                456,
                &decoder_output_construct(false, false, true, true, true, true)
            ),
            -123
        );
        assert_eq!(
            Hack::alu(
                123,
                456,
                &decoder_output_construct(false, true, true, true, true, true)
            ),
            123 + 1
        );
    }

    #[test]
    fn decoder_test() {
        assert_eq!(
            Hack::decoder(132),
            DecoderOutput {
                negate_output: false,
                function: true,
                negate_y: false,
                zero_y: false,
                negate_x: false,
                zero_x: false,
                load_a: true,
                load_d: false,
                a_or_ram: true,
                alu_or_rom: false,
                ram_storage: false
            }
        );
        assert_eq!(
            Hack::decoder(-5497),
            DecoderOutput {
                negate_output: false,
                function: true,
                negate_y: false,
                zero_y: true,
                negate_x: false,
                zero_x: true,
                load_a: false,
                load_d: false,
                a_or_ram: true,
                alu_or_rom: true,
                ram_storage: false
            }
        );
        assert_eq!(
            Hack::decoder(23030),
            DecoderOutput {
                negate_output: true,
                function: true,
                negate_y: true,
                zero_y: false,
                negate_x: false,
                zero_x: true,
                load_a: true,
                load_d: false,
                a_or_ram: true,
                alu_or_rom: false,
                ram_storage: false
            }
        );
        assert_eq!(
            Hack::decoder(0),
            DecoderOutput {
                negate_output: false,
                function: false,
                negate_y: false,
                zero_y: false,
                negate_x: false,
                zero_x: false,
                load_a: true,
                load_d: false,
                a_or_ram: true,
                alu_or_rom: false,
                ram_storage: false
            }
        );
    }
}
