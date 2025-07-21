#!/bin/bash

# Script pour préparer une nouvelle release
# Usage: ./prepare_release.sh <version>

set -e

if [ $# -eq 0 ]; then
    echo "❌ Erreur: Version requise"
    echo "Usage: $0 <version>"
    echo "Exemple: $0 0.2.0"
    exit 1
fi

NEW_VERSION="$1"
CURRENT_DATE=$(date +"%Y-%m-%d")

echo "🚀 Préparation de la release v$NEW_VERSION"

# Vérifier que nous sommes sur master
CURRENT_BRANCH=$(git branch --show-current)
if [ "$CURRENT_BRANCH" != "master" ]; then
    echo "⚠️  Attention: Vous n'êtes pas sur la branche master (actuellement sur $CURRENT_BRANCH)"
    read -p "Continuer quand même ? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Vérifier que le working directory est propre
if [ -n "$(git status --porcelain)" ]; then
    echo "❌ Erreur: Le working directory n'est pas propre"
    echo "Commitez ou stashez vos changements avant de continuer"
    exit 1
fi

# Vérifier que tous les tests passent
echo "🧪 Exécution des tests..."
cargo test
if [ $? -ne 0 ]; then
    echo "❌ Erreur: Les tests échouent"
    exit 1
fi

# Vérifier le formatage et les lints
echo "🔍 Vérification du formatage..."
cargo fmt --check
if [ $? -ne 0 ]; then
    echo "❌ Erreur: Le code n'est pas formaté correctement"
    echo "Exécutez: cargo fmt"
    exit 1
fi

echo "📋 Vérification des lints..."
cargo clippy -- -D warnings
if [ $? -ne 0 ]; then
    echo "❌ Erreur: Des warnings clippy sont présents"
    exit 1
fi

# Mettre à jour la version dans Cargo.toml
echo "📦 Mise à jour de la version dans Cargo.toml..."
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    sed -i '' "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml
else
    # Linux
    sed -i "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml
fi

# Vérifier que la version a été mise à jour
NEW_VERSION_CHECK=$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
if [ "$NEW_VERSION_CHECK" != "$NEW_VERSION" ]; then
    echo "❌ Erreur: La version n'a pas été mise à jour correctement"
    exit 1
fi

# Préparer la section du CHANGELOG
echo "📋 Mise à jour du CHANGELOG..."

# Créer une sauvegarde du CHANGELOG
cp CHANGELOG.md CHANGELOG.md.bak

# Créer un nouveau CHANGELOG avec la nouvelle version
cat << EOF > CHANGELOG_new.md
# 📋 Changelog

Toutes les modifications notables de ce projet seront documentées dans ce fichier.

Le format est basé sur [Keep a Changelog](https://keepachangelog.com/fr/1.0.0/),
et ce projet adhère au [Versioning Sémantique](https://semver.org/lang/fr/).

## [Non publié]

### ✨ Ajouté
### 🔄 Modifié
### 🐛 Corrigé
### ❌ Supprimé

## [$NEW_VERSION] - $CURRENT_DATE

### ✨ Ajouté
- Nouvelles fonctionnalités à documenter

### 🔄 Modifié
- Changements à documenter

### 🐛 Corrigé
- Corrections à documenter

EOF

# Ajouter le reste du CHANGELOG (en sautant la section [Non publié] existante)
awk '
    /^## \[Non publié\]/ { skip = 1; next }
    /^## \[/ && skip { skip = 0 }
    !skip { print }
' CHANGELOG.md >> CHANGELOG_new.md

mv CHANGELOG_new.md CHANGELOG.md

echo ""
echo "✅ Préparation terminée !"
echo ""
echo "📝 Prochaines étapes :"
echo "1. Éditez le CHANGELOG.md pour ajouter les vraies notes de release"
echo "2. Vérifiez les changements avec: git diff"
echo "3. Commitez les changements:"
echo "   git add Cargo.toml CHANGELOG.md"
echo "   git commit -m \"🚀 Release v$NEW_VERSION\""
echo "4. Poussez vers master:"
echo "   git push origin master"
echo ""
echo "🤖 Le workflow GitHub Actions créera automatiquement la release !"
echo ""
echo "⚠️  N'oubliez pas de mettre à jour les notes dans CHANGELOG.md avant de commiter !"

# Ouvrir le CHANGELOG dans l'éditeur par défaut (si disponible)
if command -v code &> /dev/null; then
    echo "📝 Ouverture du CHANGELOG dans VS Code..."
    code CHANGELOG.md
elif command -v nano &> /dev/null; then
    echo "📝 Voulez-vous éditer le CHANGELOG maintenant avec nano ? (y/N)"
    read -p "> " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        nano CHANGELOG.md
    fi
fi
