fn main() {
    //A scope is the boundary or region of code where a name is valid.
    //A block is the area between an opening curly brace and a closing curly brace

    let coffee_price = 5.99;

    {
        let coffee_price = 1.99;
        let cookie_price = 1.95;
        println!("The coffee price is {coffee_price}");
        println!("The cookie price is {cookie_price}");
        //can't use cookie_price and the new cooffee_price out of this scope as variable
    }
    println!("The coffee price is {coffee_price}");
}
