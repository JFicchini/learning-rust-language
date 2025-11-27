/*A constant is a name assigned to a value.
A constant's value cannot change.
The difference into variables and constants are two:
1 - Variables are limited to a function scope
    Constants can be declared at any scope include at the file level.
2 - The constants value must be known at compile time.
    A variable value is defined at the run time.
*/
const TAX_RATE: f64 = 7.25;

fn main() {
    let income = 100000;
    println!("My income is {income} and my tax rate is {TAX_RATE}");
}
