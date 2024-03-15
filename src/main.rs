mod ui;
mod maze;

use ::rand::{Rng, thread_rng};
use egui_macroquad::macroquad::telemetry::disable;
use macroquad::prelude::*;
use maze::{Maze, Direction};

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

pub(crate) const BG_COLOR: u32 = 0x555555; // 200040
pub(crate) const CELL_COLOR: u32 = 0xc0c0c0;
pub(crate) const UNVISITED_CELL_COLOR: u32 = 0x404040;
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
  let mut stack_dfs: Vec<(usize, usize)> = vec![];
  let mut animate = true;
  let mut delay = 0.0000; // In seconds
  let mut delta_time_bucket = 0.0;
  let mut rng = thread_rng();
  let mut generating = false;

  stack_dfs.push((0,0));

  // TODO: implement maze solver asw

  // Helpful resources:
  // https://en.wikipedia.org/wiki/Maze_generation_algorithm
  // https://www.youtube.com/watch?v=cQVH4gcb3O4
  // https://www.youtube.com/watch?v=ffkSQ_LpSkc
  // https://www.youtube.com/watch?v=Y37-gB83HKE
  loop
  {
    if generating
    {
      if animate
      {
        delta_time_bucket += get_frame_time();
        if delta_time_bucket > delay
        {
          delta_time_bucket = 0.0;
          match algorithm
          {
            Algorithm::DFS     => { generating = maze.step_dfs(&mut stack_dfs, &mut rng); }
            Algorithm::Kruskal => { generating = maze.step_kruskal(); }
            Algorithm::Prim    => { generating = maze.step_prim(); }
            Algorithm::Wilson  => { generating = maze.step_wilson(); }
          }
        }
      }
      else
      {
        match &algorithm
        {
          Algorithm::DFS     => maze.create_dfs(),
          Algorithm::Kruskal => maze.create_kruskal(),
          Algorithm::Prim    => maze.create_prim(),
          Algorithm::Wilson  => maze.create_wilson(),
        }
      }
    }

    // Making sure that no panics happen
    if stack_dfs.is_empty() { stack_dfs.push((0,0)); }

    maze::paint(&mut maze);

    // Process keys, mouse etc.

    ui::paint(
      &mut algorithm,
      &mut animate,
      &mut generating,
      &mut delay,
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
{
  DFS, Kruskal, Prim, Wilson
}

