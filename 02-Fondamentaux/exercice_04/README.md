## Exercice 4

1 ) On a une fonction `square` qui calcul le carré d'un nombre, on souhaite appeler cette fonction avec `n`, compléter le code pour y parvenir.

```rust
fn square(x: u64) -> u64 {
    x * x
}

fn main() {
    let n: u16 = 100;

    let result = square(n as u64);
}
```

2 ) Même chose ici avec `x` et `y` et la fonction `mean` qui calcul la moyenne de 2 nombres flottants.

```rust
fn mean(a: f64, b: f64) -> f64 {
    (a + b) / 2.0
}

fn main() {
    let x: i32 = 10;
    let y: i32 = 20;

    let result = mean(x as f64, y as f64);
}
```