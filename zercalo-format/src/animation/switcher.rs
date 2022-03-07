use super::animatable::Animatable;
use crate::scene::{Camera, HasBounding, HasCamera, HasMutCamera, HasMutScene, HasScene, Scene};
use glam::Vec3;
use log::*;

/// Combinator that allows you to switch between models on time. First N frames first variant, next T
/// frames other and etc. Can be used to animate complex models.
#[derive(Debug, Clone)]
pub struct Switcher<T> {
    /// Stores frames
    pub variants: Vec<T>,
    /// Current active frame
    pub active: u32,
    /// Contains end frame for each corresponding variant
    pub schedule: Vec<u32>,
    /// Last known frame
    pub last_frame: u32,
    /// Loop animation if reached the end?
    pub looping: bool,
    /// Saved offset for looping logic
    pub loop_offset: u32,
}

impl<T> Switcher<T> {
    /// Construct new switcher with pairs of frames duration and frames themselves.
    pub fn new(frames: Vec<(u32, T)>) -> Self {
        assert!(!frames.is_empty(), "Frames for switcher must be not empty!");
        let (durations, variants): (Vec<u32>, Vec<T>) = frames.into_iter().unzip();
        let mut schedule = vec![];
        let mut acc = 0;
        for dur in durations.into_iter() {
            acc += dur;
            schedule.push(acc);
        }
        Switcher {
            variants,
            active: 0,
            schedule,
            last_frame: 0,
            looping: true,
            loop_offset: 0,
        }
    }

    /// Get current active frame
    pub fn current(&self) -> &T {
        &self.variants[self.active as usize]
    }

    /// Get current active frame as mut reference
    pub fn current_mut(&mut self) -> &mut T {
        &mut self.variants[self.active as usize]
    }
}

impl<T: HasCamera> HasCamera for Switcher<T> {
    fn get_camera(&self) -> &Camera {
        self.current().get_camera()
    }
}

impl<T: HasMutCamera> HasMutCamera for Switcher<T> {
    fn get_mut_camera(&mut self) -> &mut Camera {
        self.current_mut().get_mut_camera()
    }
}

impl<T: HasBounding> HasBounding for Switcher<T> {
    fn get_bounding_volume(&self) -> (Vec3, Vec3) {
        self.current().get_bounding_volume()
    }
}

impl<T: HasScene> HasScene for Switcher<T> {
    fn get_scene(&self) -> &Scene {
        self.current().get_scene()
    }
}

impl<T: HasMutScene> HasMutScene for Switcher<T> {
    fn get_scene_mut(&mut self) -> &mut Scene {
        self.current_mut().get_scene_mut()
    }
}

impl<T: Animatable> Animatable for Switcher<T> {
    fn animate(&mut self, frame: u32) {
        if frame < self.last_frame {
            warn!(
                "Switcher<{}> animate frame backward, last frame was {}, but got new {}",
                std::any::type_name::<T>(),
                self.last_frame,
                frame
            );
            return;
        };
        assert!(
            (self.active as usize) < self.variants.len(),
            "Active frame index {} is greater than frames array {}",
            self.active,
            self.variants.len()
        );
        assert!(
            (self.active as usize) < self.schedule.len(),
            "Active frame index {} is greater than schedule array {}",
            self.active,
            self.schedule.len()
        );

        self.current_mut().animate(frame);
        let dframe = frame - self.last_frame;
        let new_frame = self.last_frame + dframe;
        let border = self.schedule[self.active as usize] + self.loop_offset;
        if new_frame >= border {
            if self.active as usize >= self.variants.len() - 1 {
                if self.looping {
                    self.active = 0;
                    self.loop_offset = new_frame;
                } else {
                    self.active = (self.variants.len() - 1) as u32;
                }
            } else {
                self.active += 1;
            }
        }
        self.last_frame = new_frame;
    }
}
