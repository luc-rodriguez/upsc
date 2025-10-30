# Presets

Different proprietary solutions include different presents for upscaling, here we have them listed, sorted by rough equivalence:


|            DLSS            |            FSR            |            XeSS            |
| :--------------------------: | :--------------------------: | :---------------------------: |
|   Native ("DLAA",`1.0x`)   |      Native (`1.0x`)      |       Native (`1.0x`)       |
|            Auto            |             -             |              -              |
|             -             |             -             | Ultra Quality Plus (`1.3x`) |
|      Quality (`1.5x`)      |      Quality (`1.5x`)      |   Ultra Quality (`1.5x`)   |
|     Balanced (`1.72x`)     |     Balanced (`1.7x`)     |      Quality (`1.7x`)      |
|    Performance (`2.0x`)    |    Performance (`2.0x`)    |      Balanced (`2.0x`)      |
|             -             |             -             |    Performance (`2.3x`)    |
| Ultra Performance (`3.0x`) | Ultra Performance (`3.0x`) | Ultra Performance (`3.0x`) |

# Notes

1. The term "native" here usually refers to rendering at full resolution, but with anti-aliasing applied.
   ^ Anti-aliasing is currently outside the scope of this project, and so when a propretary solution cannot be used, and the shader fallback is engaged, there won't be any anti-aliasing applied.
2. DLSS's "Auto" preset will adjust in real-time to achieve user-defined fps targets (e.g. the primary monitor's refresh rate) for a more consistent framerate.
   ^ Internally, this crate tries to replicate similar functionality for other backends.
3. For some of these, it is possible to define custom input/output resolutions, in which case the scale factors listed (by `1.--x`) may not be absolute.
   ^ Although we don't expose this functionality at this time.

# References

1. https://github.com/NVIDIAGameWorks/NVIDIAImageScaling/blob/main/docs/RTX%20UI%20Developer%20Guidelines.pdf
2. https://gpuopen.com/fidelityfx-super-resolution-4/#information
3. https://www.intel.com/content/www/us/en/developer/topic-technology/gamedev/xess2.html
4. https://en.wikipedia.org/wiki/Deep_Learning_Super_Sampling
