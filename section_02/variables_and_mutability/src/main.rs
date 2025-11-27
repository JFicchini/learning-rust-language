fn main() {
    let apples = 50;
    let oranges = 14 + 6;
    let _fruits = apples + oranges;

    println!(
        "\nThis year, my garden has {0} apples and {1} oranges.\nI can't believe I have {0} apples.\n",
        apples, oranges
    );

    // Mutable and imutable
    // Variables are imutable as default. If you want to change a variable turn to mutable with "mut"

    let mut gym_reps = 10;
    println!("I plan to do {gym_reps} reps");

    gym_reps = 15;
    println!("I now plan to do {gym_reps} reps\n");

    /*
    Check documentation to see all Errors at Rust.
    To see the error and solution at terminal run 'rustc --explain E0384(ERROR_NUMBER)'
    */

}
