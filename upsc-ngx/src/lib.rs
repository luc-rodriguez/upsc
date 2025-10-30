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