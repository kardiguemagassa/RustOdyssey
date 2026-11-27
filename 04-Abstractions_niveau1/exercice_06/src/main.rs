// 1 ) L'état d'une connexion réseau peut être : déconnecté, en connexion, connecté (avec une adresse IP),
// ou en erreur (avec un message). Modéliser cet état en Rust.

enum ConnectionState {
    Disconnected,
    Connecting,
    Connected(String),
    Error(String),
}
// Ces quatre états sont "mutuellement exclusifs" : une connexion ne peut pas être
// à la fois "déconnectée" et "connectée". C'est précisément le cas d'usage d'un enum.


//2 ) Une carte à jouer a une couleur (cœur, carreau, trèfle, pique) et une valeur
// (2 à 10, valet, dame, roi, as). Modéliser une carte en Rust.
enum Suit {
    Heart,
    Diamond,
    Club,
    Spade,
}

enum Rank {
    Number(u8), // 2 à 10
    Jack,
    Queen,
    King,
    Ace,
}

struct Card {
    suit: Suit,
    rank: Rank,
}
// Ici, contrairement au cas précédent, couleur et valeur coexistent toujours en même temps sur
// une même carte : c'est pour cela que Card est un struct qui regroupe deux enums, plutôt qu'un unique enum.


// 3 ) Le résultat d'une requête HTTP est soit un succès avec un corps de réponse, soit une erreur avec
// un code et un message. Modéliser ce résultat en Rust.
enum HttpResponse {
    Success(String),
    Error { code: u16, message: String },
}

// 4 ) Un utilisateur a un nom, un email, et un rôle qui est soit administrateur, soit membre
// (avec une date d'inscription), soit invité (avec une date d'expiration). Modéliser cet utilisateur en Rust.
enum Role {
    Admin,
    Member(String), // date d'inscription
    Guest(String),  // date d'expiration
}

struct User {
    name: String,
    email: String,
    role: Role,
}
// name et email sont toujours présents ensemble (struct), tandis que role ne peut prendre qu'une forme
// à la fois avec des données différentes selon le cas (enum).

//5 ) Quel critère général permet de décider si une donnée doit être modélisée avec un enum plutôt qu'un struct ?

/*
Un enum est adapté quand une donnée ne peut prendre qu'une seule forme parmi plusieurs possibilités 
mutuellement exclusives, potentiellement avec des données différentes selon le cas choisi 
(comme un "OU" logique).

Un struct est adapté quand plusieurs informations coexistent toutes en même temps (comme un "ET" logique). 
Beaucoup de modèles réels combinent les deux : un struct qui contient un ou plusieurs champs de type enum.
 */

fn main() {
    println!("Hello, world!");
}
