pub mod number_converter {
    pub fn from_dec() {
        from_number_system(NumberSystem::Dec);
    }

    pub fn from_hex() {
        from_number_system(NumberSystem::Hex);
    }

    pub fn from_bin() {
        from_number_system(NumberSystem::Bin);
    }

    enum NumberSystem {
        Dec,
        Hex,
        Bin,
    }

    fn from_number_system(ns: NumberSystem) {
        use std::io::{self, Write};
        loop {
            print!(
                "{}: ",
                match ns {
                    NumberSystem::Dec => "Dec",
                    NumberSystem::Hex => "Hex",
                    NumberSystem::Bin => "Bin",
                }
            );
            io::stdout().flush().unwrap();

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                println!("Failed to read line.");
                return;
            }
            let input_num = input.trim();
            if input_num.is_empty() {
                return;
            }

            let result = match ns {
                NumberSystem::Dec => match input_num.parse::<u32>() {
                    Ok(num) => Some((num, format!("Hex: 0x{:X}, Bin: 0b{:b}", num, num))),
                    Err(_) => None,
                },
                NumberSystem::Hex | NumberSystem::Bin => {
                    let radix = if let NumberSystem::Hex = ns { 16 } else { 2 };
                    match u32::from_str_radix(input_num, radix) {
                        Ok(num) => Some(match ns {
                            NumberSystem::Hex => (num, format!("Dec: {}, Bin: 0b{:b}", num, num)),
                            NumberSystem::Bin => (num, format!("Dec: {}, Hex: 0x{:X}", num, num)),
                            _ => unreachable!(),
                        }),
                        Err(_) => None,
                    }
                }
            };

            if let Some((_, message)) = result {
                println!("{}", message);
            } else {
                println!("Invalid number, please try again.");
                continue;
            }
        }
    }
}

pub fn number_representations() {
    let hex_num = 0xFF;
    let bin_num = 0b1111_1111;
    let dec_num = 255;
    assert!(hex_num == bin_num);
    assert!(bin_num == dec_num);
    println!(
        "Dec: {}, Hex: 0x{:X}, Bin: 0b{:b}",
        dec_num, hex_num, bin_num
    );
}
