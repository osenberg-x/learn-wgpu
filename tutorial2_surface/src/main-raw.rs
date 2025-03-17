use tutorial2_surface::run;

fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        run();
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        pollster::block_on(run());
    }
}
