# Semaine 4

### Description du travail

#### 4.25h

Découverte d'un problème avec mon implémentation du BVH (Bounding Volume Hierarchy), et recherche sur le sujet et l'implémentation en Rust pour obtenir une référence.

#### 3.25 h

Adaptation du BVH et tentative de résoudre les problèmes liés à l'ordering des nodes qui constituent des Hittables.

#### 3.25 h

Suite des changements aux BVH avec l'ajout de la fonction pour l'axis range.

#### 4.25

Finalisation de la méthode BVH (fonctionnel) et ajout du fonctionnement de textures pour les surfaces sphérique (checker). La manière avec laquelle l'intégration du système de texture a été
effectuée permettrait éventuellement de mettre des textures provenant d'images externes

<hr/>
#### ~15h

### Connaissances acquises

Cette semaine, j'ai beaucoup approfondi mes connaissances sur l'optimisation pour le raytracing entre autre avec la méthode BVH. J'ai réussi à finaliser son intégration cette semaine alors que j'avais débuté en fin de semaine passée.
Ensuite, j'ai débuté l'ajout de texture dans ma scène. Au lieu de simples couleurs associés aux "Hittables", il y a désormais une texture pour chaque élément. Certains ont une texture qui ne représente qu'une seule couleur alors que d'autres
tel que le checker sont plus complexes dans leur représentation. Ceux-ci sont encore associés à un albedo qui détermine la couleur et le niveau de réflexion.

### Références

https://doc.rust-lang.org/stable/rust-by-example/ Rust by Example pour apprendre les fonctionnalités du langage.

https://doc.rust-lang.org/rust-by-example/generics.html Exemples et explications sur l'utilisation des génériques en Rust (m'a permis de faire les textures)

https://github.com/fralken/ray-tracing-the-next-week/blob/master/src/bvh.rs Intégration du BVH en Rust

https://en.wikipedia.org/wiki/Bounding_volume_hierarchy Bounding volume hierarchy
