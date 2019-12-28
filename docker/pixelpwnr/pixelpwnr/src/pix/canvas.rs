use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::sleep;
use std::thread;
use std::time::Duration;

use image::DynamicImage;

use painter::handle::Handle;
use painter::painter::Painter;
use pix::client::Client;
use rect::Rect;

/// A pixflut instance
pub struct Canvas {
    host: String,
    painter_count: usize,
    painter_handles: Vec<Handle>,
    size: (u32, u32),
    offset: (u32, u32),
}

impl Canvas {
    /// Create a new pixelflut canvas.
    pub fn new(host: &str, painter_count: usize, size: (u32, u32), offset: (u32, u32)) -> Canvas {
        // Initialize the object
        let mut canvas = Canvas {
            host: host.to_string(),
            painter_count,
            painter_handles: Vec::with_capacity(painter_count),
            size,
            offset,
        };

        // Show a status message
        println!("Starting painter threads...");

        // Spawn some painters
        canvas.spawn_painters();

        // Return the canvas
        canvas
    }

    /// Spawn the painters for this canvas
    fn spawn_painters(&mut self) {
        // Spawn some painters
        for i in 0..self.painter_count {
            // Determine the slice width
            let width = self.size.0 / (self.painter_count as u32);

            // Define the area to paint per thread
            let painter_area = Rect::from((i as u32) * width, 0, width, self.size.1);

            // Spawn the painter
            self.spawn_painter(painter_area);
        }
    }

    /// Spawn a single painter in a thread.
    fn spawn_painter(&mut self, area: Rect) {
        // Get the host that will be used
        let host = self.host.to_string();

        // Redefine the offset to make it usable in the thread
        let offset = (self.offset.0, self.offset.1);

        // Create a channel to push new images
        let (tx, rx): (Sender<DynamicImage>, Receiver<DynamicImage>) = mpsc::channel();

        // Create the painter thread
        let thread = thread::spawn(move || {
            // Create the painter
            let mut painter = Painter::new(None, area, offset, None);

            loop {
                // The painting loop
                'paint: loop {
                    // Connect
                    let client = match Client::connect(host.clone()) {
                        Ok(client) => client,
                        Err(e) => {
                            eprintln!("Painter failed to connect: {}", e);
                            break 'paint;
                        },
                    };
                    painter.set_client(Some(client));

                    // Keep painting
                    loop {
                        if let Err(e) = painter.work(&rx) {
                            println!("Painter error: {}", e);
                            break 'paint;
                        }
                    }
                }

                // Sleep for half a second before restarting the painter
                sleep(Duration::from_millis(500));
                println!("Restarting failed painter...");
            }
        });

        // Create a new painter handle, pust it to the list
        self.painter_handles.push(Handle::new(thread, area, tx));
    }

    // Update the image that is being rendered for all painters.
    pub fn update_image(&mut self, image: &mut DynamicImage) {
        // Update the image for each specific painter handle
        for handle in &self.painter_handles {
            handle.update_image(image);
        }
    }
}
