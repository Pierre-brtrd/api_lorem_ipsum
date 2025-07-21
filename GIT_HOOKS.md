# 🪝 Git Hooks - API Lorem Ipsum

## 📋 Vue d'ensemble

Les Git hooks sont des scripts automatiques qui s'exécutent à des moments clés du workflow Git. Ils garantissent la qualité du code et automatisent les vérifications.

## 🛠️ Installation Rapide

```bash
# Installation complète des hooks
./scripts/install-hooks.sh

# Désinstallation si nécessaire
./scripts/uninstall-hooks.sh
```

## 🔧 Hooks Installés

### 🔍 Pre-commit Hook
**Déclencheur** : Avant chaque `git commit`  
**Durée** : ~30 secondes  
**Objectif** : Vérifications de qualité automatiques

#### ✅ Vérifications effectuées :
1. **📝 Formatage automatique** : `cargo fmt`
2. **🔍 Validation formatage** : Code conforme aux standards Rust
3. **📋 Clippy linting** : Détection des erreurs et mauvaises pratiques
4. **🏗️ Compilation** : Le code compile sans erreur
5. **🧪 Tests** : Tests unitaires et d'intégration passent
6. **🔒 Détection secrets** : Évite les commits de tokens/clés API
7. **📋 TODO/FIXME** : Avertissement sur branche master

#### 🚫 Bypass temporaire :
```bash
git commit --no-verify -m "Commit urgent sans vérifications"
```

### 🚀 Pre-push Hook
**Déclencheur** : Avant chaque `git push`  
**Durée** : ~2-5 minutes  
**Objectif** : Vérifications complètes avant publication

#### ✅ Vérifications effectuées :
1. **🧪 Tests complets** : Unitaires + intégration
2. **⚡ Tests performance** : (optionnel, sur demande)
3. **🔒 Audit sécurité** : `cargo audit` pour vulnérabilités

#### 🚫 Bypass temporaire :
```bash
git push --no-verify origin master
```

### 📝 Commit-msg Hook
**Déclencheur** : Lors de la création du message de commit  
**Durée** : Instantané  
**Objectif** : Format et qualité des messages

#### ✅ Vérifications effectuées :
1. **📏 Longueur** : Titre ≤ 72 caractères
2. **📝 Non-vide** : Message obligatoire
3. **🎨 Format émoji** : Suggestion d'émojis pour catégoriser

#### 💡 Format recommandé :
```
🐛 Fix: Correction du bug de validation des emails
✨ Feat: Nouvelle API endpoint pour génération avancée
🔧 Chore: Mise à jour dépendances Cargo
📝 Docs: Amélioration documentation DDD
🧪 Test: Ajout tests de performance
```

### 📤 Post-commit Hook
**Déclencheur** : Après chaque commit réussi  
**Durée** : Instantané  
**Objectif** : Informations et suggestions

#### ℹ️ Informations affichées :
1. **📋 Hash commit** : Identifiant court
2. **📊 Nombre total** : Commits sur la branche
3. **💡 Suggestions** : Prochaines étapes

## 🎯 Émojis de Commit Recommandés

| Émoji | Type | Usage |
|-------|------|-------|
| 🐛 | Fix | Correction de bugs |
| ✨ | Feat | Nouvelles fonctionnalités |
| 🔧 | Chore | Maintenance, config |
| 📝 | Docs | Documentation |
| 🧪 | Test | Tests unitaires/intégration |
| ⚡ | Perf | Optimisations performance |
| 🔒 | Security | Sécurité |
| 🎨 | Style | Formatage, UI |
| 🔄 | Refactor | Refactoring code |
| 📦 | Build | Système de build |
| 🚀 | Deploy | Déploiement |
| 💡 | Idea | Travail en cours |

## 🔄 Workflow Recommandé

### 📅 Développement quotidien
```bash
# 1. Développement
echo "nouveau code" >> src/main.rs

# 2. Commit (hooks automatiques)
git add .
git commit -m "✨ Feat: nouvelle fonctionnalité"
# ↳ Pre-commit hook s'exécute automatiquement

# 3. Push (vérifications complètes)
git push origin feature-branch
# ↳ Pre-push hook s'exécute automatiquement
```

