fn main() {
    let x: i8 = 127;
    println!("The value of x is {x}");
    let x: i8 = 16;
    println!("Now it is {x}");
    
    // So, in the rust book, it says that the value of x needs to be muted to be
    // able to change it without let. But when I try to do that it says I need to 
    // use let. But if I use let, I don't need mut and if i remove mut. It becomes
    // shadowing. Can anyone explain this to me?

    const HIGHEST_I8_NUMBER: u8 = 255;
    println!("This is value is a const = {HIGHEST_I8_NUMBER}");
}
