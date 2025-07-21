#!/bin/bash

# 🗑️ Désinstallation des Git hooks pour API Lorem Ipsum
# Ce script supprime tous les hooks Git installés

set -e

# Couleurs pour l'affichage
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🗑️ Désinstallation des Git Hooks - API Lorem Ipsum${NC}"
echo "================================================="

# Vérifier qu'on est dans un repository Git
if [ ! -d ".git" ]; then
    echo -e "${RED}❌ Erreur: Ce n'est pas un repository Git${NC}"
    echo "Exécutez ce script depuis la racine du projet"
    exit 1
fi

# Demander confirmation
echo -e "${YELLOW}⚠️ Cette action va supprimer tous les hooks Git installés${NC}"
read -p "Êtes-vous sûr ? (y/N): " -n 1 -r
echo

if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo -e "${BLUE}🚫 Désinstallation annulée${NC}"
    exit 0
fi

echo -e "\n${YELLOW}🗑️ Suppression des hooks...${NC}"

# Liste des hooks à supprimer
hooks=("pre-commit" "pre-push" "commit-msg" "post-commit")

for hook in "${hooks[@]}"; do
    if [ -f ".git/hooks/$hook" ]; then
        rm ".git/hooks/$hook"
        echo -e "${GREEN}✅ Hook $hook supprimé${NC}"
    else
        echo -e "${YELLOW}⚠️ Hook $hook non trouvé${NC}"
    fi
done

# Optionnel: sauvegarder les hooks personnalisés
echo -e "\n${YELLOW}💾 Sauvegarde des hooks personnalisés...${NC}"
mkdir -p scripts/hooks-backup

for hook_file in .git/hooks/*; do
    if [ -f "$hook_file" ] && [ ! -f "$hook_file.sample" ]; then
        hook_name=$(basename "$hook_file")
        cp "$hook_file" "scripts/hooks-backup/$hook_name"
        echo -e "${BLUE}📁 Hook personnalisé $hook_name sauvegardé${NC}"
    fi
done

echo -e "\n${GREEN}🎉 Désinstallation terminée !${NC}"
echo "================================================="
echo -e "${BLUE}📋 État:${NC}"
echo "  ✅ Hooks Git supprimés"
echo "  ✅ Hooks personnalisés sauvegardés dans scripts/hooks-backup/"
echo "  ✅ Repository Git intact"

echo -e "\n${YELLOW}🔧 Pour réinstaller:${NC}"
echo "  ./scripts/install-hooks.sh"

echo -e "\n${BLUE}💡 Alternative - Bypass temporaire:${NC}"
echo "  git commit --no-verify     # Ignorer les hooks pour un commit"
echo "  git push --no-verify       # Ignorer les hooks pour un push"

echo -e "\n${GREEN}✨ Hooks désinstallés avec succès !${NC}"
