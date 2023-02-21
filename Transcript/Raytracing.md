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

##
