// tests4.rs
//
// Make sure that we're testing for the correct conditions!
//
// Execute `rustlings hint tests4` or use the `hint` watch subcommand for a
// hint.

struct Rectangle {
    width: i32,
    height: i32
}

impl Rectangle {
    // Only change the test functions themselves
    pub fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            panic!("Rectangle width and height cannot be negative!")
        }
        Rectangle {width, height}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        // This test should check if the rectangle is the size that we pass into its constructor
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10); // check width
        assert_eq!(rect.height, 20); // check height
    }

    #[test]
    fn negative_width() {
        // This test should check if program panics when we try to create rectangle with negative width
        // Ce test doit vérifier si le programme panique lorsque nous essayons de créer un rectangle avec une hauteur négative.
        // Utilisation de l'attribut should_panic pour vérifier la panique attendue
        assert!(std::panic::catch_unwind(|| {
            let _rect = Rectangle::new(-10, 10);
        }).is_err());
    }

    #[test]
    fn negative_height() {
        // This test should check if program panics when we try to create rectangle with negative height
        // Ce test doit vérifier si le programme panique lorsque nous essayons de créer un rectangle avec une hauteur négative.
        // Utilisation de l'attribut should_panic pour vérifier la panique attendue
        assert!(std::panic::catch_unwind(|| {
            let _rect = Rectangle::new(10, -10);
        }).is_err());
    }
}