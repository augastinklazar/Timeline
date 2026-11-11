use ffmpeg_next as ffmpeg;
use tokio::sync::mpsc;
use std::thread;

pub struct DecoderPool {
    worker_tx: mpsc::Sender<DecodeRequest>,
}

pub struct DecodeRequest {
    pub filepath: String,
    pub target_frame: i64,
}

impl DecoderPool {
    pub fn new(threads: usize) -> Result<Self, String> {
        ffmpeg::init().map_err(|e| format!("Failed to init FFmpeg: {}", e))?;

        let (tx, mut rx) = mpsc::channel::<DecodeRequest>(100);

        // Spawn a multi-threaded asynchronous decoding worker pool
        // Using a single receiver thread to pull from the async channel for this POC
        thread::spawn(move || {
             while let Some(req) = rx.blocking_recv() {
                  println!("Worker pulled request for: {}", req.filepath);
             }
        });

        for i in 0..threads {
            thread::spawn(move || {
                println!("Media Worker [{}] initialized.", i);
                loop {
                    std::thread::park(); // Idle until work in real implementation
                }
            });
        }

        Ok(Self { worker_tx: tx })
    }

    pub async fn request_frame(&self, req: DecodeRequest) -> Result<(), String> {
        self.worker_tx
            .send(req)
            .await
            .map_err(|e| format!("Decoder pool channel error: {}", e))
    }
}
