#[macro_use]
extern crate vst;

use std::sync::Arc;

use vst::plugin::*;
use vst::buffer::AudioBuffer;
use vst::plugin::PluginParameters;
use vst::util::AtomicFloat;
use fdn_reverb::*;

struct FDNReverbParameters {
    drywet: AtomicFloat,
    absorbtion: AtomicFloat,
    decay: AtomicFloat,
    size: AtomicFloat
}

impl Default for FDNReverbParameters {
    fn default() -> FDNReverbParameters {
        FDNReverbParameters {
            absorbtion: AtomicFloat::new(0.5),
            decay: AtomicFloat::new(0.5),
            size: AtomicFloat::new(0.3),
            drywet: AtomicFloat::new(0.4)
        }
    }
}

impl FDNReverbParameters {
    fn get_absorbtion(&self) -> f32 {
        self.absorbtion.get()
    }
    fn get_decay(&self) -> f32 {
        self.decay.get()
    }
    fn get_size(&self) -> f32 {
        self.size.get()
    }
    fn set_absorbtion(&self, absorbtion: f32) {
        self.absorbtion.set(absorbtion);
    }
    fn set_decay(&self, decay: f32) {
        self.decay.set(decay);
    }
    fn set_size(&self, size_m: f32) {
        self.decay.set(size_m);
    }
}

impl PluginParameters for FDNReverbParameters{
    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.drywet.get(),
            1 => self.get_absorbtion(),
            2 => self.get_decay(),
            3 => self.get_size(),
            _ => 0.0,
        }
    }
    fn set_parameter(&self, index: i32, value: f32) {
        match index {
            0 => self.drywet.set(value),
            1 => self.set_absorbtion(value),
            2 => self.set_decay(value),
            3 => self.set_size(value),
            _ => (),
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "drywet".to_string(),
            1 => "absorbtion".to_string(),
            2 => "decay".to_string(),
            3 => "size".to_string(),
            _ => "".to_string(),
        }
    }
    fn get_parameter_label(&self, index: i32) -> String {
        match index {
            0 => "".to_string(),
            1 => "".to_string(),
            2 => "s".to_string(),
            3 => "m".to_string(),
            _ => "".to_string(),
        }
    }
    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            0 => format!("{:.2}", self.drywet.get()),
            1 => format!("{:.0}", self.get_absorbtion()),
            2 => format!("{:.2}", self.get_decay()),
            3 => format!("{:.3}", self.get_size()),
            _ => format!(""),
        }
    }
}

#[derive(Default)]
struct FDNReverbPlugin {
    sample_rate: f32,
    verb: FDNReverb,
    host_cb: HostCallback,
    params: Arc<FDNReverbParameters>
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

        let abs = self.params.get_absorbtion();
        let drywet = self.params.drywet.get();
        let size = self.params.get_size() * 10.;
        let decay = self.params.get_decay();

        self.verb.set_absorbtion(abs);
        self.verb.set_drywet(drywet);
        self.verb.set_size(size);
        self.verb.set_decay(decay);

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
