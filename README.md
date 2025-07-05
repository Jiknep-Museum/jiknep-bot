** Jiknep Bot **

Bot codé en Rust qui extrait depuis Discord les infos utiles au museum et les compile dans un fichier json.
Le fichier doit être reconstruit régulièrement car les lien vers les images en pièce jointe commencent à 
devienir invalides après environ un heure.

** Fonctionnement

Pour fonctionner le nécecite de définir deux variables d'environnement : 

** Compilation

Le programme s'uppuie sur la bibliothèque discord-rs qui contienrt malheureusement un bug qui fait planter le bot sur les channels qui contiennent un sondage. Pour fonctionner il faut donc une version patchée de cette bibliothèque.

Clonez le source le source de discord-rs dans un répertoire au même niveau que celui qui contient jiknep-bot. Le Cargo.toml est configuré pour utiliser récupérer cette copie locale. 