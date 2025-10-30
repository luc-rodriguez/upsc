#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(unsafe_op_in_unsafe_fn)] // https://github.com/rust-lang/rust-bindgen/issues/3243

include!(concat!(env!("OUT_DIR"), "/deps.rs"));

#[cfg(feature = "vulkan")]
use ash::vk::{
    ImageAspectFlags,
    ImageSubresourceRange,
    REMAINING_ARRAY_LAYERS,
    REMAINING_MIP_LEVELS,

    PFN_vkGetDeviceProcAddr,
    PFN_vkGetInstanceProcAddr,

    ImageSubresourceRange as VkImageSubresourceRange,
    ExtensionProperties as VkExtensionProperties,
    PhysicalDevice as VkPhysicalDevice,
    CommandBuffer as VkCommandBuffer,
    ImageView as VkImageView,
    Instance as VkInstance,
    Buffer as VkBuffer,
    Device as VkDevice,
    Format as VkFormat,
    Image as VkImage,
};

use wgpu::{Adapter, TextureUsages, TextureView};

#[cfg(feature = "vulkan")]
use wgpu::wgc::api::Vulkan;

#[cfg(feature = "vulkan")]
pub fn texture_to_ngx(texture_view: &TextureView, adapter: &Adapter) -> NVSDK_NGX_Resource_VK {
    unsafe {
        let raw_view = texture_view.as_hal::<Vulkan>().unwrap().raw_handle();
        let texture = texture_view.texture();

        NVSDK_NGX_Create_ImageView_Resource_VK(
            raw_view,
            texture.as_hal::<Vulkan>().unwrap().raw_handle(),
            ImageSubresourceRange {
                aspect_mask: if texture.format().has_color_aspect() {
                    ImageAspectFlags::COLOR
                } else {
                    ImageAspectFlags::DEPTH
                },
                base_mip_level: 0,
                level_count: REMAINING_MIP_LEVELS,
                base_array_layer: 0,
                layer_count: REMAINING_ARRAY_LAYERS,
            },
            adapter
                .as_hal::<Vulkan>()
                .unwrap()
                .texture_format_as_raw(texture.format()),
            texture.width(),
            texture.height(),
            texture.usage().contains(TextureUsages::STORAGE_BINDING),
        )
    }
}

pub fn halton_sequence(mut index: u32, base: u32) -> f32 {
    let mut f = 1.0;
    let mut result = 0.0;
    
    while index > 0 {
        f /= base as f32;
        result += f * (index % base) as f32;
        index = (index as f32 / base as f32).floor() as u32;
    }

    result
}