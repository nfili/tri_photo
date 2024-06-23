#!/usr/bin/env bash
##################################################################
# Name : install_tri_photo.sh                           			  #
# Version : 1.0.0                                                #
# Crée le : 05-2024                                              #
# Modifié le : 23-06-2024                                        #
# Desciption : Script d'installation/désinstallation de tri_photo#
##################################################################


TEXTDOMAIN=${0:2}
TEXTDOMAINDIR="$PWD/locale"

PKGNAME=tri_photo
VERSION="1.0.0"
DEV_MAIL="nicolasfilippozzi@gmail.com"


# FILE_LOG="/var/log/tri_photo/log"

DEBIAN_DERIV=(
	"debian" "linuxmint" "ubuntu" "zorin" "pop" "neon" "antiX" "elementary" "pureos" "deepin" "sparky" 
        )
ARCH_DERIV=(
	"arch" "endeavouros" "manjaro" "garuda" "cachyos" "arcolinux"
 "archcraft" "blendos" "artix" "rebornos"
 )
FEDORA_DERIV=("fedora" "almalinux" "rocky" "eurolinux")
RHEL_DERIV=("rhel" "centos")

function usage(){
	printf $"Use : ./%s\n" "$PKGNAME"
   printf $"Use : ./%s -r\n" "$PKGNAME"
	printf $"Use : ./%s --help\n" "$PKGNAME"
   }

function prerequis(){
	_prerequis="$1"
	echo $_prerequis
		for requis in $_prerequis; do
	            printf $"Verification of %s ... " "$requis"
	            $($2 $requis > /dev/null 2>&1)  && {
	                    printf $"found\n"
	                    continue
	            } || {
	                    printf $"not found\n"
	                    prog+="$requis "
	                    _return=1
	            }
	            [[ -z "$_return" ]] || {
	        		printf $"Error, not all required programs are installed!!\n"
	        		printf $"Install %s with the command: 'sudo %s %s\n'" "$prog" "$3" "$prog"
			        exit 4
					}
		done
}

clear
printf $"Welcome to the installation script of %s \n" "$PKGNAME"
echo 
#Vérification des droits de l'utilisateur
if [[ $(id -u) -eq 0 ]]
then
   echo $"This script must be launched as an average user, it will ask you for your password at the appropriate time"
   usage
   exit 1
fi

if [[ $# -eq 1 ]]; then
	case "$1" in
		"-r" | "remove")
			REMOVE=true 
			printf $"Uninstallation of %s requested\n" "$PKGNAME"
			;;
		"-h" | "help")
			printf $"Use %s [OPTION]\n" "$PKGNAME"
			printf $"Install/uninstall script of %s\n" "$PKGNAME"
			echo
			usage
			echo
			echo $"OPTION"
			printf $"\t-r | remove\t\t\tDelete of %s\n" "$PKGNAME"
			printf $"\t-h | help\t\t\tHelp of %s\n" "$PKGNAME"
			exit 0
			;;
		*)
			echo $"Unknown command"
			usage
			exit 1
			;;
	esac 
