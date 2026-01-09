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
            ui.label("Hello, world!")
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
    a: Coordinate,
    b: Coordinate,
    dir: Vector,
}

struct Coordinate
{
    x: i32,
    y: i32,
}

struct Vector
{
    x: i32,
    y: i32,
}