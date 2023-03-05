use sfml::graphics::RcText;
use std::{collections::LinkedList, time::Instant};

#[derive(Clone, Debug)]
pub struct FPSCounter {
    instances: LinkedList<Instant>,
    pub avg_over_num_frames: usize,
    text: RcText,
}

impl FPSCounter {
    pub fn new(resource_manager: &ResourceManager, avg_over_num_frames: usize) -> Self {
        Self {
            text: RcText::new("NaN", resource_manager.fetch_current_font(), 32),
            instances: LinkedList::from([Instant::now()]),
            avg_over_num_frames,
        }
    }

    /// Let FPSCounter know that a new frame has just been rendered
    pub fn new_frame(&mut self) {
        self.instances.push_front(Instant::now());
        if self.instances.len() > self.avg_over_num_frames {
            self.instances.pop_back();
        }
    }

    /// Return the average FPS
    pub fn fps(&self) -> usize {
        let mut elapsed = usize::try_from(
            self.instances
                .back()
                .unwrap_or(&Instant::now())
                .elapsed()
                .as_millis(),
        )
        .unwrap_or(u16::MAX.into());
        if elapsed == 0 {
            elapsed = 1;
        }

        1000 * self.instances.len() / elapsed
    }

    /// Get the average FPS as an SFML Text object
    pub fn fps_text<'a>(&mut self) -> &RcText {
        self.text.set_string(&self.fps().to_string());
        &self.text
    }
}
