## Exercice 3

1 ) Parmi les exemples suivants définir si les valeurs suivantes peuvent être connues à la compilation ou à l'exécution et expliquer pourquoi :

- Le résultat d'une requête HTTP

  **Exécution : dépend d'une ressource externe.**

- La valeur de Pi

  **Compilation : connue à l'avance.**

- L'heure actuelle

  **Exécution : dépend du moment où le programme s'exécute.**

- Le contenu d'un fichier

  **Exécution : peut changer après la compilation.**

- Le contenu d'une colonne dans une base de données

  **Exécution : dépend de l'état de la base de données.**

- Une clé privé GitHub

  **Compilation si elle est écrite directement dans le code, sinon exécution si elle est lue depuis un fichier ou une variable d'environnement.**



2 ) En déduire pour chacun des cas précédents si l'on peut utiliser la valeur en question pour une constante.

**Réponse :**

| Valeur                        | Constante possible ? |
| ----------------------------- | -------------------- |
| Résultat d'une requête HTTP   | Non                  |
| Pi                            | Oui                  |
| Heure actuelle                | Non                  |
| Contenu d'un fichier          | Non                  |
| Contenu d'une base de données | Non                  |
| Clé privée GitHub             | Oui si codée en dur  |



3 ) Créer une constante Pi et l'utiliser dans les fonctions suivante puis citer les avantages de cette nouvelle approche par rapport à l'ancienne :

```rust
const PI: f64 = 3.141592653589793;

fn circle_area(radius: f64) -> f64 {
    PI * radius * radius
}

fn sphere_volume(radius: f64) -> f64 {
    (4.0 / 3.0) * PI * radius * radius * radius
}
```

**Avantages :**

- Une seule source de vérité.
- Plus facile à maintenir.
- Évite les erreurs de copie.
- Rend le code plus lisible.