#!/usr/bin/env bash

BASE_NAME=${0:2}
VERSION="1.0.0"

function usage(){
    echo "Use : ./$BASE_NAME init [SCRIPTNAME]"
    echo "Use : ./$BASE_NAME newlang [LANG] [SCRIPTNAME]"
    echo "Use : ./$BASE_NAME translate [LANG] [SCRIPTNAME]"
    echo "Use : ./$BASE_NAME final [LANG] [SCRIPTNAME]"
    echo "Use : ./$BASE_NAME clear [LANG] [SCRIPTNAME]"
    echo "Use : ./$BASE_NAME clearall [SCRIPTNAME]"
    echo "Use : ./$BASE_NAME help"
}
function help(){
    echo "Use: $BASE_NAME [COMMAND] [PARAMS]"
    echo
    echo "Exemple: "
    usage
    echo
    echo "COMMAND"
    echo -e "\tinit \t\t\t Create a .pot file"
    echo -e "\tnewlang [LANG]\t\t Init directories for the language"
    echo -e "\ttranslate [LANG]\t Does the translation for the language"
    echo -e "\tfinal [LANG]\t\t Delete all construction for selected language"
    echo -e "\tclear [LANG]\t\t Delete the language"
    echo -e "\tclearall \t\t Delete all languages"
    echo -e "\thelp \t\t\t Show this help"
    echo
    echo "PARAMS"
    echo -e "\tLANG\t\t\t Language" 
    echo -e "\tSCRIPTNAME\t\t The script that will need to be translated"
    exit 0
}
function checkpot(){
    if [[ ! -r $1.pot ]]; then
        echo "this file ${1}.pot not exist, did you run ${BASE_NAME} init $1?"
        exit 1
    fi
}
function checkLanguage(){
    if [[ ! "$1" =~ [a-z]{2} ]]; then
        echo "Missing LANG parameter"
        echo "run ${BASE_NAME} help for help"
        exit 1
    else
        echo "The language chosen is $1"
    fi
}
function checkScriptname(){
    if [[ ! -x "$1" ]]; then
        echo "File not exist or user hasn't execute access"
        exit 1
    else
        if [[ ! "$1" =~ [a-Z0-9_-]*[.]{0,1}[a-z]{0,2} ]]; then
            echo "File does not match the standard file name format"
            exit 1
        fi
    fi
}
case "$1" in
    "init" )
    checkScriptname $2
    echo "file : $2"
    bash --dump-po-strings "$2" | xgettext -L PO -o "$2.pot" -
    sed -i $2.pot -e 's/CHARSET/UTF-8/'
     #   else
       #     echo "Bad parameters"
      #      exit 1
       # fi
        ;;
    "newlang" )
        echo "newlang"
        checkpot $3
        checkLanguage $2
        checkScriptname $3
        mkdir -pv locale/${2}/LC_MESSAGES
        if [[ -r "locale/$2/LC_MESSAGES/${3}.po" ]]; then 
            echo "The language already exists: $2" 
            echo "for updapte : ${BASE_NAME} translate $2 $3"
            exit 1
        fi
        msginit -l $2 -i $3.pot -o locale/$2/LC_MESSAGES/${3}.po
        ;;
    "translate" )
        checkpot $3
        checkLanguage $2
        checkScriptname $3
        if [[ ! -r $3.pot ]]; then
            echo "this file ${3}.pot not exist, did you run ${BASE_NAME} init $3?"
            exit 1
        fi
        if [[ ! -r "locale/$2/LC_MESSAGES/${3}.po" ]]; then 
            echo "this language not exist, did you run ${BASE_NAME} newlang $2 $3?" 
            exit 1
        fi
        msgmerge -U locale/$2/LC_MESSAGES/${3}.po $3.pot
        [[ ${#EDITOR} -eq 0 ]] && {
            echo "The EDITOR variable is not defined, you must edit the locale/$2/LC_MESSAGES/${3}.po file manually"
            echo "After, your translate locale/$2/LC_MESSAGES/${3}.po rerun  ${BASE_NAME} translate $2 $3"
            echo "ignore if you have already done it."
        } || {
            $EDITOR locale/$2/LC_MESSAGES/${3}.po
        }
        msgfmt -o locale/$2/LC_MESSAGES/${3}.mo locale/$2/LC_MESSAGES/${3}.po
        ;;

    "final" )
        checkLanguage $2
        checkScriptname $3
        rm -v locale/$2/LC_MESSAGES/$3.po
        rm -v $3.pot
        ;;
    "clear" )
        checkLanguage $2
        checkScriptname $3
        rm -Rv locale/$2
        rm -v $3.pot
        ;;
    "clearall" )
        rm -Rv locale
        rm -v *.pot
        ;;
    "help" )
        help
        ;;
    * )
        echo "This script must have parameters!"
        usage
        ;;
esac
exit 0
