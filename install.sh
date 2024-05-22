#!/usr/bin/env bash
clear

echo "Bienvenue dans le script d'installation de tri_photo"
echo
#Vérification des droits de l'utilisateur
if [[ $(id -u) -eq 0 ]]
then
   echo "Ce script doit être lancé en utilisateur lambda, il vous demandera votre mot de passe au moment voulu"
   echo "Usage : ./install.sh" 
   exit 1
fi

# Choix du systéme d'exploitation
# OS=$(cat /etc/lsb-release 2> /dev/null | grep -E -m1 NAME | sed -e "s/NAME=//" | sed -e "s/\"//g")
# echo "OS: $OS"

OS=""
while [[ ! $REPLY =~ ^[1-4]{1} ]]; do
	clear
	echo "Quel est votre système d'exploitation?:"
	echo "1: ArchLinux et ses dérivés "
	echo "2: Debian et ses derivés"
	echo "3: Ubuntu et ses derivés"
	echo "4: Autre système d'exploitation GNU/LINUX"
	echo "5: Quitter"
	read -p "Votre choix? " REPLY
	#statements
	[[ $REPLY == 1 ]] && OS="ArchLinux"
	[[ $REPLY == 2 ]] && OS="Debian"
	[[ $REPLY == 3 ]] && OS="Ubuntu"
	[[ $REPLY == 4 ]] && echo "Fonctionnalité pas encore prise en charge, au revoir" && exit 2
	[[ $REPLY == 5 ]] && echo "Au revoir" && exit 5
done
clear
echo "Votre OS: $OS"

echo "Vérification des prérequis"
case $OS in
	"ArchLinux")
		_prerequis="cargo gtk4 git"
		for requis in $_prerequis; do
	            echo -n "Vérification de $requis ..."
	            pacman -Q $requis > /dev/null 2>&1 && {
	                    echo "ok"
	                    continue
	            } || {
	                    echo "Non trouvé"
	                    prog+="$requis "
	                    _return=1
	            }
	            [[ -z "$_return" ]] || {
	        		echo "Erreur, les programmes requis ne sont pas tous installés!!"
	        		echo "Installer $prog"
			        exit 4
					}
		done
		echo "récupération du PKGBUILD sur gitlab et des sources sur github"
		mkdir -pv /opt/test1 && cd /opt/test1 
		git clone https://gitlab.archlinux.org/nfili/tri_photo.git
		cd tri_photo
		makepkg -Cci
		;;
	"")
		echo "OS: vide"
		;;
esac

