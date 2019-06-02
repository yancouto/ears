use openal::{ffi, al};
use internal::OpenAlData;
use presets::ReverbProperties;

/**
 * Create and configure effects.
 *
 * A Sound can optionally be connected to a ReverbEffect, which can modify
 * how the user hears the Sound (through reverb, echo, frequency shift, etc)
 *
 * This can be used to model the environment that the listener is in,
 * for example a cave.
 *
 * Internally it creates an OpenAL Effect Object with an Auxiliary Effect
 * Slot Object pair.
 *
 * # Examples
 * ```no_run
 * extern crate ears;
 * use ears::{Effect, EffectPreset, Sound, AudioController};
 *
 * fn main() -> () {
 *    // Create an effect (in this case, using a preset)
 *    let effect = Effect::preset(EffectPreset::Cave);
 *
 *    // Create a Sound with the path of the sound file.
 *    let mut sound = Sound::new("path/to/my/sound.ogg").unwrap();
 *
 *    // Connect the sound to the effect
 *    sound.connect(effect);
 *
 *    // Play it
 *    sound.play();
 *
 *    // Wait until the sound stopped playing
 *    while sound.is_playing() {}
 *
 *    // If you want to disconnect an Effect, just pass None
 *    sound.connect(None);
 * }
 * ```
 */
pub struct ReverbEffect {
    effect_id: u32,
    effect_slot_id: u32,
}

impl ReverbEffect {
    pub fn new() -> Result<ReverbEffect, String> {
        check_openal_context!(Err("Invalid OpenAL context.".into()));

        // TODO: check effect extension availability before bothering
        // to do all this

        // Create the auxiliary effect slot
        let mut effect_slot_id = 0;
        al::alGenAuxiliaryEffectSlots(1, &mut effect_slot_id);

        // Create the effect
        let mut effect_id = 0;
        al::alGenEffects(1, &mut effect_id);

        // Assume only "standard reverb" for now. May add EAX reverb at some point.
        al::alEffecti(effect_id, ffi::AL_EFFECT_TYPE, ffi::AL_EFFECT_REVERB);

        // Check if there is OpenAL internal error
        if let Some(err) = al::openal_has_error() {
            return Err(format!("ReverbEffect::new - OpenAL error: {}", err));
        };

        Ok(ReverbEffect { effect_id, effect_slot_id })
    }

    pub fn preset(reverb_properties: ReverbProperties) -> Result<ReverbEffect, String> {
        match Self::new() {
            Ok(mut effect) => {
                effect.set_density(reverb_properties.density);
                effect.set_diffusion(reverb_properties.diffusion);
                effect.set_gain(reverb_properties.gain);
                effect.set_gainhf(reverb_properties.gainhf);
                effect.set_decay_time(reverb_properties.decay_time);
                effect.set_decay_hfratio(reverb_properties.decay_hfratio);
                effect.set_reflections_gain(reverb_properties.reflections_gain);
                effect.set_reflections_delay(reverb_properties.reflections_delay);
                effect.set_late_reverb_gain(reverb_properties.late_reverb_gain);
                effect.set_late_reverb_delay(reverb_properties.late_reverb_delay);
                effect.set_air_absorption_gainhf(reverb_properties.air_absorption_gainhf);
                effect.set_room_rolloff_factor(reverb_properties.room_rolloff_factor);
                effect.set_decay_hflimit(reverb_properties.decay_hflimit);

                // Check if there is OpenAL internal error
                if let Some(err) = al::openal_has_error() {
                    return Err(format!("ReverbEffect::preset - OpenAL error: {}", err));
                };

                effect.update_slot();

                Ok(effect)
            },
            Err(e) => Err(e)
        }
    }

    pub fn slot(&self) -> u32 {
      self.effect_slot_id
    }

    fn update_slot(&mut self) {
        check_openal_context!(());
        al::alAuxiliaryEffectSloti(self.effect_slot_id, ffi::AL_EFFECTSLOT_EFFECT, self.effect_id);
    }

