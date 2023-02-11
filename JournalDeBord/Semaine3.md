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

Ajout d'un benchmark pour connaître l'effet des optimisations avec la crate std::time. Ajout d'une optimisation Bounding Volumes qui permet d'empêcher les rayons qui ne touchent pas d'objets d'être calculés pour aucune raison. Cette optimisation fonctionne avec des boîtes qui englobe des objets et qui permet sous forme d'arbre de savoir si un rayon à un contact ou non.

<hr/>
#### ~15h

### Connaissances acquises

Cette semaine, je passé énormément de temps à apprendre comment réussir le multi-threading en Rust dans le contexte de ce projet. Je devais trouver la portion de mon code qui était facilement divisable en threads et qui consommait beaucoup de temps de calcul. J'ai eu quelques difficultés en début de semaine avec les implémentations de Copy et Clone qui m'imposait un changement fondamental du fonctionnement du code, mais en creusant davantage sur les crates disponibles en Rust et des exemples de multithreading, j'ai réussi à trouver un exemple parfait pour ma situation qui m'a permis de réussir. Vers la fin de la semaine, j'ai débuté l'ajout d'une optimisation commune pour les raytracers: le Bounding volume hierarchy (Voir le 3e lien).

### Références

https://doc.rust-lang.org/stable/rust-by-example/ Rust by Example pour apprendre les fonctionnalités du langage.

https://github.com/andystanton/raytracer-rs/blob/master/src/raytracer.rs Utilisation du multithreading en Rust

https://en.wikipedia.org/wiki/Bounding_volume_hierarchy Bounding volume hierarchy
