use colored::Colorize;

pub struct Hex {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub fn parse(str: &str) -> Result<Hex, &'static str> {
    if str.len() != 6 {
        return Err("Color hex must be of length 6!");
    }

    let parsed = match u32::from_str_radix(str, 16) {
        Ok(value) => value,
        Err(_) => return Err("Could not parse color hex!"),
    };

    let [_, red, green, blue] = parsed.to_be_bytes();

    let hex = Hex { red, green, blue };

    Ok(hex)
}

pub fn print(colors: Vec<Hex>) {
    for i in 0..colors.len() {
        let hex = &colors[i];

        if i > 0 {
            println!();
        }

        println!(
            "\n{}:   {}\n{}: {}\n{}:  {}\n\n{}:  {}",
            "[R]ed".red().bold(),
            hex.red.to_string().bold(),
            "[G]reen".green().bold(),
            hex.green.to_string().bold(),
            "[B]lue".blue().bold(),
            hex.blue.to_string().bold(),
            " Color".bold(),
            "\u{2588}\u{2588}\u{2588}"
                .truecolor( hex.red, hex.green, hex.blue)
        );
    }
}