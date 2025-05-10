use egui::Ui;
use egui_plot::{Line, Plot, PlotPoints};
use num_complex::Complex64;
use tracing::debug;

fn main() -> Result<(), eframe::Error> {
    // Initialize logging for debug messages
    tracing_subscriber::fmt::init();

    // Configure application window
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 700.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Exponential Function Visualizer",
        options,
        Box::new(|_cc| {
            // Initialize the app with default values
            let app = MyApp {
                max_x_real: 5,     // Default value for real plot
                max_x_complex: 10, // Default value for complex plot (good for seeing complete cycles)
            };

            Ok(Box::new(app))
        }),
    )
}

#[derive(Default)]
struct MyApp {
    // State for controlling the maximum x-value for each plot
    max_x_real: usize,    // max x-value for e^x plot
    max_x_complex: usize, // max x-value for e^ix plot
}

// Helper function to render controls for the real exponential
fn render_real_controls(ui: &mut Ui, max_x_real: &mut usize) {
    ui.group(|ui| {
        ui.heading("Real Exponential (e^x)");
        ui.label("The function e^x grows exponentially as x increases");

        ui.horizontal(|ui| {
            ui.label("Max x value:");
            ui.add(
                egui::Slider::new(max_x_real, 1..=100)
                    .text("x")
                    .logarithmic(true),
            );
        });

        // Add formula with rich text
        ui.label("Formula: f(x) = e^x");
        ui.label("Growth is unbounded as x increases");
    });
}

// Helper function to render controls for the complex exponential
fn render_complex_controls(ui: &mut Ui, max_x_complex: &mut usize) {
    ui.group(|ui| {
        ui.heading("Complex Exponential (e^ix)");
        ui.label("By Euler's formula: e^ix = cos(x) + i·sin(x)");

        ui.horizontal(|ui| {
            ui.label("Max x value:");
            ui.add(
                egui::Slider::new(max_x_complex, 1..=100)
                    .text("x")
                    .logarithmic(true),
            );
        });

        ui.label("Both components are bounded between -1 and 1");
        ui.label("This represents circular motion in the complex plane");
    });
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Add controls for adjusting max_x values
            ui.heading("Exponential Function Visualization");

            ui.collapsing("About these functions", |ui| {
                ui.label("This application visualizes two important exponential functions:");
                ui.label("• e^x - The standard real exponential function that grows exponentially");
                ui.label("• e^ix - The complex exponential function (Euler's formula: e^ix = cos(x) + i·sin(x))");
                ui.label("These functions are fundamental in mathematics, physics, and engineering.");

                ui.add_space(10.0);
                ui.label("Try adjusting the sliders to see how these functions behave!");
            });

            ui.add_space(10.0);
            ui.heading("Plot Controls");

            // Controls section - arrange in two columns
            ui.columns(2, |columns| {
                // Left column - Real exponential
                render_real_controls(&mut columns[0], &mut self.max_x_real);

                // Right column - Complex exponential
                render_complex_controls(&mut columns[1], &mut self.max_x_complex);
            });

            ui.separator();

            // Show plot with both functions
            Plot::new("combined_plot")
                .view_aspect(2.0)
                .legend(egui_plot::Legend::default().position(egui_plot::Corner::LeftTop))
                .x_axis_label("x")
                .y_axis_label("f(x)")
                .show(ui, |plot_ui| {
                    // Plot e^x (real exponential) with the user-specified max_x value
                    plot_ui.line(real_plot(Some(self.max_x_real)));

                    // Plot e^ix (complex exponential - both real and imaginary parts)
                    // using the user-specified max_x value
                    let (real_part, imag_part) = complex_plot(Some(self.max_x_complex));
                    plot_ui.line(real_part);
                    plot_ui.line(imag_part);
                });
        });
    }
}

fn real_plot(max_x: Option<usize>) -> Line<'static> {
    // Define resolution of the plot (smaller delta = smoother curve)
    const DELTA: f64 = 0.001;
    const DELTAS_PER_POINT: usize = (1.0 / DELTA) as usize;

    // Use provided max_x or fall back to default
    let max_x = max_x.unwrap_or({
        debug!("using default value for real plot");
        100
    });

    let points: PlotPoints = (0..max_x * DELTAS_PER_POINT)
        .map(|x| {
            let x = x as f64 * DELTA;
            let value = x.exp(); // e^x (real exponential)
            [x, value]
        })
        .collect();

    // Return the line with a distinctive color
    Line::new(points)
        .name("e^x (real exponential)")
        .width(2.5) // Make line slightly thicker
        .color(egui::Color32::from_rgb(255, 100, 100)) // Red color
        .highlight(true) // Highlight on hover
}
fn complex_plot(max_x: Option<usize>) -> (Line<'static>, Line<'static>) {
    // Define resolution of the plot (smaller delta = smoother curve)
    const DELTA: f64 = 0.001;
    const DELTAS_PER_POINT: usize = (1.0 / DELTA) as usize;

    // Use provided max_x or fall back to default
    let max_x = max_x.unwrap_or({
        debug!("using default value for complex plot");
        100
    });

    // Generate points for real and imaginary parts
    let real_points: PlotPoints = (0..max_x * DELTAS_PER_POINT)
        .map(|i| {
            let x = i as f64 * DELTA;
            // For e^ix, we need a complex number with imaginary component x
            let z = Complex64::new(0.0, x);
            let value = z.exp(); // e^(ix)
            [x, value.re] // Real part is cos(x)
        })
        .collect();

    let imag_points: PlotPoints = (0..max_x * DELTAS_PER_POINT)
        .map(|i| {
            let x = i as f64 * DELTA;
            let z = Complex64::new(0.0, x);
            let value = z.exp(); // e^(ix)
            [x, value.im] // Imaginary part is sin(x)
        })
        .collect();

    // Return both lines with distinctive colors
    let real_line = Line::new(real_points)
        .name("e^ix (real part = cos(x))")
        .width(2.5) // Make line slightly thicker
        .color(egui::Color32::from_rgb(100, 100, 255)) // Blue color
        .highlight(true); // Highlight on hover

    let imag_line = Line::new(imag_points)
        .name("e^ix (imaginary part = sin(x))")
        .width(2.5) // Make line slightly thicker
        .color(egui::Color32::from_rgb(100, 255, 100)) // Green color
        .highlight(true); // Highlight on hover

    (real_line, imag_line)
}
