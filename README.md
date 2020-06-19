# RustyHack
A 16-bit computer emulator from [nand2tetris course](https://www.nand2tetris.org/). This implementation differs from the original specification by having extended rom/ram memory from 32K KB to 64K KB (you need a modified assembler to handle that additional memory range though).  
![Alt text](screenshot.jpg?raw=true)
## Usage: 
If you have [Rust](https://www.rust-lang.org/learn/get-started) installed, download repository, then type in console from top level folder of the project:  
`cargo run --release <file_from_rom_folder.hex>`  
example:  
`cargo run --release rom/Bichromia.hex`

## Example ROMs:
- [Bichromia](https://github.com/Acedio/nand2tetris/tree/master/09/Bichromia)
- [DrunkenSniper](https://github.com/leimao/Drunken_Sniper)
- [GASscroller](https://github.com/gav-/Nand2Tetris-Games_and_Demos)
- [Trig](http://nand2tetris-questions-and-answers-forum.32033.n3.nabble.com/Trigonometry-td4026900.html)
 
 If you want to include your awesome program, feel free to contact me!
