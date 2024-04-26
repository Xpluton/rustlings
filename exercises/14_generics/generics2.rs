// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.

/// Nous déclarons une structure Wrapper générique qui prend un type générique T.
/// La méthode new crée une nouvelle instance de Wrapper avec une valeur de type T.
/// Nous retournons une nouvelle instance de Wrapper avec la valeur donnée.

struct Wrapper<T> {
    value: T,
}

// Implémentation de méthodes pour la structure Wrapper.
impl<T> Wrapper<T> {

    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
