#[macro_use]
extern crate vst;

use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use vst::plugin::*;
use vst::buffer::AudioBuffer;
use vst::plugin::PluginParameters;
use vst::util::AtomicFloat;
use fdn_reverb::*;

struct FDNReverbParameters {
    drywet: AtomicFloat,
    absorbtion: AtomicFloat,
    decay: AtomicFloat,
    size: AtomicFloat,
    invalidated: AtomicBool,
}

impl Default for FDNReverbParameters {
    fn default() -> FDNReverbParameters {
        FDNReverbParameters {
            absorbtion: AtomicFloat::new(0.5),
            decay: AtomicFloat::new(0.5),
            size: AtomicFloat::new(0.3),
            drywet: AtomicFloat::new(0.4),
            invalidated: AtomicBool::new(true)
        }
    }
}

impl PluginParameters for FDNReverbParameters{
    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.drywet.get(),
            1 => self.absorbtion.get(),
            2 => self.decay.get(),
            3 => self.size.get(),
            _ => 0.0,
        }
    }
    fn set_parameter(&self, index: i32, value: f32) {
        self.invalidated.store(true, std::sync::atomic::Ordering::Release);
        match index {
            0 => self.drywet.set(value),
            1 => self.absorbtion.set(value),
            2 => self.decay.set(value),
            3 => self.size.set(value),
            _ => (),
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "dry/wet".to_string(),
            1 => "absorbtion".to_string(),
            2 => "decay".to_string(),
            3 => "size".to_string(),
            _ => "".to_string(),
        }
    }
    fn get_parameter_label(&self, index: i32) -> String {
        match index {
            0 => "%".to_string(),
            1 => "".to_string(),
            2 => "s".to_string(),
            3 => "m".to_string(),
            _ => "".to_string(),
        }
    }
    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            0 => format!("{}", self.drywet.get() * 100.),
            1 => format!("{}", self.absorbtion.get()),
            2 => format!("{}", self.decay.get()),
            3 => format!("{}", self.size.get()),
            _ => format!("!?"),
        }
    }
}

#[derive(Default)]
struct FDNReverbPlugin {
    sample_rate: f32,
    verb: FDNReverb,
    host_cb: HostCallback,
    params: Arc<FDNReverbParameters>,
}


impl Plugin for FDNReverbPlugin {
    fn new(host: HostCallback) -> FDNReverbPlugin {
        FDNReverbPlugin {
            sample_rate: 44100.,
            verb: FDNReverb::new(44100.),
            host_cb: host,
            params: Arc::new(FDNReverbParameters::default()),
        }
    }
    fn get_info(&self) -> Info {
        Info {
            name: "FDN reverb".to_string(),
            unique_id: 1360,
            inputs: 1,
            outputs: 1,
            // Set our category
            parameters: 4,
            category: Category::Effect,
            ..Default::default()
        }
    }
    fn init(&mut self) {
        self.verb = FDNReverb::new(self.sample_rate);
    }
    fn set_sample_rate(&mut self, rate: f32) {
        self.sample_rate = rate;
    }
    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let (input_buffer, mut output_buffer) = buffer.split();

        if self.params.invalidated.load(std::sync::atomic::Ordering::Acquire) {
            let abs = self.params.absorbtion.get();
            let drywet = self.params.drywet.get();
            let size = self.params.size.get();
            let decay = self.params.decay.get();

            self.verb.set_absorbtion(abs);
            self.verb.set_drywet(drywet);
            self.verb.set_size(size * 100.);
            self.verb.set_decay(decay);

            self.params.invalidated.store(false, std::sync::atomic::Ordering::Release);
        }

        for (input_channel, output_channel) in input_buffer.into_iter().zip(output_buffer.into_iter()) {
            self.verb.process(input_channel, output_channel);
        }
    }

    fn get_tail_size(&self) -> isize {
        self.verb.tail_size()
    }
    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
        Arc::clone(&self.params) as Arc<dyn PluginParameters>
    }
}

plugin_main!(FDNReverbPlugin);
