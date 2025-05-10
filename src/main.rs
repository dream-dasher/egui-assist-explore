use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};
use num_complex::Complex64;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        // initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Imaginary Number Plot",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

#[derive(Default)]
struct MyApp {}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            Plot::new("complex_plot")
                .view_aspect(2.0)
                .show(ui, |plot_ui| {
                    plot_ui.line(complex_plot());
                });
        });
    }
}

fn complex_plot() -> Line<'static> {
    let points: PlotPoints = (0..1000)
        .map(|i| {
            let x = i as f64 * 0.01;
            let z = Complex64::new(x, 0.0);
            let value = z.exp(); // e^(ix)
            [x, value.im]
        })
        .collect();

    Line::new(points).name("Imaginary part of e^(ix)")
}
