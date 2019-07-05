use vulkano::instance::PhysicalDevice;

use raytrace::vulkan::Vulkan;

fn main() {
    let instance = Vulkan::create_instance();
    PhysicalDevice::enumerate(&instance).for_each(|device| {
        eprintln!("{:?}", device);
        device.queue_families().for_each(|family| {
            println!("{:?}", family);
            dbg!(family.supports_graphics());
            dbg!(family.supports_compute());
            dbg!(family.supports_graphics());
            dbg!(family.queues_count());
        })
    });
}
