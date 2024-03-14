use egui_macroquad::{egui::{epaint::Shadow, Align2, Color32, Rounding, Stroke, Vec2, Visuals, Window}, ui};

use crate::{AUTHORS, VERSION, Algorithm};

pub(crate) fn paint(algorithm: &mut Algorithm, animate: &mut bool)
{
  ui(|egui_context|
  {
    egui_context.set_visuals(Visuals
    {
      window_shadow: Shadow::NONE,
      window_rounding: Rounding
      { nw: 10., ne: 0., sw: 10., se: 0. },
      // window_fill: Color32::from_rgb(32, 0, 64),
      // window_stroke: Stroke::new(2., Color32::from_rgb(0, 192, 192)),
      // override_text_color: Some(Color32::from_rgb(255, 210, 255)),
      ..Default::default()
    });

    Window::new("Rust Maze Generator")
      .anchor(Align2::RIGHT_TOP, Vec2::new(-1.5, 10.))
      .constrain(true)
      .collapsible(false)
      .movable(false)
      .resizable(false)
      .fixed_size(Vec2::new(200., 0.))
      .show(egui_context, |ui|
      {
        // ui.style_mut().visuals.widgets.inactive.weak_bg_fill = Color32::from_rgb(0, 64, 64);
        // ui.style_mut().visuals.widgets.inactive.bg_fill = Color32::from_rgb(0, 64, 64);
        // ui.style_mut().visuals.widgets.hovered.weak_bg_fill = Color32::from_rgb(0, 128, 128);
        // ui.style_mut().visuals.widgets.hovered.bg_fill = Color32::from_rgb(0, 128, 128);
        // ui.style_mut().visuals.widgets.active.weak_bg_fill = Color32::from_rgb(0, 192, 192);
        // ui.style_mut().visuals.widgets.active.bg_fill = Color32::from_rgb(0, 192, 192);

        ui.label("This is the Rust maze generator.");

        ui.separator();

        ui.checkbox(animate, "Animate maze generation");

        ui.separator();

        ui.label("Select an Algorithm:");

        ui.radio_value(algorithm, Algorithm::DFS, "Iterative DFS (depth first search)");
        ui.radio_value(algorithm, Algorithm::Kruskal, "Kruskal's Algorithm");
        ui.radio_value(algorithm, Algorithm::Prim, "Prim's Algorithm");
        ui.radio_value(algorithm, Algorithm::Wilson, "Wilson's Algorithm");

        ui.separator();

        if *algorithm == Algorithm::DFS
        {
          ui.label("Biases");
          ui.label("Delay (in ms)");
          ui.separator();
        }

        if ui.button("Generate").clicked()
        {
          // Generate maze
        }

        ui.label("// Adjust delay time");

        ui.label("// {OPTIONAL}: adjust maze size");

        ui.label("// Insert stuff here");

        ui.add_space(200.);

        ui.separator();

        // --- CREDITS (!important) ---
        ui.horizontal(|ui|
        {
          ui.label(format!("v{}", VERSION.unwrap_or("unknown")));
          ui.separator();
          ui.label("Made by");
          ui.hyperlink_to(format!("{}", AUTHORS.unwrap_or("unknown")), "https://github.com/an-Iceberg");
        });
      });
  });
}

