# Changelog  embassy-ttp223b

Toutes les évolutions notables de ce projet sont documentées ici.  
Ce projet suit une philosophie de développement pragmatique : chaque mise à jour vise à enrichir les fonctionnalités tout en minimisant l'empreinte mémoire sur le microcontrôleur.

---

## [0.1.0] — 2026

### Ajouté
- Implémentation initiale du driver asynchrone pour le capteur tactile TTP223B
- Debounce logiciel 50 ms configurable pour éliminer les faux positifs
- `Ttp223::new(pin)`:constructeur avec timing par défaut
- `Ttp223::new_with_timing(pin, debounce_ms, poll_ms)` — timing personnalisable
- `wait_for_change()`: attente asynchrone d'un changement d'état confirmé
- `get_state()` : lecture de l'état mémorisé sans attente
- `is_pressed()` : raccourci booléen
- `release()` :libération de la ressource GPIO
- Types publics `TouchState` (Pressed / Released) et `TouchError` (ReadError / SignalUnstable)
- Zéro allocation, zéro `unsafe`, compatible `no_std`