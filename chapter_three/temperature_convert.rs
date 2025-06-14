fn main() {
    let temperature = conv_fh_cs(32.0, "celsius");
    let temperature_one = conv_fh_cs(36.0, "celsius");
    println!("32f to celsius is {temperature}");
    println!("36f to celsius is {temperature_one}");
}

fn conv_fh_cs(temp: f32, base: &str) -> f32 {
    if base != "celsius" && base != "fahrenheidt" {
        panic!("second parameter must be celsius or fahrenheidt")
    }
    let rate_constant = if base == "celsius" {
        9.0 / 5.0
    } else {
        5.0 / 9.0
    };

    let first_operation = if base == "celsius" {
        temp * rate_constant
    } else {
        temp - 32.0
    };
    let second_operation = if base == "celsius" {
        first_operation + 32.0
    } else {
        first_operation * rate_constant
    };

    return second_operation;
}
