// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
// No hints.

/// J'ai utilisé `if let Some(_) = my_option` pour vérifier si `my_option` contient une valeur.
/// Correction de la virgule manquante dans la déclaration de tableau.
/// j'ai utilisé std::mem::swap pour échanger les valeurs a et b
/// Utilisation de Vec::new() pour instencier un nouveau vecteur
#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_some() {
        panic!("This should never happen!");
    }

    
    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6,
    ];
    println!("My array! Here it is: {:?}", my_arr);

    let my_empty_vec: Vec<i32> = vec![];
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!

    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}

