fn is_even(num:i64) -> bool {
    num % 2 == 0
}

// assinatura da funcao serve pra dizer qual tipo de valor a funcao devolve
fn sale_price(price: i64) -> i64 {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn main() {
    let original_price = 50;
    println!("Your sale price is {}.", sale_price(original_price));
}
