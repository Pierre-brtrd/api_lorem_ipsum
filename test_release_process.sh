#!/bin/bash

# Script de test pour valider le processus de release
# Usage: ./test_release_process.sh

echo "🧪 Test du processus de release"
echo "================================"

# Tester le formatage
echo "📝 Test du formatage..."
cargo fmt --check
if [ $? -eq 0 ]; then
    echo "✅ Formatage OK"
else
    echo "❌ Formatage échoué"
    exit 1
fi

# Tester les lints
echo "📋 Test des lints..."
cargo clippy -- -D warnings
if [ $? -eq 0 ]; then
    echo "✅ Clippy OK"
else
    echo "❌ Clippy échoué"
    exit 1
fi

# Tester la compilation
echo "🏗️ Test de compilation..."
cargo build
if [ $? -eq 0 ]; then
    echo "✅ Compilation OK"
else
    echo "❌ Compilation échouée"
    exit 1
fi

# Tester les tests
echo "🧪 Test des tests..."
cargo test
if [ $? -eq 0 ]; then
    echo "✅ Tests OK"
else
    echo "❌ Tests échoués"
    exit 1
fi

# Vérifier le CHANGELOG
echo "📋 Vérification du CHANGELOG..."
if [ ! -f "CHANGELOG.md" ]; then
    echo "❌ CHANGELOG.md manquant"
    exit 1
fi

if ! grep -q "## \[Non publié\]" CHANGELOG.md; then
    echo "❌ Section [Non publié] manquante dans CHANGELOG.md"
    exit 1
fi

echo "✅ CHANGELOG OK"

# Vérifier la structure Git
echo "🔍 Vérification Git..."
if [ ! -d ".git" ]; then
    echo "❌ Pas un repository Git"
    exit 1
fi

CURRENT_BRANCH=$(git branch --show-current)
if [ "$CURRENT_BRANCH" != "master" ]; then
    echo "⚠️ Pas sur la branche master (actuellement: $CURRENT_BRANCH)"
fi

if [ -n "$(git status --porcelain)" ]; then
    echo "⚠️ Working directory non propre"
    git status --porcelain
fi

echo "✅ Git OK"

# Vérifier les workflows GitHub
echo "🤖 Vérification des workflows GitHub..."
for workflow in .github/workflows/*.yml; do
    if [ -f "$workflow" ]; then
        echo "✅ $workflow existe"
    else
        echo "❌ $workflow manquant"
    fi
done

# Tester l'extraction de version
echo "📦 Test d'extraction de version..."
VERSION=$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
if [ -n "$VERSION" ]; then
    echo "✅ Version détectée: $VERSION"
else
    echo "❌ Impossible d'extraire la version"
    exit 1
fi

# Vérifier que le script de release existe
echo "🚀 Vérification du script de release..."
if [ -f "prepare_release.sh" ] && [ -x "prepare_release.sh" ]; then
    echo "✅ prepare_release.sh prêt"
else
    echo "❌ prepare_release.sh manquant ou non exécutable"
    exit 1
fi

echo ""
echo "🎉 Tous les tests passent !"
echo "✅ Le projet est prêt pour les releases automatiques"
echo ""
echo "🚀 Pour créer une release :"
echo "   1. ./prepare_release.sh <version>"
echo "   2. Éditer CHANGELOG.md"
echo "   3. git add Cargo.toml CHANGELOG.md"
echo "   4. git commit -m '🚀 Release v<version>'"
echo "   5. git push origin master"
