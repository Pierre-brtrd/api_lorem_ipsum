#!/bin/bash

# Script pour créer la structure DDD de l'API Lorem Ipsum

echo "🏗️  Création de la structure DDD pour l'API Lorem Ipsum..."

# Création de la structure des dossiers
mkdir -p src/domain/{entities,value_objects,repositories,services,errors}
mkdir -p src/application/{use_cases,dto,ports}
mkdir -p src/infrastructure/{repositories,web/{routes,handlers,middleware},persistence,config}
mkdir -p src/shared/{errors,utils}

# Création des fichiers mod.rs
echo "pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod shared;" > src/lib.rs

# Domain layer
echo "pub mod entities;
pub mod value_objects;
pub mod repositories;
pub mod services;
pub mod errors;" > src/domain/mod.rs

echo "pub mod lorem_text;
pub mod generation_request;
pub mod text_fragment;" > src/domain/entities/mod.rs

echo "pub mod text_length;
pub mod generation_unit;
pub mod text_format;" > src/domain/value_objects/mod.rs

echo "pub mod lorem_repository;" > src/domain/repositories/mod.rs

echo "pub mod text_generator;
pub mod text_validator;" > src/domain/services/mod.rs

echo "pub mod domain_errors;" > src/domain/errors/mod.rs

# Application layer
echo "pub mod use_cases;
pub mod dto;
pub mod ports;" > src/application/mod.rs

echo "pub mod generate_lorem_text;
pub mod validate_request;" > src/application/use_cases/mod.rs

echo "pub mod generation_request_dto;
pub mod generation_response_dto;" > src/application/dto/mod.rs

echo "pub mod text_generation_port;" > src/application/ports/mod.rs

# Infrastructure layer
echo "pub mod repositories;
pub mod web;
pub mod persistence;
pub mod config;" > src/infrastructure/mod.rs

echo "pub mod in_memory_lorem_repository;" > src/infrastructure/repositories/mod.rs

echo "pub mod routes;
pub mod handlers;
pub mod middleware;" > src/infrastructure/web/mod.rs

echo "pub mod lorem_routes;" > src/infrastructure/web/routes/mod.rs

echo "pub mod lorem_handler;" > src/infrastructure/web/handlers/mod.rs

echo "pub mod cors;
pub mod rate_limiting;" > src/infrastructure/web/middleware/mod.rs

echo "pub mod lorem_data;" > src/infrastructure/persistence/mod.rs

echo "pub mod app_config;" > src/infrastructure/config/mod.rs

# Shared layer
echo "pub mod errors;
pub mod utils;" > src/shared/mod.rs

echo "pub mod common_errors;" > src/shared/errors/mod.rs

echo "pub mod text_utils;" > src/shared/utils/mod.rs

echo "✅ Structure DDD créée avec succès!"
echo ""
echo "📁 Structure créée:"
echo "src/"
echo "├── lib.rs"
echo "├── main.rs"
echo "├── domain/"
echo "│   ├── entities/"
echo "│   ├── value_objects/"
echo "│   ├── repositories/"
echo "│   ├── services/"
echo "│   └── errors/"
echo "├── application/"
echo "│   ├── use_cases/"
echo "│   ├── dto/"
echo "│   └── ports/"
echo "├── infrastructure/"
echo "│   ├── repositories/"
echo "│   ├── web/"
echo "│   ├── persistence/"
echo "│   └── config/"
echo "└── shared/"
echo "    ├── errors/"
echo "    └── utils/"
echo ""
echo "🚀 Prochaines étapes:"
echo "1. Exécuter: chmod +x setup_structure.sh && ./setup_structure.sh"
echo "2. Exécuter: cargo check pour vérifier les dépendances"
echo "3. Commencer l'implémentation par les entités du domaine"
