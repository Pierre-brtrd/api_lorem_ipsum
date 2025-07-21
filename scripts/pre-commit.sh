#!/bin/bash

# 🔍 Pre-commit hook pour vérifications automatiques
# Installe avec: cp scripts/pre-commit.sh .git/hooks/pre-commit && chmod +x .git/hooks/pre-commit

set -e

# Couleurs pour l'affichage
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🔍 Vérifications pré-commit pour API Lorem Ipsum${NC}"
echo "================================================="

# Fonction pour afficher le résultat
print_result() {
    if [ $1 -eq 0 ]; then
        echo -e "${GREEN}✅ $2${NC}"
    else
        echo -e "${RED}❌ $2${NC}"
        exit 1
    fi
}

# 1. Formatage automatique
echo -e "\n${YELLOW}📝 Application du formatage automatique...${NC}"
cargo fmt
print_result $? "Formatage appliqué"

# 2. Vérification du formatage
echo -e "\n${YELLOW}🔍 Vérification du formatage...${NC}"
if cargo fmt --all -- --check > /dev/null 2>&1; then
    print_result 0 "Formatage conforme"
else
    echo -e "${RED}❌ Erreurs de formatage détectées${NC}"
    echo "Exécutez 'cargo fmt' pour corriger automatiquement"
    exit 1
fi

# 3. Clippy (linting strict)
echo -e "\n${YELLOW}📋 Vérification Clippy...${NC}"
if cargo clippy --all-targets --all-features -- -D warnings > /dev/null 2>&1; then
    print_result 0 "Clippy satisfait"
else
    echo -e "${RED}❌ Erreurs Clippy détectées${NC}"
    echo "Exécutez 'cargo clippy --all-targets --all-features -- -D warnings' pour voir les détails"
    exit 1
fi

# 4. Compilation
echo -e "\n${YELLOW}🏗️ Vérification de la compilation...${NC}"
if cargo check > /dev/null 2>&1; then
    print_result 0 "Compilation réussie"
else
    echo -e "${RED}❌ Erreurs de compilation${NC}"
    echo "Exécutez 'cargo check' pour voir les détails"
    exit 1
fi

# 5. Tests unitaires et d'intégration
echo -e "\n${YELLOW}🧪 Exécution des tests...${NC}"
if cargo test > /dev/null 2>&1; then
    print_result 0 "Tests réussis"
else
    echo -e "${RED}❌ Tests en échec${NC}"
    echo "Exécutez 'cargo test' pour voir les détails"
    exit 1
fi

# 6. Vérification des secrets/tokens (basique)
echo -e "\n${YELLOW}🔒 Vérification des secrets...${NC}"
if git diff --cached --name-only | grep -v -E "\.(md|txt|rst|doc)$|scripts/.*\.sh$" | xargs grep -l "API_KEY\|SECRET\|TOKEN\|PASSWORD" > /dev/null 2>&1; then
    echo -e "${RED}❌ Possible secret détecté dans les fichiers de code${NC}"
    echo "Vérifiez qu'aucun secret n'est committée par erreur"
    echo "Fichiers concernés:"
    git diff --cached --name-only | grep -v -E "\.(md|txt|rst|doc)$|scripts/.*\.sh$" | xargs grep -l "API_KEY\|SECRET\|TOKEN\|PASSWORD" 2>/dev/null || true
    exit 1
else
    print_result 0 "Pas de secrets détectés dans le code"
fi

# 7. Vérification TODO/FIXME pour les branches importantes
current_branch=$(git rev-parse --abbrev-ref HEAD)
if [ "$current_branch" = "master" ] || [ "$current_branch" = "main" ]; then
    echo -e "\n${YELLOW}📋 Vérification TODO/FIXME sur branche ${current_branch}...${NC}"
    if git diff --cached | grep -E "(TODO|FIXME|XXX)" > /dev/null 2>&1; then
        echo -e "${YELLOW}⚠️ TODO/FIXME détectés sur branche ${current_branch}${NC}"
        echo "Considérez résoudre ces éléments avant le commit en production"
        # Ne pas échouer, juste avertir
    fi
fi

echo -e "\n${GREEN}🎉 Toutes les vérifications sont passées !${NC}"
echo -e "${BLUE}📤 Prêt pour le commit${NC}"
echo "================================================="

exit 0
