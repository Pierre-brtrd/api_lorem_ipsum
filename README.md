# API Lorem Ipsum - Rust DDD

## Description du Projet

Une API REST moderne développée en Rust pour générer du texte Lorem Ipsum, construite selon les principes du Domain-Driven Design (DDD).

## Ubiquitous Language (Langage Ubiquitaire)

### Concepts Métier Principaux

#### **Text Generation Domain (Domaine de Génération de Texte)**

-   **Lorem Text** : Texte fictif généré basé sur le célèbre passage du Lorem Ipsum
-   **Text Fragment** : Unité atomique de texte (mot, phrase ou paragraphe)
-   **Generation Request** : Demande de génération de texte avec des paramètres spécifiques
-   **Text Length** : Longueur du texte à générer (en mots, phrases ou paragraphes)
-   **Generation Unit** : Unité de mesure pour la génération (WORDS, SENTENCES, PARAGRAPHS)
-   **Text Format** : Format de sortie du texte (PLAIN, HTML, MARKDOWN)

#### **Content Management (Gestion du Contenu)**

-   **Lorem Dictionary** : Dictionnaire contenant les mots sources du Lorem Ipsum
-   **Text Template** : Modèle de base pour la génération de texte
-   **Content Provider** : Service responsable de fournir le contenu de base
-   **Text Validator** : Service de validation du texte généré

#### **API Management (Gestion de l'API)**

-   **Generation Endpoint** : Point d'entrée de l'API pour les demandes de génération
-   **Request Parameters** : Paramètres de la requête (count, unit, format, etc.)
-   **Response Payload** : Charge utile de la réponse contenant le texte généré
-   **Rate Limiting** : Limitation du taux de requêtes par utilisateur

#### **Quality Assurance (Assurance Qualité)**

-   **Text Quality** : Qualité du texte généré (cohérence, lisibilité)
-   **Generation Metrics** : Métriques de performance de génération
-   **Content Integrity** : Intégrité du contenu généré

### Règles Métier

1. **Génération de Texte** :

    - Le texte généré doit toujours commencer par "Lorem ipsum"
    - La longueur minimum est de 1 unité
    - La longueur maximum est de 1000 unités par requête
    - Le texte doit respecter la structure grammaticale de base

2. **Formats de Sortie** :

    - PLAIN : Texte brut sans formatage
    - HTML : Texte avec balises HTML appropriées
    - MARKDOWN : Texte avec syntaxe Markdown

3. **Validation** :
    - Les paramètres de requête doivent être validés
    - Les erreurs doivent être explicites et informatives

## Architecture DDD Proposée

```
src/
├── main.rs                          # Point d'entrée de l'application
├── lib.rs                           # Module racine
│
├── domain/                          # Couche Domaine
│   ├── mod.rs
│   ├── entities/                    # Entités métier
│   │   ├── mod.rs
│   │   ├── lorem_text.rs           # Entité LoremText
│   │   ├── generation_request.rs   # Entité GenerationRequest
│   │   └── text_fragment.rs        # Entité TextFragment
│   ├── value_objects/              # Objets Valeur
│   │   ├── mod.rs
│   │   ├── text_length.rs          # Longueur du texte
│   │   ├── generation_unit.rs      # Unité de génération
│   │   └── text_format.rs          # Format du texte
│   ├── repositories/               # Interfaces des repositories
│   │   ├── mod.rs
│   │   └── lorem_repository.rs     # Interface du repository
│   ├── services/                   # Services du domaine
│   │   ├── mod.rs
│   │   ├── text_generator.rs       # Service de génération
│   │   └── text_validator.rs       # Service de validation
│   └── errors/                     # Erreurs du domaine
│       ├── mod.rs
│       └── domain_errors.rs
│
├── application/                     # Couche Application
│   ├── mod.rs
│   ├── use_cases/                  # Cas d'usage
│   │   ├── mod.rs
│   │   ├── generate_lorem_text.rs  # Cas d'usage principal
│   │   └── validate_request.rs     # Validation des requêtes
│   ├── dto/                        # Data Transfer Objects
│   │   ├── mod.rs
│   │   ├── generation_request_dto.rs
│   │   └── generation_response_dto.rs
│   └── ports/                      # Ports (interfaces)
│       ├── mod.rs
│       └── text_generation_port.rs
│
├── infrastructure/                  # Couche Infrastructure
│   ├── mod.rs
│   ├── repositories/               # Implémentations des repositories
│   │   ├── mod.rs
│   │   └── in_memory_lorem_repository.rs
│   ├── web/                        # Framework web
│   │   ├── mod.rs
│   │   ├── routes/
│   │   │   ├── mod.rs
│   │   │   └── lorem_routes.rs
│   │   ├── handlers/
│   │   │   ├── mod.rs
│   │   │   └── lorem_handler.rs
│   │   └── middleware/
│   │       ├── mod.rs
│   │       ├── cors.rs
│   │       └── rate_limiting.rs
│   ├── persistence/                # Persistance des données
│   │   ├── mod.rs
│   │   └── lorem_data.rs           # Données de base Lorem Ipsum
│   └── config/                     # Configuration
│       ├── mod.rs
│       └── app_config.rs
│
└── shared/                         # Code partagé
    ├── mod.rs
    ├── errors/                     # Erreurs communes
    │   ├── mod.rs
    │   └── common_errors.rs
    └── utils/                      # Utilitaires
        ├── mod.rs
        └── text_utils.rs
```

## Dépendances Recommandées

### Web Framework et HTTP

-   **axum** : Framework web moderne et performant
-   **tokio** : Runtime asynchrone
-   **tower** : Middleware et services
-   **tower-http** : Middleware HTTP (CORS, logging, etc.)

### Sérialisation

-   **serde** : Sérialisation/désérialisation
-   **serde_json** : Support JSON

### Validation et Erreurs

-   **validator** : Validation des données
-   **thiserror** : Gestion d'erreurs ergonomique
-   **anyhow** : Gestion d'erreurs simplifiée

### Configuration et Logging

-   **config** : Gestion de configuration
-   **tracing** : Logging structuré
-   **tracing-subscriber** : Abonnements aux logs

### Utilitaires

-   **uuid** : Génération d'UUID
-   **rand** : Génération de nombres aléatoires
-   **once_cell** : Initialisation paresseuse

### Tests

-   **mockall** : Mocking pour les tests
-   **rstest** : Tests paramétrés

## Prochaines Étapes

1. **Mise à jour du Cargo.toml** avec les dépendances
2. **Création de la structure de dossiers**
3. **Implémentation des entités du domaine**
4. **Développement des cas d'usage**
5. **Mise en place de l'infrastructure web**
6. **Tests et documentation**

## Endpoints API Prévus

```
GET  /api/v1/lorem                   # Génération avec paramètres par défaut
GET  /api/v1/lorem/{count}           # Génération avec compte spécifique
POST /api/v1/lorem/generate          # Génération avec paramètres complexes
GET  /api/v1/health                  # Health check
```

## Exemples d'Utilisation

```bash
# Générer 5 paragraphes
GET /api/v1/lorem/5?unit=paragraphs&format=html

# Générer 100 mots en markdown
GET /api/v1/lorem/100?unit=words&format=markdown
```
