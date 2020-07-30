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

fn main() {
	let instance = Instance::new(None, &InstanceExtensions::none(), None)
		.expect("failed to create instance");

	let physical = PhysicalDevice::enumerate(&instance).next().expect("no device available");

	let queue_family = physical.queue_families()
		.find(|&q| q.supports_graphics())
		.expect("couldn't find a graphical queue family");

	let (device, mut queues) = {
		Device::new(physical, &Features::none(), &DeviceExtensions::none(),
					[(queue_family, 0.5)].iter().cloned()).expect("failed to create device")
	};

	let queue = queues.next().unwrap();

	let data = MyStruct { a: 12, b: true };

	let buffer_data = CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), false,
												data).unwrap();

	let iter = (0 .. 128).map(|_| 5u8);

	let buffer_iter = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false,
												iter).unwrap();


	let mut builder = AutoCommandBufferBuilder::new(device.clone(), queue.family()).unwrap();

	let iter2 = (0 .. 128).map(|_| 12u8);

	let buffer_iter2 = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false,
													 iter2).unwrap();

	builder.copy_buffer(buffer_iter2.clone(), buffer_iter.clone()).unwrap();

	let command_buffer = builder.build().unwrap();

	let finished = command_buffer.execute(queue.clone()).unwrap();

	finished.then_signal_fence_and_flush().unwrap()
		.wait(None).unwrap();

	let mut ctt_iter = buffer_iter.write().unwrap();

	ctt_iter[3] = buffer_data.read().unwrap().a as u8;

	if device.physical_device().name() == queue.device().physical_device().name() {
		show("Buffer contain: ".to_string() + &*array_as_string(ctt_iter.as_mut()));
	} else {
		show("current device ".to_string() + &*queue.device().physical_device().name().to_string());
	}
}

fn show(arg: String) {
	println!("What is {:?} ?", arg);
}

struct MyStruct {
	a: u32,
	b: bool,
}

fn array_as_string(array: &mut [u8]) -> String {
	let mut muted_array: String = "".to_string();
	for i in array.iter() {
		let mut muted_val = i.to_string();
		muted_val.push_str(&*", ".to_string());
		muted_array.push_str(&*muted_val);
	}
	return muted_array
}
