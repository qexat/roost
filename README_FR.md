# Roost

Roost est un générateur d'erreurs de Rust écrit en Python. C'est pour faire des blagues.

C'est un simple script que j'ai écrit en genre 2 heures donc ne vous attendez pas à ce que la qualité du code soit bonne 😆.

## Usage

### Compiler

```
cargo build --release
```

### Exécuter

Dans l'émulateur de terminal, écrivez :

```
./target/release/roost
```

Ensuite, remplissez les champs ; le message d'erreur sera affiché à la fin.

### L'option `--output`

Vous pouvez écrire le message d'erreur (avec les séquences d'échappement ANSI) dans un fichier.

Pour ce faire, vous avez simplement à fournir un chemin de fichier valide après l'argument, tel que :

```
./target/release/roost --output /chemin/vers/mon_fichier.txt
```

## Capture d'écran

![example_fr.png](./images/example_fr.png)
