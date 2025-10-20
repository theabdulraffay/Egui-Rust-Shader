use eframe::egui;
use three_d::*;
extern crate winapi;

use winapi::um::winnt::*;

fn main() {
    let options = eframe::WebOptions::default();
    
    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "canvas",
                options,
                Box::new(|cc| Box::new(MyApp::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}

struct MyApp {
    three_d_data: Option<ThreeDData>,
    rotation: f32,
    color: [f32; 3],
    auto_rotate: bool,
}

struct ThreeDData {
    context: Context,
    camera: Camera,
    model: Gm<Mesh, ColorMaterial>,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext) -> Self {
        // Get the gl context - it should exist for web
        let gl_context = cc.gl.as_ref().expect("Failed to get GL context").clone();
        
        // Initialize three-d context
        let context = Context::from_gl_context(gl_context).unwrap();
        
        // Create camera
        let viewport = Viewport::new_at_origo(800, 600);
        let camera = Camera::new_perspective(
            viewport,
            vec3(0.0, 0.0, 3.0),
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
            degrees(45.0),
            0.1,
            100.0,
        );

        // Create a cube mesh
        let mut cpu_mesh = CpuMesh::cube();
        cpu_mesh.transform(Mat4::from_scale(0.5)).unwrap();
        
        let model = Gm::new(
            Mesh::new(&context, &cpu_mesh),
            ColorMaterial {
                color: Srgba::new(100, 200, 255, 255),
                ..Default::default()
            },
        );

        let three_d_data = Some(ThreeDData {
            context,
            camera,
            model,
        });

        Self {
            three_d_data,
            rotation: 0.0,
            color: [0.4, 0.8, 1.0],
            auto_rotate: true,
        }
    }

    fn render_3d(&mut self, viewport: Viewport) {
        if let Some(data) = &mut self.three_d_data {
            // Update camera viewport
            data.camera.set_viewport(viewport);
            
            // Update model rotation
            data.model.set_transformation(Mat4::from_angle_y(radians(self.rotation)));
            
            // Update color
            data.model.material.color = Srgba::new(
                (self.color[0] * 255.0) as u8,
                (self.color[1] * 255.0) as u8,
                (self.color[2] * 255.0) as u8,
                255,
            );
            
            // Render with lighting
            RenderTarget::screen(&data.context, viewport.width, viewport.height)
                .clear(ClearState::color_and_depth(0.1, 0.1, 0.15, 1.0, 1.0))
                .render(
                    &data.camera,
                    data.model.into_iter(),
                    &[&DirectionalLight::new(
                        &data.context,
                        2.0,
                        Srgba::WHITE,
                        vec3(-1.0, -1.0, -1.0),
                    )],
                );
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("controls").show(ctx, |ui| {
            ui.heading("Controls");
            ui.separator();
            
            ui.label("Rotation:");
            ui.add(egui::Slider::new(&mut self.rotation, 0.0..=360.0).suffix("°"));
            
            ui.separator();
            ui.label("Color:");
            ui.color_edit_button_rgb(&mut self.color);
            
            ui.separator();
            ui.label("Auto-rotate:");
            ui.checkbox(&mut self.auto_rotate, "Enabled");
            
            ui.separator();
            if ui.button("Reset").clicked() {
                self.rotation = 0.0;
                self.color = [0.4, 0.8, 1.0];
                self.auto_rotate = true;
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let rect = ui.available_rect_before_wrap();
            let viewport = Viewport::new_at_origo(
                rect.width() as u32,
                rect.height() as u32,
            );
            
            // Render 3D scene
            self.render_3d(viewport);
            
            // Auto-rotate
            if self.auto_rotate {
                self.rotation = (self.rotation + 1.0) % 360.0;
            }
            
            // Request continuous repainting
            ctx.request_repaint();
        });
    }
}