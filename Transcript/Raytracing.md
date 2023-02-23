Au départ, le raytracing était réservé au CGI, entre autre à cause de la lourdeur des calculs, mais les cartes graphiques modernes ont donné accès à ce type de technologie qui permet d'observer des éclairages très réalistes pour les jeux vidéos. Et c'est cette nouveau possibilité de faire du raytracing en temps réel qui m'a rendu curieux des algorithmes et de la programmation derrière.

# Rust

## Qu'est-ce que Rust?

Rust est souvent comparé au langage C++ pour plusieurs raisons: ils ont une syntaxe semblable, ils compilent vers du code natif, les deux n'ont pas de garbage collection et les deux sont des langages de programmation.

## Grandes différences avec C++

Rust est sur plusieurs aspects un langage fonctionnel, mais également orienté objet. En recherchant sur le sujet, un membre de la communauté a expliqué que Rust est un langage 25% fonctionnel, 25% orienté objet et 50% orienté trait. Les classes en Rust n'existe pas, ce sont plutôt des struct pour lesquelles des fonctions sont implémentés. Pour l'héritage, ce sont des traits qui permettent l'implémentation de fonction communes à plusieurs structs.

Pour moi la découverte de crates.io (élément central du langage) m'a énormément fait apprécié le langage. Comme pip ou npm pour python et javascript, crates est un package manager intégré directement à Rust et permet d'utiliser des "crate" de contributeurs. Les packages managers sont à mon avis, un énorme plus pour la réutilisation de code et la collaboration des développeurs.

# Qu'est-ce que le raytracing.

## Caméra

La caméra est le point de départ de la plupart des raytracers. Si vous êtes familier avec le fonctionnement de la physique de la lumière, vous savez que la lumière provient d'une source et ce sont les rayons de cette source
qui permettent de percevoir les couleurs en se rendant à notre oeil, mais faire partir les rayons de la caméra permet d'ignorer le calcul d'énormément de rayons qui ne sont pas nécessaire pour un render. C'est l'une des seules différences
avec la lumière visible dans la réalité. Le raytracing ressemble beaucoup à la réalité. L'intéraction réaliste des rayons avec les objets permet de créer des phénomènes optiques que l'ont constate chaque jour (réflection, réfraction, diffraction, etc)

## Rayons

Un rayon est envoyé à partir de la caméra, qui simule l'oeil humain, et traverse un plan qui sera render et dont la couleur des pixels sera déterminé par les objets et qui peuvent intéragir avec
ces rayons. Par exemple, pour chaque pixels, 50 rayons peuvent être envoyés pour déterminer leur couleur.

## Objets

Les objets sont des choses qui intéragissent avec les rayons, ces objets possèdent des couleurs, un albedo et des caractéristiques générales qui déterminent ce que la caméra perçoit comme couleur.

<hr/>

# Affichage d'un pixel raytraced

## Code

Afficher main_scene() -> Afficher trace() -> Aller dans ray_color -> Retourner dans trace()

# Matériaux et leur réflection

Présenter chaque type de réflection et présenté le code associé pour chacun. Présenter l'ajout de texture.

# Multi-threading et optimisations

Pour générer une image de 1920 x 1080 avec un nombre de rayon par pixel de 50 demanderait au programme de générer 103 680 000 rayons et chacun d'eux peut rebondir sur des objets et donc changer de direction, ils ont des couleurs associés etc donc vous comprenez que sans optimisations, ce n'est pas réaliste. En fait, pour seulement 5 112 000 rayons, la génération durait environ 1h.

La bonne nouvelle, c'est que les rayons n'intéragissent pas ensemble et sont tous indépendant, donc il est très simple d'utiliser le multi threading dans ce type de contexte. C'est donc ce que j'ai fait. Montrer le code. Et simplement en utilisant l'intégralité de mes coeurs logiques, la génération d'une image en 1920x1080 est devenu possible en seulement 3 minutes.

La seconde optimisation que j'ai effectuée se nomme bounding volume hierarchy. L'objectif de cette technique est d'ignorer l'évaluation des objets qui ne sont pas dans la direction du rayon évalué. Pour faire ça, un arbre est construit avec les objets comme feuille. Ainsi, des "objets" artificiels sont créés pour généré des séparations entre les objets qui sont à proximité du rayons et ceux qui ne le sont pas. En fait, c'est très semblable à un arbre binaire, mais appliqué au raytracing. Voir le diagramme.

## Conclusion

C'était très intéressant d'apprendre comment fonctionnait cette technologie et j'ai adoré utiliser Rust qui était un langage qui m'intéressait depuis vraiment longtemps. Pour la suite, j'aimerais ajouter l'importation de texture pour permettre d'utiliser des images externes. Sinon, l'importation d'objets 3D en .obj ou .stl serait également très intéressant.
