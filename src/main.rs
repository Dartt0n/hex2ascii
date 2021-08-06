use regex::Regex;
use std::env;

fn to_color_index(value: u8) -> u8 {
    if value < 48 {
        0
    } else if value < 115 {
        1
    } else {
        (value - 35) / 40
    }
}

fn abs_subtraction(a: u8, b: u8) -> u8 {
    if a < b {
        b - a
    } else {
        a - b
    }
}

fn dist_square(expected_red: u8, expected_green: u8, expected_blue: u8, red: u8, green: u8, blue: u8) -> u16 {
    abs_subtraction(expected_red, red) as u16 * abs_subtraction(expected_red, red) as u16
        + abs_subtraction(expected_green, green) as u16 * abs_subtraction(expected_green, green) as u16
        + abs_subtraction(expected_blue, blue) as u16 * abs_subtraction(expected_blue, blue) as u16
}

fn main() {
    let arg_option = env::args().nth(1);
    if arg_option.is_none() {
        println!("Please enter hex code");
        return;
    }
    
    let hex_value = arg_option.unwrap();
    let hex_code_regex = Regex::new(r"[A-Fa-f0-9]{6}").expect("Wrong regex for hex code");
    match hex_code_regex.shortest_match(&hex_value) {
        None => {
            println!("Wrong Format");
            return;
        }
        Some(match_length) => {
            if match_length != hex_value.len() {
                println!("Wrong Format");
                return;
            }
        }
    }
    

    let red = u8::from_str_radix(&hex_value[..2], 16).expect("Unable to convert hex string to i32");
    let green = u8::from_str_radix(&hex_value[2..4], 16).expect("Unable to convert hex string to i32");
    let blue = u8::from_str_radix(&hex_value[4..], 16).expect("Unable to convert hex string to i32");

    let red_index = to_color_index(red);
    let blue_index = to_color_index(blue);
    let green_index = to_color_index(green);

    let averange = ((red as u16 + green as u16 + blue as u16) / 3u16) as u8;

    let gray_index: u8 = if averange > 238 { 23 } else { (averange - 3) / 10 };
    let gray_value = gray_index * 10 + 8;
    
    let to_color_value = [0u8, 0x5f, 0x87, 0xaf, 0xd7, 0xff];
    let color_red = to_color_value[red_index as usize];
    let color_green = to_color_value[green_index as usize];
    let color_blue = to_color_value[blue_index as usize];

    let color_error = dist_square(color_red, color_green, color_blue, red, green, blue);
    let gray_error = dist_square(gray_value, gray_value, gray_value, red, green, blue);
    
    let ascii_color_number :u8 = if color_error <= gray_error {
        16 + (36 * red_index + 6 * green_index + blue_index)
    } else {
        232 + gray_index
    };
    println!("{}", ascii_color_number);
}
