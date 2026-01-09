use std::default;

use egui::{emath::RectTransform, Color32, Pos2, Rect, Sense, Stroke, Vec2};

fn main() -> eframe::Result<()>
{
    // runs the app
    eframe::run_native(
        "Field Visualizer",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(VisApp::new()))),
    )
}

struct VisApp
{}

impl Default for VisApp
{
    fn default() -> Self
    {
        Self {}
    }
}

impl eframe::App for VisApp
{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame)
    {
        egui::CentralPanel::default().show(&ctx, |ui|
        {
            ui.label("Hello, world!");
            let (response, painter) = ui.allocate_painter(
                ui.available_size_before_wrap(),
                Sense::drag()
            );
            let painter_proportions = response.rect.square_proportions();
            let to_screen = RectTransform::from_to(
                Rect::from_min_size(
                    Pos2::ZERO - painter_proportions,
                    2. * painter_proportions),
                response.rect,);
            let arrow = Arrow::default();
            painter.arrow(
                to_screen * arrow.origin,
                to_screen.scale() * arrow.direction,
                Stroke::new(1.0, Color32::RED)
            );
        });
    }
}

impl VisApp
{
    fn new() -> Self
    {
        Default::default()
    }
}

struct Arrow
{
    origin: Pos2,
    direction: Vec2,
}

impl Default for Arrow
{
    fn default() -> Self
    {
        Self
        {
            origin: Pos2::new(0., 0.),
            direction: Vec2::new(1., 1.)
        }
    }
}