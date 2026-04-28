// Copyright (C) 2026 Jorge Andre Castro
// GPL-3.0-or-later

#![no_std]
#![forbid(unsafe_code)]

//! # embassy-ttp223b
//!
//! Driver asynchrone `no_std` pour le capteur tactile TTP223B via GPIO.
//! Élimine les faux positifs par un filtrage temporel logiciel (debounce)
//! sans jamais bloquer le CPU, garantissant une efficacité énergétique maximale.
//!
//! Fonctionne avec n'importe quel pin implémentant `embedded-hal::digital::InputPin`
//! et l'exécuteur asynchrone Embassy.
//!
//! # Exemple
//!
//! ```rust,no_run
//! use embassy_ttp223b::{Ttp223, TouchState};
//!
//! let mut touch = Ttp223::new(pin);
//!
//! loop {
//!     match touch.wait_for_change().await {
//!         Ok(TouchState::Pressed)  => { /* réaction au toucher */ }
//!         Ok(TouchState::Released) => { /* relâchement */ }
//!         Err(_) => { /* gestion d'erreur */ }
//!     }
//! }
//! ```

use embassy_time::{Duration, Timer};
use embedded_hal::digital::InputPin;

/// Durée de debounce par défaut (ms).
pub const DEBOUNCE_MS: u64 = 50;

/// Durée de polling par défaut quand aucun changement n'est détecté (ms).
pub const POLL_MS: u64 = 10;

/// États possibles du capteur tactile.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TouchState {
    /// Le capteur est pressé (signal haut).
    Pressed,
    /// Le capteur est relâché (signal bas).
    Released,
}

/// Erreurs spécifiques au capteur TTP223B.
#[derive(Debug)]
pub enum TouchError {
    /// Problème de lecture matérielle sur le GPIO.
    ReadError,
    /// Le signal oscille trop pour être considéré fiable.
    SignalUnstable,
}

/// Instance principale du driver TTP223B.
///
/// Fonctionne avec n'importe quel pin implémentant
/// `embedded-hal::digital::InputPin`.
pub struct Ttp223<P: InputPin> {
    pin: P,
    last_confirmed_state: bool,
    debounce_ms: u64,
    poll_ms: u64,
}

impl<P: InputPin> Ttp223<P> {
    /// Initialise une nouvelle instance du driver avec un pin GPIO.
    ///
    /// Utilise les durées de debounce et de polling par défaut
    /// ([`DEBOUNCE_MS`] et [`POLL_MS`]).
    ///
    /// # Arguments
    /// * `pin` — Pin GPIO configuré en entrée.
    pub fn new(pin: P) -> Self {
        Self {
            pin,
            last_confirmed_state: false,
            debounce_ms: DEBOUNCE_MS,
            poll_ms: POLL_MS,
        }
    }

    /// Initialise une nouvelle instance avec des durées personnalisées.
    ///
    /// # Arguments
    /// * `pin`          Pin GPIO configuré en entrée.
    /// * `debounce_ms`  Durée d'attente de stabilisation (ms).
    /// * `poll_ms`     Période de polling en l'absence de changement (ms).
    pub fn new_with_timing(pin: P, debounce_ms: u64, poll_ms: u64) -> Self {
        Self {
            pin,
            last_confirmed_state: false,
            debounce_ms,
            poll_ms,
        }
    }

    /// Attend un changement d'état avec confirmation temporelle (debounce).
    ///
    /// Cette fonction est asynchrone : elle rend la main à l'exécuteur
    /// pendant les phases d'attente ([`Timer`]), sans jamais bloquer le CPU.
    ///
    /// # Retourne
    /// * `Ok(TouchState::Pressed)`  Toucher confirmé.
    /// * `Ok(TouchState::Released)` Relâchement confirmé.
    /// * `Err(TouchError::ReadError)` Erreur de lecture GPIO.
    pub async fn wait_for_change(&mut self) -> Result<TouchState, TouchError> {
        loop {
            let current_raw = self
                .pin
                .is_high()
                .map_err(|_| TouchError::ReadError)?;

            if current_raw != self.last_confirmed_state {
                // Debounce : attendre que les rebonds électriques se calment.
                Timer::after(Duration::from_millis(self.debounce_ms)).await;

                let confirmed = self
                    .pin
                    .is_high()
                    .map_err(|_| TouchError::ReadError)?;

                if confirmed == current_raw {
                    self.last_confirmed_state = confirmed;
                    return Ok(if confirmed {
                        TouchState::Pressed
                    } else {
                        TouchState::Released
                    });
                }
                // Signal instable : on ignore et on reboucle.
            }

            // Pause légère pour ne pas saturer le scheduler.
            Timer::after(Duration::from_millis(self.poll_ms)).await;
        }
    }

    /// Renvoie l'état mémorisé sans attendre de nouveau changement.
    ///
    /// Utile pour consulter l'état courant dans une tâche périodique.
    pub fn get_state(&self) -> TouchState {
        if self.last_confirmed_state {
            TouchState::Pressed
        } else {
            TouchState::Released
        }
    }

    /// Indique si le capteur est actuellement pressé (état mémorisé).
    pub fn is_pressed(&self) -> bool {
        self.last_confirmed_state
    }

    /// Libère le pin GPIO et retourne la ressource matérielle.
    pub fn release(self) -> P {
        self.pin
    }
}