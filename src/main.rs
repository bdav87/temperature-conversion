use std::io;

fn main() {
    
    loop {
        let mut option = String::new();

        println!("Convert Temperature (enter Q to quit)");
        println!("Enter F to convert Fahrenheit to Celsius");
        println!("Enter C to convert Celsius to Fahrenheit");

        io::stdin().read_line(&mut option)
            .expect("Failed read line");
        
        option = option.trim().to_string();

        if option == "q" {
            break;
        } else if option.to_lowercase() == "f" || option.to_lowercase() == "c" {
            break enter_temp(option);
        }
    };

}

fn enter_temp(scale: String) {
    println!("Enter degree to convert:");
    let mut temp_to_convert = String::new();

    io::stdin().read_line(&mut temp_to_convert)
        .expect("Failed read line");
    
    let temp_to_convert: u32 = temp_to_convert.trim().parse()
        .expect("Please type a number");

    if scale == "f" {
        return f_to_c(temp_to_convert);
    } else {
        return c_to_f(temp_to_convert);
    }
}

fn f_to_c(temp: u32) {
   let result = (temp - 32) * 5/9;
   println!("{}F is {}C", temp, result); 
}

fn c_to_f(temp: u32) {
    let result = temp * 9/5 + 32;
    println!("{}C is {}F",temp, result);
}