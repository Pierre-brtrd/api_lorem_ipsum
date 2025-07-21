# 🚀 Guide de Release Automatique

Ce document explique comment fonctionne le système de release automatique de l'API Lorem Ipsum.

## 🔄 Processus Automatique

### Déclenchement

Le workflow de release se déclenche automatiquement quand :

1. **Un commit est pushé sur `master`**
2. **Le fichier `Cargo.toml` OU `CHANGELOG.md` est modifié**
3. **La version dans `Cargo.toml` a changé**
4. **Tous les tests CI passent**

### Étapes du Workflow

1. **🔍 Détection de Version**

    - Compare la version actuelle avec la précédente
    - Extrait les notes de release du CHANGELOG
    - Détermine si c'est une pre-release (alpha, beta, rc)

2. **🏗️ Build de Release**

    - Compile en mode release
    - Exécute tous les tests
    - Crée une archive binaire avec checksum

3. **🏷️ Création du Tag et Release**

    - Crée un tag Git `v{version}`
    - Génère une release GitHub avec notes détaillées
    - Upload des artifacts (binaire + checksum)

4. **📢 Notification**
    - Crée une issue d'annonce de release
    - Notifie la communauté

## 🛠️ Utilisation Manuelle

### Script de Préparation

Utilisez le script `prepare_release.sh` pour préparer une release :

```bash
# Préparer la version 0.2.0
./prepare_release.sh 0.2.0
```

Le script va :

-   ✅ Vérifier que vous êtes sur `master`
-   ✅ Vérifier que le working directory est propre
-   ✅ Exécuter les tests et vérifications
-   ✅ Mettre à jour la version dans `Cargo.toml`
-   ✅ Préparer une nouvelle section dans `CHANGELOG.md`
-   ✅ Ouvrir l'éditeur pour compléter les notes

### Étapes Manuelles

1. **Préparer la release**

    ```bash
    ./prepare_release.sh 1.0.0
    ```

2. **Éditer le CHANGELOG**

    - Complétez la section `[1.0.0]` avec vos changements
    - Supprimez les sections vides
    - Vérifiez le formatage

3. **Commiter et pousser**

    ```bash
    git add Cargo.toml CHANGELOG.md
    git commit -m "🚀 Release v1.0.0"
    git push origin master
    ```

4. **🤖 Laisser le workflow faire le reste !**

## 📋 Format du CHANGELOG

Le CHANGELOG doit suivre le format [Keep a Changelog](https://keepachangelog.com/) :

```markdown
## [1.0.0] - 2025-07-21

### ✨ Ajouté

-   Nouvelle fonctionnalité X
-   Support pour Y

### 🔄 Modifié

-   Amélioration de Z
-   Changement de comportement de W

### 🐛 Corrigé

-   Correction du bug A
-   Fix de la vulnérabilité B

### ❌ Supprimé

-   Ancien endpoint déprécié
```

### Types de Changements

-   ✨ **Ajouté** - Nouvelles fonctionnalités
-   🔄 **Modifié** - Changements dans les fonctionnalités existantes
-   ⚠️ **Déprécié** - Fonctionnalités qui seront supprimées
-   ❌ **Supprimé** - Fonctionnalités supprimées
-   🐛 **Corrigé** - Corrections de bugs
-   🔒 **Sécurité** - Corrections de vulnérabilités

## 🔍 Validation Automatique

### Sur les Pull Requests

Le workflow `pr-validation.yml` vérifie :

-   ✅ **Code formaté** (`cargo fmt`)
-   ✅ **Lints propres** (`cargo clippy`)
-   ✅ **Tests passants** (`cargo test`)
-   ✅ **CHANGELOG mis à jour** (si code source modifié)
-   ✅ **Taille de la PR** (avec labels automatiques)

### Commentaires Automatiques

-   📋 **CHANGELOG manquant** - Si du code est modifié sans mise à jour du CHANGELOG
-   📏 **PR trop grande** - Si la PR contient plus de 500 changements

## 🎯 Versioning Sémantique

Nous suivons [SemVer](https://semver.org/) :

-   **MAJOR** (1.0.0) - Changements incompatibles
-   **MINOR** (0.1.0) - Nouvelles fonctionnalités compatibles
-   **PATCH** (0.0.1) - Corrections de bugs compatibles

### Pre-releases

-   **alpha** (1.0.0-alpha.1) - Version très précoce
-   **beta** (1.0.0-beta.1) - Version de test
-   **rc** (1.0.0-rc.1) - Release candidate

## 📦 Artifacts de Release

Chaque release contient :

-   **📁 Archive binaire** (`api_lorem_ipsum-{version}-x86_64-unknown-linux-gnu.tar.gz`)
-   **🔐 Checksum SHA256** (`.sha256`)
-   **📚 Documentation** (README, CHANGELOG, LICENSE)
-   **📝 Notes détaillées** avec instructions d'installation

## 🐛 Dépannage

### La release ne se déclenche pas

1. Vérifiez que la version a changé dans `Cargo.toml`
2. Vérifiez que vous êtes sur la branche `master`
3. Vérifiez que les tests CI passent
4. Vérifiez les logs du workflow dans l'onglet Actions

### Erreur lors de la création du tag

-   Le tag existe peut-être déjà
-   Vérifiez les permissions du `GITHUB_TOKEN`

### Artifacts manquants

-   Vérifiez que la compilation release fonctionne localement
-   Vérifiez les logs de build dans le workflow

## 📞 Support

Si vous rencontrez des problèmes :

1. Consultez les [logs des workflows](https://github.com/Pierre-brtrd/api_lorem_ipsum/actions)
2. Créez une [issue](https://github.com/Pierre-brtrd/api_lorem_ipsum/issues) avec le label `ci/cd`
3. Contactez [@Pierre-brtrd](https://github.com/Pierre-brtrd)

---

**Dernière mise à jour** : 21 juillet 2025
