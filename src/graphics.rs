use vulkano::instance::Instance;
use vulkano::instance::InstanceExtensions;
use vulkano::instance::PhysicalDevice;
use vulkano::device::Device;
use vulkano::device::DeviceExtensions;
use vulkano::device::Features;
use vulkano::buffer::BufferUsage;
use vulkano::buffer::CpuAccessibleBuffer;
use vulkano::command_buffer::AutoCommandBufferBuilder;
use vulkano::command_buffer::CommandBuffer;
use vulkano::sync::GpuFuture;
use vulkano::pipeline::ComputePipeline;
use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use vulkano::descriptor::PipelineLayoutAbstract;
use vulkano::format::Format;
use vulkano::image::{Dimensions, ImageAccess};
use vulkano::image::StorageImage;
use std::sync::Arc;
use std::iter::*;

#[path = "shaders.rs"]
mod shaders;

pub(crate) fn init_gpu_graphics() -> String {
    let instance = Instance::new(None, &InstanceExtensions::none(), None)
        .expect("failed to create instance");

    let physical = PhysicalDevice::enumerate(&instance).next().expect("no device available");
    let queue_family = physical.queue_families()
        .find(|&q| q.supports_graphics())
        .expect("couldn't find a graphical queue family");

    let (device, mut queues) = {
        Device::new(
            physical,
            &Features::none(),
            &DeviceExtensions::none(),
            [(queue_family, 0.5)].iter().cloned()).expect("failed to create device")
    };
    let queue = queues.next().unwrap();

    let data = MyStruct { a: 12, b: true };
    let buffer_data = CpuAccessibleBuffer::from_data(
        device.clone(),
        BufferUsage::all(),
        false,
        data).unwrap();

    let iter = (0..128).map(|_| 5u8);
    let buffer_iter = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false,
                                                     iter).unwrap();

    let mut builder = AutoCommandBufferBuilder::new(device.clone(), queue.family()).unwrap();

    let iter2 = (0..128).map(|_| 12u8);
    let buffer_iter2 = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false,
                                                      iter2).unwrap();

    builder.copy_buffer(buffer_iter2.clone(), buffer_iter.clone()).unwrap();
    let command_buffer = builder.build().unwrap();

    let finished = command_buffer.execute(queue.clone()).unwrap();
    finished.then_signal_fence_and_flush().unwrap()
        .wait(None).unwrap();

    let mut ctt_iter = buffer_iter.write().unwrap();
    ctt_iter[3] = buffer_data.read().unwrap().a as u8;

    let data_iter = 0..65536;
    let data_buffer = CpuAccessibleBuffer::from_iter(
        device.clone(),
        BufferUsage::all(),
        false,
        data_iter).expect("failed to create buffer");

        let shader = shaders::cs::Shader::load(device.clone())
            .expect("failed to create shader module");

        let compute_pipeline = Arc::new(
            ComputePipeline::new(
                device.clone(),
                &shader.main_entry_point(),
                &()).expect("failed to create compute pipeline"));

        let layout = compute_pipeline.layout().descriptor_set_layout(0).unwrap();
        let set = Arc::new(PersistentDescriptorSet::start(layout.clone())
            .add_buffer(data_buffer.clone()).unwrap()
            .build().unwrap()
        );

        let mut builder = AutoCommandBufferBuilder::new(device.clone(), queue.family()).unwrap();
        builder.dispatch([1024, 1, 1], compute_pipeline.clone(), set.clone(), ()).unwrap();
        let command_buffer = builder.build().unwrap();

    let finished = command_buffer.execute(queue.clone()).unwrap();

    finished.then_signal_fence_and_flush().unwrap()
        .wait(None).unwrap();

    let _content = data_buffer.read().unwrap();

    let image = StorageImage::new(device.clone(), Dimensions::Dim2d { width: 1024, height: 1024 },
                                  Format::R8G8B8A8Unorm, Some(queue.family())).unwrap();

    return array_as_string(image.dimensions().width_height().as_mut());
}


struct MyStruct {
    a: u32,
    b: bool,
}


impl MyStruct {
    fn as_box(&self, size: usize) -> Box<u32> {
        return Box::new(self.a);
    }
}

fn array_as_string(array: &mut [u32]) -> String {
    let mut muted_array: String = "".to_string();
    for i in array.iter() {
        let mut muted_val = i.to_string();
        muted_val.push_str(&*", ".to_string());
        muted_array.push_str(&*muted_val);
    }
    return muted_array;
}