## Exercice 6

1 ) Créer un module `draw` qui contient une fonction `draw_triangle` qui va dessiner un triangle de hauteur `h` dans le terminal. Ainsi `draw_triangle(3)` va afficher un triangle de 3 étages :

```ansi
  *
 ***
*****
```

**Réponse :**

```rust
mod draw {
    pub fn draw_triangle(height: u32) {
        for i in 0..height {
            let spaces = height - i - 1;
            let stars = 2 * i + 1;

            println!(
                "{}{}",
                " ".repeat(spaces as usize),
                "*".repeat(stars as usize)
            );
        }
    }
}
```



2 ) Créer un fichier `draw.rs` qui contiendra la logique de dessin puis l'utiliser dans la fonction main

**draw.rs**

```rust
pub fn draw_triangle(height: u32) {
    for i in 0..height {
        let spaces = height - i - 1;
        let stars = 2 * i + 1;

        println!(
            "{}{}",
            " ".repeat(spaces as usize),
            "*".repeat(stars as usize)
        );
    }
}
```

**main.rs**

```rust
mod draw;

fn main() {
    draw::draw_triangle(3);
}
```



3 ) Créer un dossier `geometry` qui contiendra un fichier `draw.rs` avec la logique pour `draw_triangle` et un fichier `math.rs` avec une fonction `triangle_area` qui calcule l'aire du triangle dessiné précédemment, on suppose le triangle équilatéral.

On suppose que le symbole `*` équivaut à une unité.

**Arborescence :**

```text
src/
├── main.rs
└── geometry/
    ├── mod.rs
    ├── draw.rs
    └── math.rs
```

**geometry/mod.rs**

```rust
pub mod draw;
pub mod math;
```

**geometry/draw.rs**

```rust
pub fn draw_triangle(height: u32) {
    for i in 0..height {
        let spaces = height - i - 1;
        let stars = 2 * i + 1;

        println!(
            "{}{}",
            " ".repeat(spaces as usize),
            "*".repeat(stars as usize)
        );
    }
}
```

**geometry/math.rs**

```rust
pub fn triangle_area(side: f64) -> f64 {
    (3.0.sqrt() / 4.0) * side * side
}
```

**main.rs**

```rust
mod geometry;

fn main() {
    geometry::draw::draw_triangle(4);

    let area = geometry::math::triangle_area(4.0);

    println!("Area = {}", area);
}
```