fi
# Détection du systéme d'exploitation
OS=$(cat /etc/*-release 2> /dev/null | grep -E -m1 ID | sed -e "s/ID=//" | sed -e "s/\"//g")
printf $"Operating system: %s\n" "$OS"
# Sélection du gestionnaire de paquet
for name in "${DEBIAN_DERIV[@]}"; do
	[[ "$OS" == "$name" ]] && PACKAGE_MANAGER="apt"
done
if [[ ! $PACKAGE_MANAGER ]]; then
	for name in "${ARCH_DERIV[@]}"; do
		[[ "$OS" == "$name" ]] && PACKAGE_MANAGER="pacman"
	done
fi
if [ ! $PACKAGE_MANAGER ]; then
	for name in "${FEDORA_DERIV[@]}"; do
		[[ "$OS" == "$name" ]] && PACKAGE_MANAGER="dnf"
	done
fi
if [ ! $PACKAGE_MANAGER ]; then
	for name in "${RHEL_DERIV[@]}"; do
		[[ "$OS" == "$name" ]] && PACKAGE_MANAGER="yum"
	done
fi
# ID de os-release inconnu, ajout manuel par l'utilisateur
if [ ! $PACKAGE_MANAGER ]; then
	while [[ ! $PACKAGE_MANAGER ]]; do
		echo $"Your operating system is not known to the script."
		echo $"What is your package manager?"
		echo "1. pacman"
		echo "2. apt"
		echo "3. dnf"
		echo "4. yum"
		echo $"5. other"
		read -p $"Your choice?"
		[[ $REPLY -eq 1 ]] && $PACKAGE_MANAGER="pacman"
		[[ $REPLY -eq 2 ]] && $PACKAGE_MANAGER="apt"
		[[ $REPLY -eq 3 ]] && $PACKAGE_MANAGER="dns"
		[[ $REPLY -eq 4 ]] && $PACKAGE_MANAGER="yum"
		if [[ $REPLY -eq 5 ]]; then
			if [[ $REMOVE ]]; then
				read -p $"Command to remove a package? "
				if [[ "$REPLY" =~ ^[a-Z_\]*[[:space:]]{1}[-]{0,2}[a-zA-Z]*$ ]]; then
					PACKAGE_REMOVE="$REPLY"
				fi
			else
				while [[ ! $PACKAGE_MANAGER ]]; do
					read -p $"Name of your package manager?"
					[[ "$REPLY" =~ ^[[:alpha:]]*$ ]] && PACKAGE_MANAGER="$REPLY"
				done
				while [[ ! $PARAM_INSTALL ]]; do
					echo $"Indicate the argument for the installation."
					read -p $"exemple: apt install ?"
					[[ "$REPLY" =~ ^[-]{0,2}[[:alpha:]]*$ ]] && PARAM_INSTALL="$REPLY"
				done
				while [[ ! $PARAM_SEARCH ]]; do
					echo $"Indicate the argument for the installed package search"
					read -p $"exemple: apt show ?"
					[[ "$REPLY" =~ ^[-]{0,2}[[:alpha:]]*$ ]] && PARAM_SEARCH="$REPLY"
				done
			fi
		fi
	done
fi
printf $"Package Manager: %s\n" "$PACKAGE_MANAGER"

# vérification des prérequis pour l'installation de tri_photo et installation
echo $"Checking prerequisites"
_prerequis="sudo cargo gtk4 wget"
case $PACKAGE_MANAGER in
	"pacman")
		#désinstallation detectée
		if [[ $REMOVE ]]; then
			sudo pacman -R tri_photo
		else
			_prerequis_mod=$( echo $_prerequis | sed -e "s/wget/git/" )
			prerequis "$_prerequis_mod" "pacman -Q" "pacman -S"
			echo $"Retrieving PKGBUILD from gitlab and sources from github"
			mkdir -pv /opt/$PKGNAME && cd /opt/$PKGNAME
			mkdir -pv pkgbuild && cd pkgbuild  
			git clone https://gitlab.archlinux.org/nfili/$PKGNAME.git
			cd $PKGNAME
			makepkg -Cci
		fi
		;;
	"apt" )
		#désinstallation detectée
		if [[	$REMOVE ]]; then
			sudo apt remove $PKGNAME
		else
			prerequis "$_prerequis" "apt show" "apt install"
			echo $"Retrieving the .deb package from github"
			wget https://github.com/nfili/${PKGNAME}/releases/download/${VERSION}/${PKGNAME}_${VERSION}-1_amd64.deb
			printf $"Installation of %s" $PKGNAME
			sudo apt install ${PKGNAME}_${VERSION}-1_amd64.deb
		fi
		;;
	"dnf")
		#désinstallation detectée
		if [[ $REMOVE ]]; then
			sudo dnf remove $PKGNAME
		else
			prerequis "$_prerequis" "dnf list" "dnf install"
			echo $"Retrieving the .rpm from github for installation with dnf"
			wget https://github.com/nfili/${PKGNAME}/releases/download/${VERSION}/${PKGNAME}-${VERSION}-1.x86_64.rpm
			sudo dnf install ${PKGNAME}_${VERSION}-1.x86_64.rpm
		fi
		;;
	"yum")
		#désinstallation detectée
		if [[ $REMOVE ]]; then
			sudo yum remove $PKGNAME
		else
			prerequis "$_prerequis" "yum list" "yum install"
			echo $"Retrieving the .rpm from github for installation with yum"
			wget https://github.com/nfili/${PKGNAME}/releases/download/${VERSION}/${PKGNAME}-${VERSION}-1.x86_64.rpm
			sudo yum install ${PKGNAME}_${VERSION}-1.x86_64.rpm
		fi
		;;
	*)
		#désinstallation detectée
		if [[ $REMOVE ]]; then
			sudo rm -v /usr/bin/tp
			sudo rm -v /usr/share/applications/tp.desktop
			sudo rm -v -R /usr/share/{doc,licences}/$PKGNAME
			sudo rm -v -R /usr/share/locale/*/LC_MESSAGES/tp.mo
			for res in 512x512 256x256 128x128 80x80 72x72 64x64 48x48 44x44 36x36 32x32 24x24 22x22 20x20 16x16 8x8; do
				sudo rm -v "/usr/share/icons/hicolor/${res}/apps/tp.png"
			done
			echo $"Uninstallation successful"
		else
			_prerequis_mod=$( echo $_prerequis | sed -e "s/wget/git/" )
			prerequis "$_prerequis_mod" "$PACKAGE_MANAGER $PARAM_SEARCH" "$PACKAGE_MANAGER $PARAM_INSTALL"
			git clone https://github.com/nfili/${PKGNAME}
			cd "$PKGNAME"
			export RUSTUP_TOOLCHAIN=stable
	    	cargo fetch --locked --target "$(rustc -vV | sed -n 's/host: //p')"
			cargo build --release --locked --all-features

			sudo install -dm755 "/usr/bin"
			sudo install -Dm755 "target/release/tp" "/usr/bin/tp"

			for res in 512x512 256x256 128x128 80x80 72x72 64x64 48x48 44x44 36x36 32x32 24x24 22x22 20x20 16x16 8x8; do
				sudo install -dm755 "/usr/share/icons/hicolor/${res}/apps"
				sudo cp "icons/${res}/tp.png" "/usr/share/icons/hicolor/${res}/apps/tp.png"
			done

			for lang in `ls locale`;do
				sudo install -dm755 "/usr/share/locale/${lang}/LC_MESSAGES/"
				sudo cp -v "locale/${lang}/LC_MESSAGES/tp.mo" "/usr/share/locale/${lang}/LC_MESSAGES/tp.mo"
			done

			sudo install -dm755 "/usr/share/applications"
			sudo install -Dm644 'tp.desktop' "/usr/share/applications/tp.desktop"
			sudo install -Dm644 "README.md" "/usr/share/doc/${PKGNAME}/README.md"
			sudo install -Dm644 "COPYRIGHT.md" "/usr/share/doc/${PKGNAME}/COPYRIGHT"
			sudo install -Dm644 "CHANGELOG.md" "/usr/share/doc/${PKGNAME}/CHANGELOG"
		  	sudo install -Dm644 "LICENSE-MIT" "/usr/share/licenses/${PKGNAME}/LICENSE-MIT"
		  	sudo install -Dm644 "LICENSE-APACHE" "/usr/share/licenses/${PKGNAME}/LICENSE-APACHE"
		fi
		read -p $"Would you like to help us by sharing the commands you used? [Y/N]"
		while [[ ! $OK ]]; do
			[[ "$REPLY" =~ "[OoYy]{1}" ]] && {
				printf $"Send mail to %s\n" "$DEV_MAIL"
				printf $"Subject: %s, adding ID, on %\n" "$PKGNAME" "$(date)"
				printf $"Body: To install %s, I used the following commands on the OS: %s\n" "$PKGNAME" "$OS"
				printf $"PACKAGE_MANAGER : %s\n" "$PACKAGE_MANAGER"
				printf $"Installation argument : %s\n" "$PARAM_INSTALL"
				printf $"Installed package search argument: %s\n" "$PARAM_SEARCH"
				OK=true
			} || {
				[[ "$REPLY" =~ "[Nn]{1}" ]] && OK=true
			}
		done
		;;
esac
exit 0
