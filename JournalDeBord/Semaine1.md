# Semaine 1

### Description du travail

#### 3h
Installation et apprentissage des concepts fondamentaux de Rust avec les ressources offertes sur leur site. Je me suis beaucoup servit de la section *Rust by Example* pour me permettre de voir les différences entre Rust et un langage comme C++ que je connais mieux. J'ai également appris à comprendre et utiliser Cargo: le package manager de Rust.

#### 2h
Début du développement du render de couleurs avec un format PPM comme présenté dans le projet RayTracing in one weekend. Ajout d'un package Rust pour gérer les vecteurs 3d et intégration de cargo le package manager Rust dans mon projet.

#### 1h
Ajout du module Ray pour le fondement du raytracing et ajout d'une "skybox" comme fond sur lequel construire les objets qui seront observés.

Raytracing du premier objet (un cercle vert). La couleur retournée est basée sur la position du rayon dans la projection. Si elle touche un objet vert (dans ce cas-ci) la couleur rendu sera verte.

#### 4h
Ajout de plusieurs objets dans le render raytraced par l'abstraction de l'interaction avec les rayons de couleurs. Les objets hérite désormais d'un trait Hittable qui permet d'implémenter leur propre géométrie (et éventuellement leur propre valeur de réflexion). Ajout d'un système pour changer la couleur basé sur les normales d'un objet par rapport au rayon envoyé. J'ai passé beaucoup de temps également à découvrir le langage Rust, car puisque ma principale ressource pour le développement est entièrement en C++ je dois trouver des alternatives et gérer les différences de langage.

#### 3h
Ajout d'un matériau qui diffuse les rayons(donne un aspect matte). Ajout de l'antialiasing avec le calcul de densité de rayon et de leur couleur.

### Références

https://raytracing.github.io/books/RayTracingInOneWeekend.html Tutoriel d'implémentation de Raytracing en C++.

https://doc.rust-lang.org/stable/rust-by-example/ Rust by Example pour apprendre les fonctionnalités du langage.