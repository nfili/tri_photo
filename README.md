# ![logo](https://github.com/nfili/tri_photo/blob/develop/src/resources/icons/16x16/actions/image.ico =32x32) tri_photo ![Static Badge](https://img.shields.io/badge/Rust-1.75.0-%2Cblue?style=plastic&logo=Rust)
***
 ![left 50%](https://github.com/nfili/tri_photo/blob/develop/image/acceuil.png)
 ![right 50%](https://github.com/nfili/tri_photo/blob/develop/image/configuration.png)

---

## Table of Contents
1. [A Propos](#à-propos)
2. [Installation](#instalation)
3. [](#installation)
4. [Collaboration](#collaboration)
5. [FAQs](#faqs)

## A propos

tri_photo est un programme écrit en rust, il utilise gtk4 pour son interface graphique.

Il permet de regrouper, dans un meme répertoire hiérachisé de destination, des images dispersées sur son disque ou tout autre support.



## Installation

> Archlinux

```
        git clone https://emplacement/du/PKGBUILD
        cd tri_photo
        makepkg
        sudo pacman -U tri_photo.extensionde pacman
```


## Fonctionnement

1. __Les répertoire de travail__
    * repertoire source : permet de sélectionner le répertoire ou le programme doit chercher
        - exemple : /home/{user}/Downloads
    * répertoire de desticnation : permet de choisir l'emplacement ou les fichiers seront copier ou déplacer
        - exemple : /home/{user}/save

2. __Les fichiers à trouver__

l'utilisateur pourra sélectionner la structure du répertoire choisie via des options propposé, jour,mois, année, et lieux (pour les photos).
L'utilisateur pourra choisir de gérer chaque fichier individuellement, de copier/déplacer l'ensemble des fichiers images.



# Titre alternatif

## Sous-titre alternatif

Les paragraphes sont séparés
par une ligne laissée vide.

Deux espaces à la fin d'une ligne
produisent un saut de ligne.
