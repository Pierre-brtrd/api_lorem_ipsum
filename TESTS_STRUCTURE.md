# 📁 Structure des Tests - Organisation DDD

## 🏗️ Structure Actuelle

```
tests/
├── unit_tests.rs                           # Point d'entrée pour tous les tests unitaires
├── integration.rs                          # Tests d'intégration (existant)
├── performance.rs                          # Tests de performance (existant)
│
└── unit/                                   # Tests unitaires organisés par couche DDD
    ├── mod.rs                              # Module racine des tests unitaires
    │
    └── domain/                             # Tests de la couche domaine
        ├── mod.rs                          # Module domaine
        │
        └── value_objects/                  # Tests des Value Objects
            ├── mod.rs                      # Module Value Objects
            └── test_generation_unit.rs     # Tests de GenerationUnit ✅
            # TODO:
            # ├── test_text_format.rs
            # └── test_text_length.rs
        #
        # TODO: Ajouter quand implémentés
        # ├── entities/
        # │   ├── mod.rs
        # │   └── test_lorem_ipsum_request.rs
        # │
        # └── services/
        #     ├── mod.rs
        #     └── test_generation_service.rs
    #
    # TODO: Ajouter quand implémentées
    # ├── application/                      # Tests de la couche application
    # │   ├── mod.rs
    # │   └── use_cases/
    # │       ├── mod.rs
    # │       └── test_generate_lorem_ipsum.rs
    # │
    # └── infrastructure/                   # Tests de la couche infrastructure
    #     ├── mod.rs
    #     └── web/
    #         ├── mod.rs
    #         └── test_lorem_ipsum_controller.rs
```

## 🎯 Conventions de Nommage

### 📝 Fichiers de Tests

-   **Format** : `test_{nom_du_module}.rs`
-   **Exemples** :
    -   `test_generation_unit.rs` pour `GenerationUnit`
    -   `test_text_format.rs` pour `TextFormat`
    -   `test_lorem_ipsum_request.rs` pour `LoremIpsumRequest`

### 🧪 Modules de Tests

-   **Format** : `{nom}_tests`
-   **Exemple** : `generation_unit_tests` pour grouper les tests de `GenerationUnit`

### ✅ Fonctions de Tests

-   **Format** : `should_{action}_{condition}`
-   **Exemples** :
    -   `should_create_words_unit()`
    -   `should_reject_invalid_strings()`
    -   `should_serialize_to_json()`

## 🚀 Commandes de Test

### 🏃 Tests par Catégorie

```bash
# Tous les tests unitaires
cargo test --test unit_tests

# Tests d'intégration seulement
cargo test --test integration

# Tests de performance seulement
cargo test --test performance -- --ignored
```

### 🔍 Tests Spécifiques

```bash
# Tests d'un Value Object spécifique
cargo test --test unit_tests generation_unit_tests

# Test spécifique
cargo test --test unit_tests should_create_words_unit

# Tests avec pattern
cargo test --test unit_tests should_serialize
```

### 📊 Tests avec Détails

```bash
# Tests avec sortie détaillée
cargo test --test unit_tests -- --nocapture

# Tests avec threads spécifiques
cargo test --test unit_tests -- --test-threads=1
```

## 📋 Checklist Nouveau Value Object

Quand vous créez un nouveau Value Object, suivez cette checklist :

### 1. ✅ Implémentation du Value Object

-   [ ] Créer `src/domain/value_objects/{nom}.rs`
-   [ ] Implémenter les traits requis (`Debug`, `Clone`, `PartialEq`, etc.)
-   [ ] Ajouter la sérialisation Serde si nécessaire
-   [ ] Exposer dans `src/domain/value_objects/mod.rs`

### 2. ✅ Tests Unitaires

-   [ ] Créer `tests/unit/domain/value_objects/test_{nom}.rs`
-   [ ] Implémenter les tests de base :
    -   [ ] `should_create_{variants}()` - Tests de création
    -   [ ] `should_implement_{traits}()` - Tests des traits
    -   [ ] `should_parse_valid_{cases}()` - Tests de parsing valide
    -   [ ] `should_reject_invalid_{cases}()` - Tests de cas invalides
    -   [ ] `should_serialize_to_json()` - Tests de sérialisation
    -   [ ] `should_deserialize_from_json()` - Tests de désérialisation
-   [ ] Ajouter dans `tests/unit/domain/value_objects/mod.rs`

### 3. ✅ Validation

-   [ ] Exécuter `cargo test --test unit_tests {nom}_tests`
-   [ ] Vérifier que tous les tests passent
-   [ ] Corriger les warnings Clippy

## 🎨 Exemple Template

Voici un template pour un nouveau Value Object :

```rust
// tests/unit/domain/value_objects/test_nouveau_value_object.rs
use api_lorem_ipsum::domain::value_objects::nouveau_value_object::NouveauValueObject;
use serde_json;

#[cfg(test)]
mod nouveau_value_object_tests {
    use super::*;

    #[test]
    fn should_create_valid_instance() {
        // Test de création valide
    }

    #[test]
    fn should_implement_equality() {
        // Test d'égalité
    }

    #[test]
    fn should_serialize_to_json() {
        // Test de sérialisation
    }

    #[test]
    fn should_reject_invalid_input() {
        // Test de validation
    }
}
```

## 🔄 Intégration Continue

Les tests unitaires sont automatiquement exécutés dans la CI/CD :

```yaml
# Dans .github/workflows/ci.yml
- name: 🧪 Run tests
  run: |
      # Tests unitaires
      cargo test --test unit_tests --verbose
      # Tests d'intégration  
      cargo test --test integration --verbose
      # Tests de documentation
      cargo test --doc --verbose
```

Cette structure vous garantit une organisation claire et maintenable de vos tests ! 🎯
