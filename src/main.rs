use egui::{emath::RectTransform, Color32, Pos2, Rect, Sense, Stroke, Ui, Vec2};
use std::f32::{consts::PI, EPSILON};

fn main() -> eframe::Result<()>
{
    // runs the app
    eframe::run_native(
        "Field Visualizer",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(App::new()))),
    )
}

struct App
{
    charges: Vec<Charge>,
    grid_size: usize
}

impl Default for App
{
    fn default() -> Self
    {
        Self {
            charges: Vec::new(),
            grid_size: 50
        }
    }
}

impl eframe::App for App
{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame)
    {
        egui::CentralPanel::default().show(&ctx, |ui|
        {
            // prepare painting area, normalize to fit all arrows
            let (response, painter) = ui.allocate_painter(
                ui.available_size_before_wrap(),
                Sense::drag()
            );
            let painter_proportions = response.rect.square_proportions();
            let to_screen = RectTransform::from_to(
                Rect::from_min_size(
                    Pos2::ZERO,
                    painter_proportions * (self.grid_size as f32)),
                response.rect
            );

            // invoke charges
            self.charges = vec![Charge::new(4.7, 5.8, 0)];
            
            // draw charges
            for charge in &mut self.charges
            {
                Self::draw_draggable_charge(ui, charge);
            }
            
            // invoke arrows
            let mut arrow_map: Vec<Vec<Arrow>> = Vec::new();
            let grid_size = self.grid_size;
            for y in 0..grid_size
            {
                let mut row: Vec<Arrow> = Vec::new();
                for x in 0..grid_size
                {
                    row.push(Arrow::new(Pos2::new(x as f32, y as f32)));
                }
                arrow_map.push(row);
            }
        
            // draw arrows
            for arrow_row in arrow_map
            {
                for arrow in arrow_row
                {
                    self.draw_arrows(arrow, &painter, to_screen);
                }
            }
        });
    }
}

impl App
{
    fn new() -> Self
    {
        Default::default()
    }
    fn draw_arrows(&self, arrow: Arrow, painter: &egui::Painter, to_screen: RectTransform)
    {
        // find direction of force, normalize (unit vector)
        let distance_vec = arrow.origin - self.charges[0].position;
        let unit_vec = distance_vec.normalized();

        // find force vector
        // currently missing PI*EPSILON ni first term.
        let factor = 4.; // emphasizing factor, otherwise small
        let force = factor * (1./(4.))*(1./(distance_vec.length()*distance_vec.length()))*unit_vec;

        painter.arrow(
            to_screen * arrow.origin,
            to_screen.scale() * force,
            Stroke::new(1.0, Color32::RED)
        );
    }

    fn draw_draggable_charge(ui: &mut Ui, charge: &mut Charge)
    {
        let radius = 6.;
        let id = egui::Id::new(charge.id);
        let rect = egui::Rect::from_center_size(
            charge.position,
            Vec2::splat(radius * 2.)
        );

        let response = ui.interact(rect, id, egui::Sense::click_and_drag());

        if response.dragged()
        {
            if let Some(pos) = response.interact_pointer_pos()
            {
                charge.position = pos;
            }
        }

        let painter = ui.painter();
        painter.circle_filled(charge.position, radius, egui::Color32::BLUE);
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
    id: usize,
    position: Pos2
}

impl Default for Charge
{
    fn default() -> Self
    {
        Self
        {
            id: 0,
            position: Pos2::new(0., 0.)
        }    
    }
}

impl Charge
{
    fn new(x: f32, y: f32, id: usize) -> Self
    {
        Self
        {
            id: id,
            position: Pos2::new(x, y)
        }
    }
}