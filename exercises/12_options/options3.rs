// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y), // rajout de ref dans le SOome pour permettre de lié la valeur eviter de la déplacer dans un pointeur
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
