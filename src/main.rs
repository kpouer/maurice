use maurice::gui::Gui;
use maurice::hardware;
use maurice::hardware::machine::Machine;
use maurice::hardware::screen::{HEIGHT, WIDTH};
use maurice::raw_image::RawImage;
use maurice::user_input::UserInput;
use std::sync::mpsc::channel;
use std::thread;

fn main() {
    #[cfg(not(any(feature = "speedy2d-display", feature = "egui-display")))]
    compile_error!("You must activate speedy2d or egui.");
    #[cfg(all(feature = "speedy2d-display", feature = "egui-display"))]
    compile_error!("You must activate only one of speedy2d or egui.");

    env_logger::init();
    let (image_data_sender, image_data_receiver) = channel::<RawImage>();
    let (user_input_sender, user_input_receiver) = channel::<UserInput>();

    thread::spawn(move || {
        let mut machine = Machine::new(image_data_sender, user_input_receiver);
        machine.run_loop()
    });
    #[cfg(feature = "egui-display")]
    {
        let native_options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_drag_and_drop(true)
                .with_inner_size([(3 * WIDTH) as f32, (3 * HEIGHT) as f32]),
            ..Default::default()
        };
        let gui = Gui::new(user_input_sender, image_data_receiver);
        let _ = eframe::run_native("Maurice", native_options, Box::new(|_cc| Ok(Box::new(gui))));
    }
    #[cfg(feature = "speedy2d-display")]
    {
        let window = speedy2d::Window::new_centered(
            "Maurice",
            (
                (hardware::screen::DEFAULT_PIXEL_SIZE * WIDTH) as u32,
                (hardware::screen::DEFAULT_PIXEL_SIZE * HEIGHT) as u32,
            ),
        );

        match window {
            Ok(window) => window.run_loop(Gui::new(user_input_sender, image_data_receiver)),
            Err(e) => {
                log::error!("Error creating window: {e}");
            }
        }
    }
}
