use nih_plug::prelude::*;

use hrtf_spatial_audio::HrtfSpatialAudio;

fn main() {
    nih_export_standalone::<HrtfSpatialAudio>();
}
