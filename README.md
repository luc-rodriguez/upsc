## upsc

This is an ambitious project which aims to make modern graphics features such as:
- Super Resolution
- Frame Generation
- Async Reprojection

easier to integrate. Intended to be used with the incredible `wgpu` ecosystem.

Although bindings for them are generated here, We're aiming to make safe-ish, unified, solution for various proprietary SDKs, as well as custom fallbacks whenever possible.

## Notes / Disclaimers

As much as we would like to, due to the usually higher requirements of most upscaling technologies, this project may not function everywhere that `wgpu` may normally.
Consider it to be best-of-effort for the foreseeable future.

Although we seek to ease this as much as possible, many of the underlying technologies require fairly deep integration, and can be quite unsafe, which is usually out of our control.

When using this project, ensure that you have read & comply with **all** of the various terms, licenses, and include any required copyright or license blurbs.
The authors of this project will not be responsible for any misuse of any of the underlying technologies.

---

Other product names used in this publication are for identification purposes only and may be trademarks of their respective companies.

AMD, the AMD Arrow logo, Radeon, Ryzen, CrossFire, RDNA, FidelityFX and combinations thereof are trademarks of Advanced Micro Devices, Inc.

DirectX is a registered trademark of Microsoft Corporation in the US and other jurisdictions.

Vulkan and the Vulkan logo are registered trademarks of the Khronos Group Inc.

OpenCL is a trademark of Apple Inc. used by permission by Khronos Group, Inc.

Microsoft is a registered trademark of Microsoft Corporation in the US and other jurisdictions.

Windows is a registered trademark of Microsoft Corporation in the US and other jurisdictions.

## Contributing

Contributions are welcome. Please open an issue before starting work on a non-trivial PR.