# Guide de Contribution

Merci de votre intérêt pour contribuer à l'API Lorem Ipsum ! 🎉

## Comment Contribuer

### 1. Fork et Clone

```bash
git clone https://github.com/Pierre-brtrd/api_lorem_ipsum.git
cd api_lorem_ipsum
```

### 2. Configuration de l'Environnement

```bash
# Installer Rust si ce n'est pas déjà fait
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Créer la structure de projet
chmod +x setup_structure.sh
./setup_structure.sh

# Vérifier que tout compile
cargo check
```

### 3. Développement

#### Standards de Code

-   Suivre les conventions Rust standard (`cargo fmt`)
-   Utiliser `cargo clippy` pour les lints
-   Maintenir une couverture de tests élevée
-   Documenter les API publiques

#### Architecture DDD

-   Respecter la séparation des couches (Domain, Application, Infrastructure)
-   Les entités du domaine ne doivent pas dépendre de l'infrastructure
-   Utiliser l'injection de dépendance via les traits

#### Tests

```bash
# Lancer tous les tests
cargo test

# Tests avec couverture
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

### 4. Processus de Pull Request

1. Créer une branche pour votre feature : `git checkout -b feature/nouvelle-fonctionnalite`
2. Commiter vos changements : `git commit -m "feat: description de la fonctionnalité"`
3. Pusher vers votre fork : `git push origin feature/nouvelle-fonctionnalite`
4. Ouvrir une Pull Request

#### Convention de Commits

Nous utilisons [Conventional Commits](https://www.conventionalcommits.org/) :

-   `feat:` nouvelle fonctionnalité
-   `fix:` correction de bug
-   `docs:` documentation
-   `style:` formatage, style
-   `refactor:` refactoring
-   `test:` ajout de tests
-   `chore:` tâches de maintenance

## Types de Contributions

### 🐛 Rapports de Bugs

-   Utiliser les templates d'issues
-   Inclure les étapes de reproduction
-   Spécifier l'environnement (OS, version Rust)

### 💡 Nouvelles Fonctionnalités

-   Ouvrir d'abord une issue pour discussion
-   Respecter l'architecture DDD
-   Ajouter des tests appropriés

### 📚 Documentation

-   Améliorer le README
-   Ajouter des exemples d'utilisation
-   Documenter l'API

### 🧪 Tests

-   Augmenter la couverture de tests
-   Ajouter des tests d'intégration
-   Améliorer les tests existants

## Règles du Code

### Domaine (Domain Layer)

```rust
// ✅ Bon : Entité pure sans dépendances externes
pub struct LoremText {
    id: TextId,
    content: String,
    length: TextLength,
}

// ❌ Mauvais : Dépendance vers l'infrastructure
use crate::infrastructure::database::Database;
```

### Application (Application Layer)

```rust
// ✅ Bon : Use case avec injection de dépendance
pub struct GenerateLoremTextUseCase<R: LoremRepository> {
    repository: R,
}

// ❌ Mauvais : Dépendance directe vers l'implémentation
use crate::infrastructure::repositories::SqlLoremRepository;
```

### Infrastructure (Infrastructure Layer)

```rust
// ✅ Bon : Implémentation du trait du domaine
impl LoremRepository for InMemoryLoremRepository {
    // implémentation
}
```

## Questions ?

-   Ouvrir une issue avec le label `question`
-   Consulter la documentation dans le README
-   Regarder les exemples existants

Merci pour votre contribution ! 🚀
