use glam::UVec2;

#[cfg(dlss)]
use upsc_ngx::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
pub enum QualityMode {
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

impl QualityMode {
    #[cfg(dlss)]
    pub(crate) fn raw(&self, target: UVec2) -> NVSDK_NGX_PerfQuality_Value {
        match self {
            Self::Auto => {
                let mega_pixels = (target.x * target.y) as f32 / 1_000_000.0;

                if mega_pixels < 2.03 {
                    NVSDK_NGX_PerfQuality_Value_NVSDK_NGX_PerfQuality_Value_DLAA
                }
                else if mega_pixels < 3.68 {
                    NVSDK_NGX_PerfQuality_Value_NVSDK_NGX_PerfQuality_Value_MaxQuality
                }
                else if mega_pixels < 8.29 {
                    NVSDK_NGX_PerfQuality_Value_NVSDK_NGX_PerfQuality_Value_MaxPerf
                }
                else {
                    NVSDK_NGX_PerfQuality_Value_NVSDK_NGX_PerfQuality_Value_UltraPerformance
                }
            }
            Self::Native => NVSDK_NGX_PerfQuality_Value_NVSDK_NGX_PerfQuality_Value_DLAA,
            Self::Quality => NVSDK_NGX_PerfQuality_Value_NVSDK_NGX_PerfQuality_Value_MaxQuality,
            Self::Balanced => NVSDK_NGX_PerfQuality_Value_NVSDK_NGX_PerfQuality_Value_Balanced,
            Self::Performance => NVSDK_NGX_PerfQuality_Value_NVSDK_NGX_PerfQuality_Value_MaxPerf,
            Self::Fastest => NVSDK_NGX_PerfQuality_Value_NVSDK_NGX_PerfQuality_Value_UltraPerformance,
        }
    }
}