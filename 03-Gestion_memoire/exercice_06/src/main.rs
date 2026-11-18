fn main() {

    // À retenir
    // - Longueur (len) : nombre d'éléments réellement présents.
    // - Capacité (capacity) : nombre d'éléments que le vecteur peut accueillir sans agrandir son stockage.
    // - Allocation : réservation de mémoire.
    // - Réallocation : agrandissement du stockage lorsque la capacité est insuffisante.
    // - Move : v.push(text) transfère la propriété de la String au vecteur.

    let mut v: Vec<String> = vec![
        "Bonsoir".to_string(),
        "Hello".to_string()
    ];

    let text = "Super Michel".to_string();
    v.push(text);

    for i in 0..7 {
        v.push("Coucou".to_string());
    }

    dbg!(v.len());      // 10
    dbg!(v.capacity()); // Généralement 16
}