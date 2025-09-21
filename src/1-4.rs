fn main()
{
    let mut lower:   f32 =   0.0; /* lower limit of temperature table */
    let mut upper:   f32 = 300.0; /* upper limit */
    let mut step:    f32 =  20.0; /* step size */
    let mut celsius: f32 = lower;
    let mut fahr:    f32 =   0.0;

    println!("\n🌡️Temperature");
    println!("+---+------+");
    println!("+°C +  °F  +");
    println!("+---+------+");
    while celsius <= upper {
        fahr = celsius / (5.0/9.0) + 32.0;
        println!("|{:>3.0}|{:>6.1}|", celsius, fahr);
        celsius += step;
    }
    println!("+---+------+");
}