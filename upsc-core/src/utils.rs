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
        /// Does nothing, useful for testing.
        const NOOP = 1 << 0;
        /// https://www.nvidia.com/en-us/geforce/technologies/dlss/
        const NGX = 1 << 1;
        /// Currently: https://github.com/GPUOpen-LibrariesAndSDKs/FidelityFX-SDK/tree/c6efa6bf7f2027b3ec94f28578bb5965eabb9e55
        /// 
        /// Eventually: https://gpuopen.com/fidelityfx-super-resolution-4/
        const FSR = 1 << 2;
        /// https://www.intel.com/content/www/us/en/developer/topic-technology/gamedev/xess2.html
        const XeSS = 1 << 5;
        /// https://developer.apple.com/documentation/metalfx
        const MTLFX = 1 << 6;
        /// This is a custom fallback, intended for systems otherwise without an available backend.
        const COMP = 1 << 7;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
pub enum Presets {
    /// Allow the quality mode to decide
    #[default]
    Auto,
    /// Anti-aliased, with no upscaling.
    Native,
    /// Commonly "Ultra Quality", We make this available, but it's advised not to use.
    /// 
    /// It doesn't seem to have much support, if any, on some backends.
    Best,
    /// Minimal upscaling to maintain high visual fidelity.
    Quality,
    /// A nice trade-off of quality/artifacts for higher performance.
    Balanced,
    /// High upscaling, most applications should stop here.
    Performance,
    /// Highest upscaling, only advised for super high resolution targets (e.g. 8k).
    Fastest
}

#[cfg(ngx)]
use upsc_ngx as ngx;

#[cfg(ffx)]
use fidelityfx_sys::sdk::fsr3;

/// A generic alias for internal (e.g. sdk-specific) performance mode values
pub enum Mode {
    #[cfg(ngx)]
    NGX(ngx::NVSDK_NGX_PerfQuality_Value),
    #[cfg(ffx)]
    FSR(Option<fsr3::Fsr3QualityMode>),
}

impl Presets {
    pub fn as_val(&self, target_resolution: UVec2, backends: Backends) -> Vec<Mode> {
        let mut modes = Vec::<Mode>::new();

        match self {
            Self::Auto => {
                // To simplify things, let's resuse the logic for existing presets.
                let mut recurse = |preset: Presets| modes.extend(preset.as_val(target_resolution, backends));

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
                    #[cfg(ngx)]
                    Backends::NGX => modes.push(Mode::NGX(ngx::NVSDK_NGX_PerfQuality_Value_NVSDK_NGX_PerfQuality_Value_DLAA)),
                    #[cfg(ffx)]
                    Backends::FSR => modes.push(Mode::FSR(None)),
                    _ => (),
                })
            }
            Self::Best => {
                bitflags_match!(backends, {
                    #[cfg(ngx)]
                    Backends::NGX => modes.push(Mode::NGX(ngx::NVSDK_NGX_PerfQuality_Value_NVSDK_NGX_PerfQuality_Value_UltraQuality)),
                    #[cfg(ffx)]
                    Backends::FSR => modes.push(Mode::FSR(Some(fsr3::Fsr3QualityMode::QUALITY))),
                    _ => (),
                })
            }
            Self::Quality => {
                bitflags_match!(backends, {
                    #[cfg(ngx)]
                    Backends::NGX => modes.push(Mode::NGX(ngx::NVSDK_NGX_PerfQuality_Value_NVSDK_NGX_PerfQuality_Value_MaxQuality)),
                    #[cfg(ffx)]
                    Backends::FSR => modes.push(Mode::FSR(Some(fsr3::Fsr3QualityMode::QUALITY))),
                    _ => (),
                })
            }
            Self::Balanced => {
                bitflags_match!(backends, {
                    #[cfg(ngx)]
                    Backends::NGX => modes.push(Mode::NGX(ngx::NVSDK_NGX_PerfQuality_Value_NVSDK_NGX_PerfQuality_Value_Balanced)),
                    #[cfg(ffx)]
                    Backends::FSR => modes.push(Mode::FSR(Some(fsr3::Fsr3QualityMode::BALANCED))),
                    _ => (),
                })
            }
            Self::Performance => {
                bitflags_match!(backends, {
                    #[cfg(ngx)]
                    Backends::NGX => modes.push(Mode::NGX(ngx::NVSDK_NGX_PerfQuality_Value_NVSDK_NGX_PerfQuality_Value_MaxPerf)),
                    #[cfg(ffx)]
                    Backends::FSR => modes.push(Mode::FSR(Some(fsr3::Fsr3QualityMode::PERFORMANCE))),
                    _ => (),
                })
            }
            Self::Fastest => {
                bitflags_match!(backends, {
                    #[cfg(ngx)]
                    Backends::NGX => modes.push(Mode::NGX(ngx::NVSDK_NGX_PerfQuality_Value_NVSDK_NGX_PerfQuality_Value_UltraPerformance)),
                    #[cfg(ffx)]
                    Backends::FSR => modes.push(Mode::FSR(Some(fsr3::Fsr3QualityMode::ULTRA_PERFORMANCE))),
                    _ => (),
                })
            }
        };

        modes
    }
}