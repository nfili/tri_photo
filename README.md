# ![logo](src/resources/icons/16x16/actions/image.ico) tri_photo (tp) 
![Static Badge](https://img.shields.io/badge/Rust-1.75.0-%2Cblue?style=plastic&logo=Rust)

tri_photo est un programme écrit en rust, il utilise gtk4 pour son interface graphique.

Il permet de regrouper, dans un meme répertoire hiérachisé de destination, des images dispersées sur son disque ou tout autre support.

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

Pour installer le programme vous aurez besoin des dependances suivant

* cargo
* gtk4

> sous Archlinux

* base-devel
  
Fonctionne avec tous les systemes d'exploitation, les suivants ont été testés :
* Archlinux

## Installation

Pour le mement seul archlinux a été testé

<details>
    <summary><img src="https://github.com/archlinux/archwiki/blob/master/extensions/ArchLinux/modules/favicon.ico"> Archlinux</summary>

```
        git clone https://emplacement/du/PKGBUILD
        cd tri_photo
        makepkg
        sudo pacman -U tri_photo.extensionde pacman
```
</details>

<details>
    <summary><img src="image/linux.ico"> autre linux</summary>

```
        git clone https://github.com/nfili/tri_photo.git
        cd tri_photo
        cargo build --release
        cd build/
```
</details>

## Utilisation

### Répertoire de travail

* répertoire source : permet de sélectionner le répertoire où le programme doit chercher
        - exemple : /home/{user}/Downloads
* répertoire de desticnation : permet de choisir l'emplacement où les fichiers seront copier ou déplacer
        - exemple : /home/{user}/save

### Fichiers à trouver

* sélection des types de fichier image

### Arborescence du tri

Sélection de la structure du répertoire de déstination via des options propposées :

* jour
* mois
* année
* lieux (pour les photos).
* mois en lettre

### Traitement

Sélection des option de traitement des fichiers : 
 * Renomer les fichiers
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
Contactez-moi par email: [nicolasfilippozzi@gmail.com](mailto:nicolasfilippozzi@gmail.com)
<!-- 
* nicolasfilippozzi@gmail.com
* Nicolas Filippozzi
-->
    
</details>

## Historique des versions
* 0.1
  * Premiére version

## Licence

Voir les fichiers :
* [LICENSE](./LICENSE.md) du dépôt.
* [3rdparty](./3rdparty.md) du dépôt.
