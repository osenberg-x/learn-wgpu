use winit::window::Window;

fn main() {
    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY,
        ..Default::default()
    });

    let _adapter = instance
        .enumerate_adapters(wgpu::Backends::all())
        .iter()
        .filter(|adapter| {
            println!("{:?}", adapter);
            println!("{:?}", adapter.get_info());
            println!("{:?}", adapter.features());
            true
        })
        .next()
        .unwrap();
}
