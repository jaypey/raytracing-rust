# Semaine 3

### Description du travail

#### 3.25h

Ajout et implémentation des attributs Copy et Clone pour permettre à certains objets de se copier avant d'être divisé entre les différents threads.

#### 2h

Changement de la manière pour le partage d'objet entre les threads. La méthode en implémentant les attributs Copy et Clone n'était pas optimale et ne fonctionnait pas bien. J'ai donc continuer de rechercher une manière plus optimale.

#### 2h

Utilisation d'une ressource qui utilise le multi-threading en Rust pour me permettre d'utiliser des manières optimales selon le langage avec les packages appropriés. (num_cpus, ImageBuffer, Arc)

#### 3.15h

Finalisation du multi-threading (fonctionnel), changement du fonctionnement de génération d'image en utilisant un package Rust image. Avec l'optimisation général, j'ai pu passé de 3h pour la génération d'une image de 400x225 avec un sampling de 100 par pixel à 3 minutes pour la génération d'une image de 1920x1080.

#### 4.75h

Ajout d'un benchmark pour connaître l'effet des optimisations avec la crate std::time.

<hr/>
#### ~15h

### Connaissances acquises

### Références

https://doc.rust-lang.org/stable/rust-by-example/ Rust by Example pour apprendre les fonctionnalités du langage.

https://github.com/andystanton/raytracer-rs/blob/master/src/raytracer.rs Utilisation du multithreading en Rust
