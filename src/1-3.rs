fn main()
{
    let mut lower:   f32 =   0.0; /* lower limit of temperature table */
    let mut upper:   f32 = 300.0; /* upper limit */
    let     step:    f32 =  20.0; /* step size */
    let mut fahr:    f32 = lower;
    let mut celsius: f32 =   0.0;

    println!("\n🌡️Temperature");
    println!("+---+------+");
    println!("+°F +  °C  +");
    println!("+---+------+");
    while fahr <= upper {
        celsius = (5.0/9.0) * (fahr - 32.0);
        println!("|{:>3.0}|{:>6.1}|", fahr, celsius);
        fahr += step;
    }
    println!("+---+------+");
}
