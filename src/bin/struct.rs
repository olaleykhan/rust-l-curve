// create a flavour  enum
// create our onbject structure
// init the objects in 
// print using match syntax

// creating enum

enum FlavourType {
    Chocolate, Cola, Vanilla
}

struct Drink{
    flavour: FlavourType,
    cost: f64,
}

fn print_drink(drink:Drink){

    println!(" the cost of this drink is {:?}", drink.cost);

    match drink.flavour{
        FlavourType::Chocolate => println!(" the flavour of the drink is Chocolate"),
        FlavourType::Cola => println!(" the flavour of the drink is Cola"),
        FlavourType::Vanilla => println!(" the flavour of the drink is Vanilla"),
    }
}

fn main(){
    let pepsi =  Drink{
        flavour:FlavourType::Cola,
        cost: 150.00
    };

    println!("Hello");
    print_drink(pepsi)


}