### 🚨 En cas d'échec des hooks

#### ❌ Pre-commit échoue
```bash
# 1. Voir les erreurs détaillées
cargo fmt      # Corriger le formatage
cargo clippy   # Voir les warnings Clippy
cargo test     # Exécuter les tests

# 2. Corriger et recommiter
git add .
git commit -m "🔧 Fix: correction erreurs pre-commit"
```

#### ❌ Pre-push échoue
```bash
# 1. Tests d'intégration échouent
cargo test --test integration

# 2. Audit sécurité échoue
cargo audit
cargo audit fix  # Mise à jour automatique si possible

# 3. Corriger et repush
git push origin branch-name
```

## ⚙️ Configuration Avancée

### 🎛️ Personnalisation Pre-commit
Éditez `scripts/pre-commit.sh` pour :
- Ajouter des vérifications spécifiques
- Modifier les seuils de performance
- Personnaliser les messages

### 🔧 Variables d'environnement
```bash
# Désactiver certaines vérifications (développement)
export SKIP_CLIPPY=1        # Ignorer Clippy
export SKIP_TESTS=1         # Ignorer tests
export SKIP_AUDIT=1         # Ignorer audit sécurité

# Puis commiter normalement
git commit -m "🧪 Test: commit sans vérifications complètes"
```

### 📋 Hooks par projet
Créez `.git/hooks/local-config` :
```bash
#!/bin/bash
# Configuration locale spécifique au développeur
export RUST_LOG=debug
export SKIP_PERFORMANCE_TESTS=1
```

## 🛡️ Sécurité et Bonnes Pratiques

### 🔒 Détection de secrets
Le pre-commit hook détecte :
- `API_KEY`, `SECRET`, `TOKEN`, `PASSWORD`
- Patterns de clés SSH, AWS, etc.
- URLs avec credentials

### 📋 Vérifications qualité
- **Code coverage** : Maintenir > 80%
- **Performance** : Tests de régression
- **Sécurité** : Audit dependencies automatique
- **Style** : Formatage uniforme

### 🎯 Workflow équipe
```bash
# Setup initial pour nouveaux développeurs
git clone https://github.com/Pierre-brtrd/api_lorem_ipsum.git
cd api_lorem_ipsum
./scripts/install-hooks.sh   # Installation automatique

# Développement avec qualité garantie
# → Hooks s'exécutent automatiquement
# → Pas de code de mauvaise qualité en production
```

## 🚨 Dépannage

### ❓ Hooks ne s'exécutent pas
```bash
# Vérifier l'installation
ls -la .git/hooks/

# Réinstaller si nécessaire
./scripts/install-hooks.sh
```

### ❓ Hooks trop lents
```bash
# Optimiser les tests
export SKIP_INTEGRATION_TESTS=1

# Ou bypass ponctuel
git commit --no-verify
```

### ❓ Erreurs de permissions
```bash
# Corriger les permissions
chmod +x .git/hooks/*
chmod +x scripts/*.sh
```

## 📊 Métriques et Monitoring

### 📈 Temps d'exécution moyens
- **Pre-commit** : 20-45 secondes
- **Pre-push** : 2-5 minutes
- **Commit-msg** : < 1 seconde
- **Post-commit** : < 1 seconde

### 🎯 Objectifs qualité
- **100%** des commits passent les vérifications
- **< 1%** de bypass des hooks
- **0** secrets committés
- **> 90%** satisfaction développeurs

## 🎉 Avantages

✅ **Qualité garantie** : Code toujours propre et testé  
✅ **Automatisation** : Pas d'oubli de vérifications  
✅ **Feedback rapide** : Erreurs détectées immédiatement  
✅ **Sécurité** : Protection contre les secrets  
✅ **Uniformité** : Style de code cohérent équipe  
✅ **Documentation** : Messages de commit structurés  

Les Git hooks transforment votre workflow de développement en garantissant automatiquement la qualité ! 🚀
