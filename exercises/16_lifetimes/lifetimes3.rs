// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a
// hint.

/// Définition d'une structure Book qui contient deux champs de référence avec un lifetime 'a.
/// Cela signifie que les références contenues dans une instance de Book doivent vivre au moins aussi longtemps que 'a.
/// Champ 'author' contient une référence à une chaîne de caractères avec un lifetime 'a.
/// Champ 'title' contient une référence à une chaîne de caractères avec un lifetime 'a.



struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}
