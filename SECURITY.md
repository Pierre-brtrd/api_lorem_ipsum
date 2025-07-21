# 🔒 Politique de Sécurité

## 🚨 Signalement des Vulnérabilités de Sécurité

Nous prenons la sécurité de l'API Lorem Ipsum très au sérieux. Si vous découvrez une vulnérabilité de sécurité, merci de nous la signaler de manière responsable.

### 📧 Comment Signaler

**NE PAS** créer d'issue publique pour les vulnérabilités de sécurité.

**À la place, merci d'envoyer un e-mail à :** [pierre.bertrand.professionnel@gmail.com]

Incluez les informations suivantes dans votre rapport :

-   **Description** : Description détaillée de la vulnérabilité
-   **Impact** : Impact potentiel et sévérité
-   **Reproduction** : Étapes pour reproduire le problème
-   **Version** : Version de l'API affectée
-   **Environnement** : Détails de l'environnement (OS, Rust version, etc.)

### 🔄 Processus de Réponse

1. **Accusé de réception** : Nous accuserons réception de votre rapport dans les **48 heures**
2. **Évaluation** : Nous évaluerons et confirmerons la vulnérabilité dans les **7 jours**
3. **Développement** : Nous développerons un correctif si nécessaire
4. **Test** : Nous testerons le correctif de manière approfondie
5. **Publication** : Nous publierons le correctif et un avis de sécurité
6. **Remerciements** : Nous vous remercierons publiquement (si vous le souhaitez)

### ⏱️ Délais de Réponse

-   **Accusé de réception** : 48 heures
-   **Évaluation initiale** : 7 jours
-   **Correctif pour vulnérabilités critiques** : 30 jours
-   **Correctif pour vulnérabilités non-critiques** : 90 jours

## 🛡️ Versions Supportées

| Version | Supportée |
| ------- | --------- |
| 0.1.x   | ✅ Oui    |
| < 0.1   | ❌ Non    |

## 🔍 Périmètre de Sécurité

### ✅ Dans le Périmètre

-   Injection de commandes
-   Injection SQL (si applicable dans le futur)
-   Vulnérabilités de désérialisation
-   Déni de service (DoS)
-   Vulnérabilités d'authentification/autorisation
-   Exposition d'informations sensibles
-   Vulnérabilités de validation d'entrée

### ❌ Hors Périmètre

-   Attaques de déni de service distribué (DDoS)
-   Ingénierie sociale
-   Attaques physiques
-   Vulnérabilités dans les dépendances tierces (signalez-les directement aux mainteneurs)

## 🏆 Programme de Reconnaissance

Bien que nous n'ayons pas de programme de bug bounty monétaire, nous reconnaissons les contributions à la sécurité de la manière suivante :

-   **Remerciements publics** dans les notes de version
-   **Ajout au hall of fame** dans ce fichier (si souhaité)
-   **Badge de contributeur sécurité** sur votre profil GitHub

## 📋 Bonnes Pratiques de Sécurité

### Pour les Développeurs

-   Toujours valider et assainir les entrées utilisateur
-   Utiliser des bibliothèques cryptographiques établies
-   Éviter les fonctions `unsafe` sans justification
-   Implémenter une journalisation de sécurité appropriée
-   Effectuer des révisions de code axées sur la sécurité

### Pour les Utilisateurs

-   Maintenir l'API à jour avec les dernières versions
-   Configurer des timeouts appropriés
-   Implémenter une limitation de débit
-   Surveiller les logs pour les activités suspectes
-   Utiliser HTTPS en production

## 🎖️ Hall of Fame

_Aucun contributeur sécurité pour le moment._

<!--
Format pour les futurs ajouts :
- **[Nom]** - [Date] - [Description brève de la vulnérabilité]
-->

## 📞 Contact

Pour toute question concernant cette politique de sécurité :

-   **E-mail** : [pierre.bertrnd@example.com]
-   **GitHub** : [@Pierre-brtrd](https://github.com/Pierre-brtrd)

---

**Dernière mise à jour** : 21 juillet 2025
