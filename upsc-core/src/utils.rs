use bitflags::{bitflags, bitflags_match};
use glam::UVec2;

bitflags! {
    /// Used internally to identify enabled backends. Although exposed, Underlying values may change between releases.
    /// So they should not be relied on.
    /// 
    /// And in real-world scenarios, only a few, if any, of these will actually be available (compatible, enabled, etc).
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Backends: u8 {
        const DLSS = 1 << 0;
        const FSR1 = 1 << 1;
        const FSR2 = 1 << 2;
        const FSR3 = 1 << 3;
        const XeSS = 1 << 4;
        const COMP = 1 << 5; // Compute shader fallback
    }
}

/// 
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
pub enum Presets {
    /// Allow the quality mode to decide
    #[default]
    Auto,
    /// Anti-aliased, with no upscaling.
    Native,
    /// Minimal upscaling to maintain high visual fidelity.
    Quality,
    /// A nice trade-off of quality/artifacts for higher performance.
    Balanced,
    /// High upscaling, most applications should stop here.
    Performance,
    /// Highest upscaling, only advised for super high resolution targets (e.g. 8k).
    Fastest
}

#[cfg(dlss)]
use upsc_ngx::*;

/// A generic alias for internal (e.g. sdk-specific) performance mode values
pub enum Mode {
    DLSS(NVSDK_NGX_PerfQuality_Value),
}

impl Presets {
    pub fn as_val(&self, target_resolution: UVec2, backends: Backends) -> Vec<Mode> {
        let mut modes = Vec::<Mode>::new();

        match self {
            Self::Auto => {
                // To simplify things, let's resuse the logic for existing presets.
                let mut recurse = |preset: Presets| {
                    modes.extend(preset.as_val(target_resolution, backends));
                };

                // TODO: This should probably have some unique logic for different backends.
                // The rest of this case is loosely based on Nvidia's recommendations for DLSS.
                let mega_pixels = (target_resolution.x * target_resolution.y) as f32 / 1_000_000.0;

                if mega_pixels < 2.03 {
                    recurse(Presets::Native);
                }
                else if mega_pixels < 3.68 {
                    recurse(Presets::Performance);
                }
                else if mega_pixels < 8.29 {
                    recurse(Presets::Performance);
                }
                else {
                    recurse(Presets::Fastest);
                }
            }
            Self::Native => {
                bitflags_match!(backends, {
                    Backends::DLSS => modes.push(Mode::DLSS(NVSDK_NGX_PerfQuality_Value_NVSDK_NGX_PerfQuality_Value_DLAA)),
                    _ => (),
                })
            }
            Self::Quality => {
                bitflags_match!(backends, {
                    Backends::DLSS => modes.push(Mode::DLSS(NVSDK_NGX_PerfQuality_Value_NVSDK_NGX_PerfQuality_Value_MaxQuality)),
                    _ => (),
                })
            }
            Self::Balanced => {
                bitflags_match!(backends, {
                    Backends::DLSS => modes.push(Mode::DLSS(NVSDK_NGX_PerfQuality_Value_NVSDK_NGX_PerfQuality_Value_Balanced)),
                    _ => (),
                })
            }
            Self::Performance => {
                bitflags_match!(backends, {
                    Backends::DLSS => modes.push(Mode::DLSS(NVSDK_NGX_PerfQuality_Value_NVSDK_NGX_PerfQuality_Value_MaxPerf)),
                    _ => (),
                })
            }
            Self::Fastest => {
                bitflags_match!(backends, {
                    Backends::DLSS => modes.push(Mode::DLSS(NVSDK_NGX_PerfQuality_Value_NVSDK_NGX_PerfQuality_Value_UltraPerformance)),
                    _ => (),
                })
            }
        };

        modes
    }
}