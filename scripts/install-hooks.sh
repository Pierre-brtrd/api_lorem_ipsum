#!/bin/bash

# 🛠️ Installation des Git hooks pour API Lorem Ipsum
# Ce script installe automatiquement les hooks Git pour maintenir la qualité du code

set -e

# Couleurs pour l'affichage
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🛠️ Installation des Git Hooks - API Lorem Ipsum${NC}"
echo "================================================="

# Vérifier qu'on est dans un repository Git
if [ ! -d ".git" ]; then
    echo -e "${RED}❌ Erreur: Ce n'est pas un repository Git${NC}"
    echo "Exécutez ce script depuis la racine du projet"
    exit 1
fi

# Vérifier que le script pre-commit existe
if [ ! -f "scripts/pre-commit.sh" ]; then
    echo -e "${RED}❌ Erreur: scripts/pre-commit.sh introuvable${NC}"
    echo "Assurez-vous que le fichier existe avant d'installer les hooks"
    exit 1
fi

# Créer le répertoire .git/hooks s'il n'existe pas
mkdir -p .git/hooks

echo -e "\n${YELLOW}📋 Installation des hooks...${NC}"

# 1. Hook pre-commit
echo -e "${BLUE}🔍 Installation du hook pre-commit...${NC}"
cp scripts/pre-commit.sh .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit
echo -e "${GREEN}✅ Hook pre-commit installé${NC}"

# 2. Hook pre-push (optionnel, pour tests plus complets)
echo -e "${BLUE}🚀 Création du hook pre-push...${NC}"
cat > .git/hooks/pre-push << 'EOF'
#!/bin/bash

# 🚀 Pre-push hook pour vérifications complètes
set -e

# Couleurs
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}🚀 Vérifications pré-push pour API Lorem Ipsum${NC}"
echo "================================================="

# Tests complets incluant les tests d'intégration
echo -e "\n${YELLOW}🧪 Tests complets (unitaires + intégration)...${NC}"
if cargo test --all; then
    echo -e "${GREEN}✅ Tous les tests passent${NC}"
else
    echo -e "${RED}❌ Des tests échouent${NC}"
    exit 1
fi

# Tests de performance (optionnel, peut être long)
read -p "Exécuter les tests de performance ? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo -e "\n${YELLOW}⚡ Tests de performance...${NC}"
    if cargo test --test performance -- --ignored; then
        echo -e "${GREEN}✅ Tests de performance OK${NC}"
    else
        echo -e "${RED}❌ Tests de performance échoués${NC}"
        exit 1
    fi
fi

# Audit sécurité
echo -e "\n${YELLOW}🔒 Audit de sécurité...${NC}"
if command -v cargo-audit &> /dev/null; then
    if cargo audit; then
        echo -e "${GREEN}✅ Audit sécurité OK${NC}"
    else
        echo -e "${RED}❌ Vulnérabilités détectées${NC}"
        exit 1
    fi
else
    echo -e "${YELLOW}⚠️ cargo-audit non installé, audit ignoré${NC}"
fi

echo -e "\n${GREEN}🎉 Prêt pour le push !${NC}"
EOF

chmod +x .git/hooks/pre-push
echo -e "${GREEN}✅ Hook pre-push installé${NC}"

# 3. Hook commit-msg pour vérifier le format des messages
echo -e "${BLUE}📝 Création du hook commit-msg...${NC}"
cat > .git/hooks/commit-msg << 'EOF'
#!/bin/bash

# 📝 Hook commit-msg pour vérifier le format des messages de commit
# Format attendu: émoji type: description courte

commit_file="$1"
commit_msg=$(cat "$commit_file")

# Couleurs
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Vérifier que le message n'est pas vide
if [ -z "$(echo "$commit_msg" | tr -d '[:space:]')" ]; then
    echo -e "${RED}❌ Message de commit vide${NC}"
    exit 1
fi

# Vérifier la longueur de la première ligne (titre)
first_line=$(echo "$commit_msg" | head -n1)
if [ ${#first_line} -gt 72 ]; then
    echo -e "${YELLOW}⚠️ Titre du commit trop long (${#first_line} > 72 caractères)${NC}"
    echo "Titre: $first_line"
fi

# Suggérer le format avec émoji si ce n'est pas déjà le cas
if ! echo "$first_line" | grep -qE '^[🐛🚀✨🔧📝🎨⚡🔒📊🧪🔍📋🛠️💡🔄📚🎯]'; then
    echo -e "${YELLOW}💡 Suggestion: Utilisez un émoji au début${NC}"
    echo "Exemples:"
    echo "  🐛 Fix: Correction du bug de validation"
    echo "  ✨ Feat: Nouvelle fonctionnalité Lorem Ipsum"
    echo "  🔧 Chore: Mise à jour des dépendances"
    echo "  📝 Docs: Amélioration de la documentation"
    echo "  🧪 Test: Ajout de tests unitaires"
fi

echo -e "${GREEN}✅ Format de commit validé${NC}"
EOF

chmod +x .git/hooks/commit-msg
echo -e "${GREEN}✅ Hook commit-msg installé${NC}"

# 4. Hook post-commit pour affichage informatif
echo -e "${BLUE}📤 Création du hook post-commit...${NC}"
cat > .git/hooks/post-commit << 'EOF'
#!/bin/bash

# 📤 Hook post-commit pour affichage informatif

# Couleurs
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${GREEN}✅ Commit réussi !${NC}"

# Afficher le hash du commit
commit_hash=$(git rev-parse --short HEAD)
echo -e "${BLUE}📋 Hash: $commit_hash${NC}"

# Afficher le nombre de commits sur la branche
commit_count=$(git rev-list --count HEAD)
echo -e "${BLUE}📊 Total commits: $commit_count${NC}"

# Suggestions utiles
echo -e "${BLUE}💡 Prochaines étapes suggérées:${NC}"
echo "  - git push origin $(git rev-parse --abbrev-ref HEAD)"
echo "  - ./validate_github_setup.sh (vérifier la config)"
echo "  - cargo test (relancer les tests si nécessaire)"
EOF

chmod +x .git/hooks/post-commit
echo -e "${GREEN}✅ Hook post-commit installé${NC}"

echo -e "\n${GREEN}🎉 Installation terminée !${NC}"
echo "================================================="
echo -e "${BLUE}📋 Hooks installés:${NC}"
echo "  ✅ pre-commit   : Formatage, Clippy, tests automatiques"
echo "  ✅ pre-push     : Tests complets et audit sécurité"
echo "  ✅ commit-msg   : Validation format des messages"
echo "  ✅ post-commit  : Informations post-commit"

echo -e "\n${YELLOW}🔧 Configuration:${NC}"
echo "  - Les hooks sont maintenant actifs automatiquement"
echo "  - Pour désactiver temporairement: git commit --no-verify"
echo "  - Pour réinstaller: relancez ce script"

echo -e "\n${BLUE}📖 Aide:${NC}"
echo "  - Format commits recommandé: 'émoji type: description'"
echo "  - Tests automatiques avant chaque commit"
echo "  - Validation qualité code (fmt + clippy)"

echo -e "\n${GREEN}✨ Développement de qualité activé !${NC}"