    fn set_density(&mut self, density: f32) {
        check_openal_context!(());
        al::alEffectf(self.effect_id, ffi::AL_REVERB_DENSITY, density);
    }

    fn set_diffusion(&mut self, diffusion: f32) {
        check_openal_context!(());
        al::alEffectf(self.effect_id, ffi::AL_REVERB_DIFFUSION, diffusion);
    }

    fn set_gain(&mut self, gain: f32) {
        check_openal_context!(());
        al::alEffectf(self.effect_id, ffi::AL_REVERB_GAIN, gain);
    }

    fn set_gainhf(&mut self, gainhf: f32) {
        check_openal_context!(());
        al::alEffectf(self.effect_id, ffi::AL_REVERB_GAINHF, gainhf);
    }

    fn set_decay_time(&mut self, decay_time: f32) {
        check_openal_context!(());
        al::alEffectf(self.effect_id, ffi::AL_REVERB_DECAY_TIME, decay_time);
    }

    fn set_decay_hfratio(&mut self, decay_hfratio: f32) {
        check_openal_context!(());
        al::alEffectf(self.effect_id, ffi::AL_REVERB_DECAY_HFRATIO, decay_hfratio);
    }

    fn set_reflections_gain(&mut self, reflections_gain: f32) {
        check_openal_context!(());
        al::alEffectf(self.effect_id, ffi::AL_REVERB_REFLECTIONS_GAIN, reflections_gain);
    }

    fn set_reflections_delay(&mut self, reflections_delay: f32) {
        check_openal_context!(());
        al::alEffectf(self.effect_id, ffi::AL_REVERB_REFLECTIONS_DELAY, reflections_delay);
    }

    fn set_late_reverb_gain(&mut self, late_reverb_gain: f32) {
        check_openal_context!(());
        al::alEffectf(self.effect_id, ffi::AL_REVERB_LATE_REVERB_GAIN, late_reverb_gain);
    }

    fn set_late_reverb_delay(&mut self, late_reverb_delay: f32) {
        check_openal_context!(());
        al::alEffectf(self.effect_id, ffi::AL_REVERB_LATE_REVERB_DELAY, late_reverb_delay);
    }

    fn set_air_absorption_gainhf(&mut self, air_absorption_gainhf: f32) {
        check_openal_context!(());
        al::alEffectf(self.effect_id, ffi::AL_REVERB_AIR_ABSORPTION_GAINHF, air_absorption_gainhf);
    }

    fn set_room_rolloff_factor(&mut self, room_rolloff_factor: f32) {
        check_openal_context!(());
        al::alEffectf(self.effect_id, ffi::AL_REVERB_ROOM_ROLLOFF_FACTOR, room_rolloff_factor);
    }

    fn set_decay_hflimit(&mut self, decay_hflimit: i32) {
        check_openal_context!(());
        al::alEffecti(self.effect_id, ffi::AL_REVERB_DECAY_HFLIMIT, decay_hflimit);
    }
}

impl Drop for ReverbEffect {
    // Delete the Effect Object and Auxiliary Effect Slot Object
    fn drop(&mut self) -> () {
        check_openal_context!(());

        // Disconnect the effect and slot
        al::alAuxiliaryEffectSloti(self.effect_slot_id, ffi::AL_EFFECTSLOT_EFFECT, ffi::AL_EFFECT_NULL as u32);

        unsafe {
            ffi::alDeleteEffects(1, &mut self.effect_id);
            ffi::alDeleteAuxiliaryEffectSlots(1, &mut self.effect_slot_id);
        }

        // Check if there is OpenAL internal error
        //
        // TODO: this could probably be avoided with some better design
        if al::openal_has_error().is_some() {
            eprintln!("Ears failed to drop ReverbEffect completely, one or more source is probably still referencing it.");
            eprintln!("\tEffect Object: {}", self.effect_id);
            eprintln!("\tAuxiliary Effect Slot: {}", self.effect_slot_id);
        };
    }
}
