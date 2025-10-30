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