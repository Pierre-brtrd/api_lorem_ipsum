# 🧪 Guide des Tests - Commandes Pratiques

## 🚀 Commandes Essentielles

### 📋 Tests par Type

```bash
# Tests unitaires (organisation DDD)
cargo test --test unit_tests

# Tests d'intégration (flux complets)
cargo test --test integration

# Tests de performance (charge et benchmarks)
cargo test --test performance -- --ignored

# Tous les tests (sauf performance)
cargo test
```

### 🎯 Tests Spécifiques

```bash
# Tests d'un Value Object
cargo test --test unit_tests generation_unit_tests

# Test spécifique par nom
cargo test --test unit_tests should_create_words_unit

# Tests avec pattern de recherche
cargo test --test unit_tests should_serialize
```

### 📊 Tests avec Options

```bash
# Tests avec sortie détaillée
cargo test --test unit_tests -- --nocapture

# Tests en mode verbeux
cargo test --test unit_tests --verbose

# Tests avec un seul thread (pour debug)
cargo test --test unit_tests -- --test-threads=1

# Tests ignorés seulement
cargo test --test performance -- --ignored --nocapture
```

## 🔍 Tests Coverage

### 📈 Générer la Coverage

```bash
# Installation de cargo-llvm-cov (une seule fois)
cargo install cargo-llvm-cov

# Coverage complète avec rapport HTML
cargo llvm-cov --all-features --workspace --html

# Coverage pour tests unitaires seulement
cargo llvm-cov --test unit_tests --html

# Coverage en format LCOV (pour CI)
cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info
```

### 📊 Visualiser les Résultats

```bash
# Ouvrir le rapport HTML
open target/llvm-cov/html/index.html

# Coverage dans le terminal
cargo llvm-cov --all-features --workspace
```

## ⚡ Tests de Performance

### 🏃 Exécution

```bash
# Tests de performance basiques
cargo test --test performance -- --ignored

# Tests avec temps de timeout étendu
cargo test --test performance -- --ignored --test-threads=1

# Benchmarks avec Criterion (si implémenté)
cargo bench
```

## 🔧 Debug et Troubleshooting

### 🐛 Debug des Tests

```bash
# Tests avec debug info
RUST_LOG=debug cargo test --test unit_tests -- --nocapture

# Tests avec backtraces
RUST_BACKTRACE=1 cargo test --test unit_tests

# Tests avec backtraces complètes
RUST_BACKTRACE=full cargo test --test unit_tests
```

### 🔍 Vérification des Tests

```bash
# Vérifier que les tests compilent
cargo check --tests

# Linter sur les tests
cargo clippy --tests

# Formatage des tests
cargo fmt --all
```

## 🎪 CI/CD - Tests Automatiques

### 🤖 Commandes CI Locales

```bash
# Simulation CI complète
./scripts/run_ci_tests.sh

# Tests comme dans GitHub Actions
cargo test --all-features --workspace --verbose
```

### 📋 Validation Complète

```bash
# Script de validation complète (à créer)
#!/bin/bash
echo "🧪 Tests unitaires..."
cargo test --test unit_tests

echo "🔗 Tests d'intégration..."
cargo test --test integration

echo "📊 Coverage..."
cargo llvm-cov --all-features --workspace

echo "📋 Clippy..."
cargo clippy --all-targets --all-features

echo "🔍 Formatage..."
cargo fmt --all -- --check

echo "✅ Tous les tests passent !"
```

## 📁 Organisation des Résultats

### 📊 Fichiers Générés

```
target/
├── llvm-cov/
│   ├── html/           # Rapports HTML de coverage
│   └── lcov.info       # Format LCOV pour CI
├── criterion/          # Benchmarks (si configuré)
└── debug/deps/         # Binaires de tests compilés
```

### 🗂️ Logs et Rapports

```bash
# Sauvegarder les résultats de tests
cargo test --test unit_tests 2>&1 | tee test_results.log

# Coverage avec timestamp
cargo llvm-cov --all-features --workspace > coverage_$(date +%Y%m%d).txt
```

## 🎯 Workflow de Développement

### 🔄 TDD Cycle

```bash
# 1. Écrire le test (qui échoue)
cargo test --test unit_tests new_feature_test

# 2. Implémenter le minimum pour passer
cargo test --test unit_tests new_feature_test

# 3. Refactorer
cargo test --test unit_tests

# 4. Coverage pour vérifier
cargo llvm-cov --test unit_tests --html
```

### ⚡ Tests Rapides (Développement)

```bash
# Tests unitaires seulement (rapide)
cargo test --test unit_tests --lib

# Tests du module en cours
cargo test --test unit_tests generation_unit

# Watch mode (avec cargo-watch)
cargo watch -x "test --test unit_tests"
```

## 🏆 Best Practices

### ✅ Avant Commit

```bash
# Checklist rapide
cargo test --test unit_tests
cargo clippy --tests
cargo fmt --all -- --check
```

### ✅ Avant Push

```bash
# Validation complète
cargo test --all-features --workspace
cargo llvm-cov --all-features --workspace
```

### ✅ Release

```bash
# Tests complets avec performance
cargo test --all-features --workspace
cargo test --test performance -- --ignored
cargo llvm-cov --all-features --workspace --lcov
```

Cette approche vous garantit une suite de tests robuste et maintenable ! 🚀
