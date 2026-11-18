Exercices Chapitre 3

Exercice 1

1 ) Imaginons qu'au lancement du programme le RSP soit l'adresse hexadécimale 0x00007FFDD4A3B8C0, sachant que l'on est sur un OS ou la taille de la stack est de 8Mo, quelle est la dernière adresse disponible de la stack ?

2 ) Même question mais avec une taille de stack de 1Mo ?

3 ) Quelle est l'adresse mémoire hexadécimale correspondant à rsp - 3 ?

4 ) Comment s'écrit cette adresse mémoire en binaire ?

5 ) Combien de bits occupe une adresse mémoire ?


# Correction Exercices Chapitre 3

## Exercice 1

1 ) Imaginons qu'au lancement du programme le RSP soit l'adresse hexadécimale `0x00007FFDD4A3B8C0`, sachant que l'on est sur un OS ou la taille de la stack est de 8Mo, quelle est la dernière adresse disponible de la stack ?

RSP = `0x00007FFDD4A3B8C0`, taille de stack = 8 Mo = `0x7A1200`. La pile grandit vers les adresses basses, donc la dernière adresse disponible est :

```
0x00007FFDD4A3B8C0 - 0x00000000007A1200 = 0x00007FFDD429A6C0
```

2 ) Même question mais avec une taille de stack de 1Mo ?

Avec une taille de stack de 1 Mo = `0x0F4240` :

```
0x00007FFDD4A3B8C0 - 0x00000000000F4240 = 0x00007FFDD4947680
```

3 ) Quelle est l'adresse mémoire hexadécimale correspondant à `rsp - 3` ?

`rsp - 3` :

```
0x00007FFDD4A3B8C0 - 0x3 = 0x00007FFDD4A3B8BD
```

4 ) Comment s'écrit cette adresse mémoire en binaire ?

On converti d'abord en décimal ce qui nous donne `140728170952893` puis on va convertir en binaire ce qui nous donne `0000 0000 0000 0000 0111 1111 1111 1101 1101 0100 1010 0011 1011 1000 1011 1101` , une autre possibilité est de transformer chaque chiffre du nombre hexadecimal en un nombre binaire sur 4 bits.

5 ) Combien de bits occupe une adresse mémoire ?

Une adresse mémoire occupe **64 bits** sur une architecture 64 bits et 32bits sur une architecture 32bits.