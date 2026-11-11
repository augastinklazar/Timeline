use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct AudioMixer {
    pub sample_rate: u32,
    pub channels: u16,
    // Real implementation would have a crossbeam bounded channel to push/pull float samples
    #[allow(dead_code)]
    buffer_underruns: Arc<Mutex<u64>>,
}

impl AudioMixer {
    pub fn new() -> Result<Self, String> {
        let host = cpal::default_host();

        let device = match host.default_output_device() {
            Some(d) => d,
            None => {
                println!("No audio output device available. Running in headless audio mode.");
                return Ok(Self {
                    sample_rate: 48000,
                    channels: 2,
                    buffer_underruns: Arc::new(Mutex::new(0))
                });
            }
        };

        let mut supported_configs_range = match device.supported_output_configs() {
            Ok(configs) => configs,
            Err(e) => {
                println!("Audio hardware detected but configuration failed ({}). Running in headless audio mode.", e);
                return Ok(Self {
                    sample_rate: 48000,
                    channels: 2,
                    buffer_underruns: Arc::new(Mutex::new(0))
                });
            }
        };

        let supported_config = match supported_configs_range.next() {
             Some(cfg) => cfg.with_max_sample_rate(),
             None => {
                 println!("No supported audio configuration found. Running in headless audio mode.");
                 return Ok(Self {
                    sample_rate: 48000,
                    channels: 2,
                    buffer_underruns: Arc::new(Mutex::new(0))
                 });
             }
        };

        let config = supported_config.into();

        // Setup stream thread
        let underruns = Arc::new(Mutex::new(0));
        let underruns_clone = underruns.clone();

        thread::spawn(move || {
            let stream_result = device.build_output_stream(
                &config,
                move |_data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    // Audio mixer processing graph goes here
                    // Read from Timeline RationalTime sync channel, resample, and write to _data
                },
                move |err| {
                    println!("Audio playback stream error: {}", err);
                    let mut lock = underruns_clone.lock().unwrap();
                    *lock += 1;
                },
                None
            );

            if let Ok(stream) = stream_result {
                if let Err(e) = stream.play() {
                    println!("Error starting audio stream: {}", e);
                }
                loop {
                    std::thread::park(); // Keep stream alive
                }
            } else {
                println!("Failed to build audio stream. Running in headless audio mode.");
            }
        });

        Ok(Self {
            sample_rate: 48000, // Hardcoded for POC
            channels: 2,
            buffer_underruns: underruns,
        })
    }
}
