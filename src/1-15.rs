fn to_celsius (fahr: f32) -> f32 {
    5.0/9.0*(fahr - 32.0)
}

fn to_fahr (celsius: f32) -> f32 {
    celsius / (5.0/9.0) + 32.0
}

fn main() {
    let lower: f32 = 0.0;
    let upper: f32 = 300.0;
    let step:  f32 = 20.0;
    let mut celsius: f32 = lower;
    let mut fahr:    f32;

    println!("\n🌡️Temperature");
    println!("+---+------+");
    println!("+°C +  °F  +");
    println!("+---+------+");
    while celsius <= upper {
        fahr = to_fahr(celsius);
        println!("|{:>3.0}|{:>6.1}|", celsius, fahr);
        celsius += step;
    }
    println!("+---+------+");

    fahr = lower;
    println!("\n🌡️Temperature");
    println!("+---+------+");
    println!("+°F +  °C  +");
    println!("+---+------+");
    while fahr <= upper {
        celsius = to_celsius(fahr);
        println!("|{:>3.0}|{:>6.1}|", fahr, celsius);
        fahr += step;
    }
    println!("+---+------+");
}