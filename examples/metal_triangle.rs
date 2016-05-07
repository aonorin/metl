extern crate metl;

use metl::extras::window::{Event, WindowBuilder};
use metl::ClearColor;

fn main() {
    let device = metl::Device::system_default_device().unwrap();
    let mut window = WindowBuilder::new(device)
                     .with_dimensions(800, 600)
                     .with_title("Metal Window".into())
                     .build()
                     .unwrap();
    window.view.set_clear_color(ClearColor::new(1.0, 0.0, 0.0, 1.0));

    'mainloop: loop {
        for event in window.poll_events() {
            match event {
                Event::Closed => {
                    break 'mainloop
                }
                _ => {}
            }
        }

        // drawing code here
        window.view.draw();
    }
}