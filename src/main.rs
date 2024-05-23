mod ui;
mod maze;

use egui_macroquad::macroquad::telemetry::disable;
use macroquad::prelude::*;
use maze::Maze;

fn window_configuration() -> Conf
{
  return Conf
  {
    window_title: "Rust Maze Generator".to_string(),
    window_width: 1290,
    window_height: 720,
    fullscreen: false,
    window_resizable: false,
    ..Conf::default()
  };
}

pub(crate) const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
pub(crate) const AUTHORS: Option<&str> = option_env!("CARGO_PKG_AUTHORS");

pub(crate) const BG_COLOR: u32 = 0x805c7f; // 200040
pub(crate) const CELL_COLOR: u32 = 0xc0c0c0;
pub(crate) const UNVISITED_CELL_COLOR: u32 = 0x000861;
pub(crate) const WALL_COLOR: u32 = 0x000000;

// 1768 cells
pub(crate) const MAZE_WIDTH: u32 = 52;
pub(crate) const MAZE_HEIGHT: u32 = 34;
pub(crate) const CELL_WIDTH: u32 = 15;
pub(crate) const WALL_WIDTH: u32 = 5;
pub(crate) const TOP_MAZE_OFFSET: u32 = 22;
pub(crate) const LEFT_MAZE_OFFSET: u32 = 21;

#[macroquad::main(window_configuration)]
async fn main()
{
  disable();

  let mut algorithm = Algorithm::DFS;
  let mut maze = Maze::new();
  // Top of stack
  let mut tos: Option<(usize, usize)> = None;
  let mut animate = true;
  let mut speed = 1; // In cells
  // let mut delay_bucket = 0;
  // let mut rng = thread_rng();
  // This boolean needs to be present everywhere in the app
  let mut generating = false;

  // TODO: implement many more generating algorithms
  // TODO: compile to wasm b/c the base functionality is already done
  // TODO: release 1.0.0 b/c it's ready
  // TODO: maybe implement strategy pattern for different maze generating algorithms?
  // TODO: tests

  // Helpful resources:
  // https://en.wikipedia.org/wiki/Maze_generation_algorithm
  // https://www.youtube.com/watch?v=cQVH4gcb3O4
  // https://www.youtube.com/watch?v=ffkSQ_LpSkc
  // https://www.youtube.com/watch?v=Y37-gB83HKE
  // https://www.youtube.com/watch?v=1GENvb4y95s&list=PLwIV1dqznwSfvkh5YVYF3ioumOVpMR9KV
  // https://www.youtube.com/watch?v=U3meEXvYFsc
  loop
  {
    if generating
    {
      if animate
      {
        for _ in 1..=speed
        {
          match algorithm
          {
            Algorithm::DFS => tos = maze.step_dfs(),
          }
        }
      }
      else
      {
        match algorithm
        {
          Algorithm::DFS =>
          {
            tos = maze.tos();
            while tos.is_some() { tos = maze.step_dfs(); }
          }
        }
      }

      if tos.is_none() { generating = false; }
    }

    maze::paint(
      &maze,
      &tos,
      &generating
    );

    ui::paint(
      &mut algorithm,
      &mut animate,
      &mut generating,
      &mut speed,
      &mut maze,
    );

    // Draw things before egui

    egui_macroquad::draw();

    // Draw things after egui

    next_frame().await;
  }
}

#[derive(PartialEq, Eq)]
enum Algorithm
{ DFS /*, Kruskal, Prim, Wilson */ }
