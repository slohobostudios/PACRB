use sfml::graphics::{IntRect, RcSprite, RcTexture};
use std::fmt;
use tracing::error;

pub struct Asset {
    meta: Meta,
    frames: Vec<Frame>,
    pub(super) texture: RcTexture,
}

impl Asset {
    pub fn meta(&self) -> &Meta {
        &self.meta
    }

    pub fn texture(&self) -> &RcTexture {
        &self.texture
    }

    /// Fetches frame information from a specific frame id
    pub fn fetch_frame(&self, frame_num: usize) -> Frame {
        self.frames
            .get(frame_num)
            .cloned()
            .unwrap_or_else(|| {
                error!("No frame {} for asset {}", frame_num, self.meta.image);
                Default::default()
            })
            .clone()
    }

    /// Fetches frames related to specified FrameTag
    pub fn fetch_frames_in_frame_tag(
        &self,
        frame_tag: &FrameTag,
    ) -> Result<impl Iterator<Item = &Frame>, SimpleError> {
        let (min, max) = if frame_tag.to > frame_tag.from {
            (frame_tag.from.into(), frame_tag.to.into())
        } else {
            (frame_tag.to.into(), frame_tag.from.into())
        };
        if max > self.frames.len() {
            Err(SimpleError::new(
                "FrameTag animations exceeds numbers of animations available!".to_string(),
            ))
        } else {
            Ok(self.frames[min..=max].iter())
        }
    }

    /// Returns the duration of the animation in milliseconds
    pub fn total_animation_time_in_frame_tag(&self, frame_tag: &FrameTag) -> u32 {
        if let Ok(frames) = self.fetch_frames_in_frame_tag(&frame_tag) {
            frames.map(|frame| u32::from(frame.duration)).sum()
        } else if let Err(err) = self.fetch_frames_in_frame_tag(&frame_tag) {
            error!("{:#?}", err);
            0
        } else {
            0
        }
    }

    /// Slice bounds are normally not shifted to the appropriate frame. This returns the slice
    /// bounds shifted in accordance to the provided frame_num
    pub fn get_shifted_slice_bound(&self, slice_name: &str, frame_num: usize) -> IntRect {
        let frame = self.fetch_frame(frame_num);
        let keys = self.meta.fetch_slice_with_name(slice_name).keys;
        let unshifted_slice_bounds = keys
            .iter()
            .copied()
            .find(|slice_key| slice_key.frame == usize::from(frame_num))
            .unwrap_or_else(|| {
                keys.get(0).copied().unwrap_or_else(|| {
                    error!(
                        "No keys for slice {} for asset {}",
                        slice_name, self.meta.image
                    );
                    Default::default()
                })
            })
            .bounds;

        IntRect::from_vecs(
            unshifted_slice_bounds.position() + frame.frame.position().into_other(),
            unshifted_slice_bounds.size(),
        )
    }

    /// Scales up the shifted slice bound. See [`Self::get_shifted_slice_bound()`] for more
    /// information
    pub fn get_scaled_and_shifted_slice_bound(
        &self,
        slice_name: &str,
        frame_num: usize,
        scale: f32,
    ) -> IntRect {
        let mut bounds = self
            .get_shifted_slice_bound(slice_name, frame_num)
            .as_other::<f32>();
        bounds.top *= scale;
        bounds.left *= scale;
        bounds.width *= scale;
        bounds.height *= scale;

        bounds.as_other()
    }

    /// Returns an RcSprite given a slice name, frame number, and scaling
    pub fn get_rc_sprite_with_slice_name_and_frame_num(
        &self,
        slice_name: &str,
        frame_num: usize,
    ) -> RcSprite {
        RcSprite::with_texture_and_rect(
            &self.texture,
            self.get_shifted_slice_bound(slice_name, frame_num),
        )
    }

    /// Returns an RcSprite of the entire frame.
    pub fn get_rc_sprite_with_frame_num(&self, frame_num: usize) -> RcSprite {
        RcSprite::with_texture_and_rect(
            &self.texture,
            self.fetch_frame(frame_num).frame.into_other(),
        )
    }
}

impl fmt::Debug for Asset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Asset")
            .field("meta", &self.meta)
            .field("frames", &self.frames)
            .finish()
    }
}
