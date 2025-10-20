use eframe::egui;
use egui::Color32;
use std::f32::consts::PI;

struct ShaderApp {
    time: f32,
}

impl Default for ShaderApp {
    fn default() -> Self {
        Self { time: 0.0 }
    }
}

impl eframe::App for ShaderApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.time += ctx.input(|i| i.stable_dt).unwrap_or(0.016);

        egui::CentralPanel::default().show(ctx, |ui| {
            let (rect, _) = ui.allocate_exact_size(ui.available_size(), egui::Sense::hover());

            let painter = ui.painter();

            // Simple gradient pattern that oscillates over time
            for y in (rect.top() as i32..rect.bottom() as i32).step_by(4) {
                let t = (y as f32 / rect.height()) + (self.time * 0.2).sin() * 0.3;
                let r = (t * PI).sin() * 127.0 + 128.0;
                let g = (t * PI * 1.3).sin() * 127.0 + 128.0;
                let b = (t * PI * 1.7).sin() * 127.0 + 128.0;
                let color = Color32::from_rgb(r as u8, g as u8, b as u8);

                painter.rect_filled(
                    egui::Rect::from_min_max(
                        egui::pos2(rect.left(), y as f32),
                        egui::pos2(rect.right(), (y + 4) as f32),
                    ),
                    0.0,
                    color,
                );
            }

            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                "EGUI SHADER DEMO",
                egui::FontId::proportional(24.0),
                Color32::WHITE,
            );
        });

        ctx.request_repaint(); // Keep animating
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    wasm_bindgen_futures::spawn_local(async {
        let web_options = eframe::WebOptions::default();
        eframe::start_web(
            "canvas",
            web_options,
            Box::new(|_cc| Box::new(ShaderApp::default())),
        )
        .await
        .expect("failed to start eframe");
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Egui Shader Demo",
        native_options,
        Box::new(|_cc| Box::new(ShaderApp::default())),
    )
    .unwrap();
}
