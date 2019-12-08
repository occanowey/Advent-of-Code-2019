pub fn part1() -> Result<(), String> {
    let input = crate::load_input()?;

    let fuel = input.lines()
        .map( |line| u32::from_str_radix( line, 10 ).unwrap() ) // parse lines into u32's
        .map( |module| ( module / 3 ) - 2 ) // calculate fuel
        .fold( 0, |acc, fuel| acc + fuel ); // sum fuel
    
    println!( "required fuel: {}", fuel );
    Ok(())
}

pub fn part2() -> Result<(), String> {
    let input = crate::load_input()?;

    let modules: Vec<i32> = input.lines()
        .map( |line| i32::from_str_radix( line, 10 ).unwrap() ) // parse lines into i32's
        .collect();

    let mut sum = 0;

    let mut next_mass = modules[0];
    let mut index = 1;

    loop {
        let fuel = ( next_mass / 3 ) - 2;

        if fuel <= 0 {
            if index >= modules.len() {
                break;
            }

            next_mass = modules[index];
            index += 1;
        } else {
            sum += fuel;
            next_mass = fuel;
        }
    }

    println!( "required fuel {}", sum );
    Ok(())
}