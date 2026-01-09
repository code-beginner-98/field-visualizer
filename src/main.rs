use egui::{emath::RectTransform, Color32, Pos2, Rect, Sense, Stroke};

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
            // invoke arrows
            let mut arrow_map: Vec<Vec<Arrow>> = Vec::new();
            for y in 0..4
            {
                let mut row: Vec<Arrow> = Vec::new();
                for x in 0..10
                {
                    row.push(Arrow::new(Pos2::new(x as f32, y as f32)));
                }
                arrow_map.push(row);
            }

            // invokes charges
            let charge = Charge::new(1.6, 2.3);

            // prepare painting area, normalize to fit all arrows
            let (response, painter) = ui.allocate_painter(
                ui.available_size_before_wrap(),
                Sense::drag()
            );
            let painter_proportions = response.rect.square_proportions();
            let to_screen = RectTransform::from_to(
                Rect::from_min_size(
                    Pos2::ZERO,
                    painter_proportions * (arrow_map.len() as f32)),
                response.rect
            );

            // paint charges
            painter.circle(
                charge.position,
                1.,
                Color32::RED,
                Stroke::new(1.0, Color32::RED));

            // paint arrows
            for arrow_row in arrow_map
            {
                for arrow in arrow_row
                {
                    let arrow_direction = arrow.origin - charge.position;
                    painter.arrow(
                        to_screen * arrow.origin,
                        to_screen.scale() * arrow_direction,
                        Stroke::new(1.0, Color32::RED)
                    );
                }
            }
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
}

impl Default for Arrow
{
    fn default() -> Self
    {
        Self
        {
            origin: Pos2::new(0.5, 0.25),
        }
    }
}

impl Arrow
{
    fn new(origin: Pos2) -> Self
    {
        Self
        {
            origin: origin,
        }
    }
}
struct Charge
{
    position: Pos2
}

impl Default for Charge
{
    fn default() -> Self
    {
        Self
        {
            position: Pos2::new(0., 0.)
        }    
    }
}

impl Charge
{
    fn new(x: f32, y: f32) -> Self
    {
        Self
        {
            position: Pos2::new(x, y)
        }
    }
}