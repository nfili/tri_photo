# ![logo](icons/32x32/tp.png) tri_photo (tp) 
![Static Badge](https://img.shields.io/badge/Rust-1.75.0-%2Cblue?style=plastic&logo=Rust)

tri_photo est un programme écrit en Rust, il utilise gtk4 pour son interface graphique.

Il permet de regrouper, dans un même répertoire hiérarchisé de destination, des images dispersées sur son disque ou tout autre support.

![tp](image/configuration.png)
***

## Table des matières

- 📦 [Prérequis](#prérequis)
- 🚀 [Installation](#installation)
- 🛠️ [Utilisation](#utilisation)
- 🤝 [Contributeurs](#contributeurs)
- 💬 [Retour](#retour)
- 🏷️ [Historique des versions](#historique-des-versions)
- 📝 [Licence](#licence)

## Prérequis

Pour installer le programme vous aurez besoin des dépendances suivante

* sudo
* gtk4
* git

Vous aurez besoin en plus

<details>
    <summary><img src="https://github.com/archlinux/archwiki/blob/master/extensions/ArchLinux/modules/favicon.ico"> sous Archlinux, et ses dérivés</summary>
        
* base-devel
* cargo
        
</details>

<details>
    <summary><img src="https://www.debian.org/logos/openlogo-nd-25.png"> sous Debian/Ubuntu et ses dérivés</summary>
        
* apt
* wget

</details>

<details>
    <summary><img src="https://www.redhat.com/favicon.ico"> sous RHEL et ses dérivés</summary>
    
* yum
* wget

</details>

<details>
    <summary><img src="https://fedoraproject.org/favicon.ico"> sous Fedora et ses dérivés</summary>
        
* dnf
* wget

</details>

<details>
    <summary><img src="https://www.kernel.org/theme/images/logos/favicon.png"> pour les autres système GNU/Linux non listée</summary>
        
* cargo (rust)

</details>

Fonctionne avec tous les systèmes GNU/Linux, les suivants ont été testés :
* Archlinux

## Installation

Le script "install_tri_photo.sh" installera automatiquement le paquet en fonction de votre distribution, lancer le simplement en mode utilisateur

```
        cd /opt/
        git clonne https://github.com/nfili/tri_photo.git
        cd tri_photo
        ./install_tri_photo.sh
```

## Désinstallation

Vous pouvez désinstaller le paquets soit grâce à votre gestionnaire de paquet, soit grâce au script install_tri_photo.sh situé dans le répertoire source du paquet

```
        ./install_tri_photo.sh -r
```

## Utilisation

### Répertoire de travail

* répertoire source : permet de sélectionner le répertoire où le programme doit chercher
  - exemple : /home/{user}/Downloads
* répertoire de destination : permet de choisir l'emplacement où les fichiers seront copier ou déplacer
  - exemple : /home/{user}/save

### Fichiers à trouver

* sélection des types de fichier image

### Arborescence du tri

Sélection de la structure du répertoire de destination via des options proposées :

* jour
* mois
* année
* lieux (pour les photos).
* mois en lettre

### Traitement

Sélection des option de traitement des fichiers : 
 * Renommer les fichiers
 * Supprimer les sources
 * Gestion au fur et à mesure

## Contributeurs

<a href="https://github.com/nfili/tri_photo/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=nfili/tri_photo" />
</a>

## Retour

> Vous pouvez améliorer ce projet, n'hésitez pas à ouvrir une  [Pull Request](https://github.com/nfili/tri_photo/pulls).
- Si vous constatez un bug ou une faute de frappe utilisez la balise "Correction".
- Si vous souhaitez partager des idées pour améliorer ce projet, utilisez la balise « Amélioration ».

<details>
    <summary>Contacter Moi 📨</summary>

### Contact<!-- Required -->
Contactez-moi par e-mail: [nicolasfilippozzi@gmail.com](mailto:nicolasfilippozzi@gmail.com)
<!-- 
* nicolasfilippozzi@gmail.com
* Nicolas Filippozzi
-->
    
</details>

## Historique des versions

* 0.1.0
  * Première version de test pre-release

* 1.0.0
  * Première version de production

## Licence

Voir les fichiers :
* [LICENSE-MIT](./LICENSE-MIT) du dépôt.
* [LICENCE-APACHE](./LICENCE-APACHE) du dépôt.
