pub fn main() -> Result<(), String> {
    let input = crate::load_input()?;

    let fuel = input.lines()
        .map( |line| u32::from_str_radix( line, 10 ).unwrap() ) // parse lines into u32's
        .map( |module| ( module / 3 ) - 2 ) // calculate fuel
        .fold( 0, |acc, fuel| acc + fuel ); // sum fuel
    
    println!( "required fuel: {}", fuel );
    Ok(())
}