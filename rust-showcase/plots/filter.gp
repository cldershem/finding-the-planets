set datafile separator ','
set key top left
set terminal pngcairo size 750,560

color = "forest-green"

set output "../assets/filter.png"
plot [2905:2985] "../assets/filter.csv" using 1:2 title "filtered" linecolor color