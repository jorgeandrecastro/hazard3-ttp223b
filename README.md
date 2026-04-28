[![crates.io](https://img.shields.io/crates/v/embassy-ttp223b.svg)](https://crates.io/crates/embassy-ttp223b)
[![docs.rs](https://docs.rs/embassy-ttp223b/badge.svg)](https://docs.rs/embassy-ttp223b)
[![License: GPL v3](https://img.shields.io/badge/License-GPL_v3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0.en.html)

# embassy-ttp223b

Driver asynchrone `no_std` pour le capteur tactile **TTP223B** via GPIO, testé sur la Pico 2 et Pico 2040.

Élimine les faux positifs par un filtrage temporel logiciel (**debounce 50 ms**) sans jamais bloquer le CPU, garantissant une efficacité énergétique maximale. Optimisé pour l'exécuteur `embassy`.

---

## Câblage exemple Testé 

### Waveshare Pico 2350B

| TTP223B | Pico 2350B | Description |
|---------|-----------|-------------|
| GND     | GND       | Ground (masse commune) |
| VCC     | 3V3       | Alimentation 3.3V |
| OUT     | GPIO16    | Signal de sortie tactile |

**Schéma de connexion :**
```
TTP223B     Pico 2350B
  GND  -------> GND (pin 38 ou autre GND)
  VCC  -------> 3V3 (pin 36)
  OUT  -------> GPIO16 (pin 21)
```

---

## Utilisation

```toml
[dependencies]
embassy-ttp223b = "0.1.0"
```

```rust
use embassy_ttp223b::{Ttp223, TouchState};

let mut touch = Ttp223::new(pin);

loop {
    match touch.wait_for_change().await {
        Ok(TouchState::Pressed)  => { /* réaction au toucher */ }
        Ok(TouchState::Released) => { /* relâchement */ }
        Err(_) => { /* gestion d'erreur GPIO */ }
    }
}
```

---

## Exemple avec affichage SSD1306

```rust
use embassy_ttp223b::{Ttp223, TouchState};
use embassy_ssd1306::Ssd1306;

let mut oled  = Ssd1306::new(i2c, 0x3C);
let mut touch = Ttp223::new(pin);

oled.init().await.unwrap();

loop {
    match touch.wait_for_change().await {
        Ok(TouchState::Pressed) => {
            oled.clear();
            oled.draw_str(10, 3, b"TOUCHE!");
            oled.flush().await.unwrap();
        }
        Ok(TouchState::Released) => {
            oled.clear();
            oled.draw_str(10, 3, b"RELACHE");
            oled.flush().await.unwrap();
        }
        Err(_) => {}
    }
}
```

---

## Exemple Pico 2040

```rust
let mut touch = Ttp223::new(p.PIN_15.into());

loop {
    if let Ok(state) = touch.wait_for_change().await {
        match state {
            TouchState::Pressed  => led.set_high(),
            TouchState::Released => led.set_low(),
        }
    }
}
```

---

## Timing personnalisé

```rust
use embassy_ttp223b::Ttp223;

// debounce 80 ms, polling toutes les 5 ms
let mut touch = Ttp223::new_with_timing(pin, 80, 5);
```

---

## Fonctionnalités

- `wait_for_change` : attend un changement d'état confirmé (debounce asynchrone)
- `get_state` : retourne l'état mémorisé sans attendre
- `is_pressed`:  raccourci booléen pour l'état pressé
- `new_with_timing` durées de debounce et de polling configurables
- `release` : libère le pin GPIO et retourne la ressource matérielle
- Zéro allocation, zéro `unsafe`, zéro blocage CPU

---

# 📋 Historique et Évolutions (Changelog)

**Version : v0.1.0**

- Implémentation initiale du driver TTP223B asynchrone
- Debounce logiciel 50 ms par défaut
- API `wait_for_change`, `get_state`, `is_pressed`, `release`
- Timing configurable via `new_with_timing`

Pour consulter le détail de toutes les versions, veuillez vous référer au fichier :
👉 [CHANGELOG.md](CHANGELOG.md)

---

## Licence

GPL-3.0-or-later — Copyright (C) 2026 Jorge Andre Castro