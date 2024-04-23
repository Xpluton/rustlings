// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num // j'ai supprimé le point virgule qui bloquait le calcul num * num, en l'enlevant cela effectue le calcul et retourne le résultat
}
