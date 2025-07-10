** Jiknep Bot **

Bot codé en Rust qui extrait depuis Discord les infos utiles au museum et les compile dans un fichier json.
Le fichier doit être reconstruit régulièrement car les lien vers les images en pièce jointe commencent à 
devienir invalides après environ un heure.

** Fonctionnement

Pour fonctionner le bot nécessite de définir deux variables d'environnement : 
 - DISCORD_CHANNEL : contient l'identifiant du channel à lire 
 - DISCORD_TOKEN : contient le token du compte Discord utilisé pour lire le channel
Le contenu du channel est récupéré progressivement par groupes de 100 fichiers (limitation de Discord). La progression est affichée sur la sortie d'erreur standard. Le JSON extrait est écrit sur la sortie standard 

** Compilation
Le programme se compile avec un compilateur Rust 1.87.0 ou supérieur. Il se compile comme un programe Rust classique via la commande `cargo build --release` le binaire final se trouve dans le sous-répertoire target/release   

Le programme s'appuie sur la bibliothèque discord-rs dont la version officielle contient malheureusement un bug qui fait planter le bot sur les channels qui contiennent un sondage. le jiknep-bot utilise une version patchée de cette bibliothèque hébergée sur son dépot github. Le Cargo.toml est configuré pour utiliser cette version patchée. 