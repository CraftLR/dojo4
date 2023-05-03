# Quatrième dojo (04/05/23) : kata LCD en Rust

## Ouvrir le code

Ce dépôt contient la configuration nécessaire pour faire le kata directement dans un devcontainer. Le dossier peut être ouvert directement dans un VSCode avec l'extension `Dev Containers` installée.

Une fois le projet chargé, vous pouvez modifier le programme principal situé dans le fichier `src/main.rs`. Pour lancer le programme, il suffit d'exécuter la commande suivante :

```bash
cargo run
```

## Lancer les tests

Pour exécuter les tests, lancer la commande suivante :

```bash
cargo test
```

Tous les tests sont ignorés par défault dans ce dépôt. Après avoir fait passé le premier test, ouvrir le fichier `dojo4.rs` situé dans le dossier `tests` et enlever l'annotation `#[ignore]` sur le second test et relancer la commande précédente pour voir le résultat. Comme le nom du test l'indique, le test va échouer et cela montrera que votre système de test fait bien son travail. Un test est une fonction précédée d'une annotation `#[test]`. Pour résoudre notre kata, nous devrons en ajouter plusieurs.

Pour lancer _uniquement_ les tests ignorés sans éditer le fichier, vous pouvez lancer la commande suivante :

```bash
cargo test -- --ignored
```

Pour lancer _tous_ les tests, vous pouvez utiliser la commande suivante :

```bash
cargo test -- --include-ignored
```

Pour lancer uniquement un test spécifique, par exemple `useless_test`, utilisez cette commande :

```bash
cargo test useless_test
```

Si le test à lancer est _ignoré_, vous pouvez lancer la commande suivante :

```bash
cargo test always_fail -- --ignored
```

Pour en apprendre plus sur les tests en Rust, référez vous à la [documentation des tests][rust-tests].

[rust-tests]: https://doc.rust-lang.org/book/ch11-00-testing.html

## Énoncé du problème

**Objectif : écrire un programme qui affiche des nombres de type LCD.**

### Partie 1

Ecrivez un programme qui, étant donné un nombre (avec un nombre arbitraire de chiffres), le convertit en nombres de style LCD en utilisant le format suivant :

```
   _  _     _  _  _  _  _
 | _| _||_||_ |_   ||_||_|
 ||_  _|  | _||_|  ||_| _|
```

(chaque chiffre a une hauteur de 3 lignes)

Note : Ne lisez _PAS_ la deuxième partie avant d'avoir terminé la première. Un des objectifs de ce kata est de vous faire pratiquer le refactoring et l'adaptation aux changements d'exigences.

### Partie 2

Modifiez votre programme pour qu'il prenne en charge la largeur ou la hauteur variable des chiffres.
Par exemple, pour une largeur = 3 et une hauteur = 2, le chiffre 2 sera :

```
 ___
    |
    |
 ___
|
|
 ___
```

Ce kata est basé sur

- <https://codingdojo.org/kata/NumberToLCD/>

- <https://github.com/coreyhaines/kata-number-to-led>

- <http://rubyquiz.com/quiz14.html>
