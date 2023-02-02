# Semaine 1

### Description du travail

#### 2h
Ajout de matériaux avec différents algorithme de réflection. Lambertian(mat) et métallique. 

![ExempleFinal](./images/exempleMateriau.png)

#### 8.25 h
Ajout de la fuzziness des objets métalliques, ajout de dielectrics (matériaux comme de la vitre ou de l'eau) et de la réfraction dans les objets. Changement de fonctionnement de la caméra pour qu'elle puisse changer de position, d'orientation et de field of view. Ajout de la profondeur de champ pour créer un effet de lentille. Création d'un render final avec les différentes sphères de matériaux différents pour observer les effets sur les rayons de couleurs.

![ExempleFinal](./images/exempleFinal.png)

<hr/>
#### ~15h

### Connaissances acquises

### Références

https://raytracing.github.io/books/RayTracingInOneWeekend.html Tutoriel d'implémentation de Raytracing en C++.

https://doc.rust-lang.org/stable/rust-by-example/ Rust by Example pour apprendre les fonctionnalités du langage.

https://docs.rs/nalgebra/0.8.2/nalgebra/ Crate pour Rust qui permet l'utilisation de Vector3 avec les transformations vectorielles associées.

https://github.com/jmarshallstewart/Ray-Tracing-In-One-Weekend/blob/master/Complete%20Book%20One/RTiOW/Main.cpp 
https://mikeadev.net/2019/11/parallelizing-ray-tracing/ Ressources qui montre l'utilisation de multi-threading pour le render d'une image raytraced. 