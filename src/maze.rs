use macroquad::{window::clear_background, color::Color, shapes::draw_rectangle};

use crate::{MAZE_WIDTH, BG_COLOR, LEFT_MAZE_OFFSET, WALL_WIDTH, TOP_MAZE_OFFSET, CELL_WIDTH, WALL_COLOR, MAZE_HEIGHT, UNVISITED_CELL_COLOR, CELL_COLOR};

pub(crate) fn paint(maze: &mut Maze)
{
  clear_background(Color::from_hex(BG_COLOR));

  // Top maze wall
  draw_rectangle(
    (LEFT_MAZE_OFFSET - WALL_WIDTH) as f32,
    (TOP_MAZE_OFFSET - WALL_WIDTH) as f32,
    ((MAZE_WIDTH * (CELL_WIDTH + WALL_WIDTH)) + WALL_WIDTH) as f32,
    WALL_WIDTH as f32,
    Color::from_hex(WALL_COLOR)
  );
  // Left maze wall
  draw_rectangle(
    (LEFT_MAZE_OFFSET - WALL_WIDTH) as f32,
    TOP_MAZE_OFFSET as f32,
    WALL_WIDTH as f32,
    (MAZE_HEIGHT * (CELL_WIDTH + WALL_WIDTH)) as f32,
    Color::from_hex(WALL_COLOR)
  );

  // Paints the maze onto the screen
  for x in 0..MAZE_WIDTH
  {
    for y in 0..MAZE_HEIGHT
    {
      // Cell
      draw_rectangle(
        (LEFT_MAZE_OFFSET + (x * (CELL_WIDTH + WALL_WIDTH))) as f32,
        (TOP_MAZE_OFFSET + (y * (CELL_WIDTH + WALL_WIDTH))) as f32,
        CELL_WIDTH as f32,
        CELL_WIDTH as f32,
        if maze.get_direction(x, y) == Direction::None
        { Color::from_hex(UNVISITED_CELL_COLOR) }
        else
        { Color::from_hex(CELL_COLOR) }
      );
      // Lower wall
      draw_rectangle(
        (LEFT_MAZE_OFFSET + (x * (CELL_WIDTH + WALL_WIDTH))) as f32,
        (TOP_MAZE_OFFSET + ((y * (CELL_WIDTH + WALL_WIDTH)) + CELL_WIDTH)) as f32,
        CELL_WIDTH as f32,
        WALL_WIDTH as f32,
        Color::from_hex(WALL_COLOR)
      );
      // Right wall
      draw_rectangle(
        (LEFT_MAZE_OFFSET + ((x * (CELL_WIDTH + WALL_WIDTH)) + CELL_WIDTH)) as f32,
        (TOP_MAZE_OFFSET + (y * (CELL_WIDTH + WALL_WIDTH))) as f32,
        WALL_WIDTH as f32,
        CELL_WIDTH as f32,
        Color::from_hex(WALL_COLOR)
      );
      // Wall intersection in the lower right
      draw_rectangle(
        (LEFT_MAZE_OFFSET + ((x * (CELL_WIDTH + WALL_WIDTH)) + CELL_WIDTH)) as f32,
        (TOP_MAZE_OFFSET + ((y * (CELL_WIDTH + WALL_WIDTH)) + CELL_WIDTH)) as f32,
        WALL_WIDTH as f32,
        WALL_WIDTH as f32,
        Color::from_hex(WALL_COLOR)
      );
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Direction
{
  None, Up, Left, Down, Right, Terminal
}

pub(crate) struct Maze
{
  maze: [Direction; 1768]
}

impl Maze
{
  pub(crate) fn get_direction(&self, x: u32, y: u32) -> Direction
  {
    return self.maze[(y as usize * MAZE_WIDTH as usize) + x as usize];
  }

  pub(crate) fn visit_and_set(&mut self, x: usize, y: usize, direction: Direction)
  {
    self.maze[(y * MAZE_WIDTH as usize) + x] = direction;
  }

  pub(crate) fn step_dfs(&mut self, stack: &mut Vec<(u32, u32)>)
  {
  }

  pub(crate) fn step_kruskal(&mut self)
  {
  }

  pub(crate) fn step_prim(&mut self)
  {
  }

  pub(crate) fn step_wilson(&mut self)
  {
  }

  pub(crate) fn create_dfs(&mut self)
  {
    let mut stack: Vec<(u32, u32)> = vec![];
  }

  pub(crate) fn create_kruskal(&mut self)
  {
  }

  pub(crate) fn create_prim(&mut self)
  {
  }

  pub(crate) fn create_wilson(&mut self)
  {
  }

  pub(crate) fn new() -> Self
  {
    Maze{ maze: [
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
      Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None, Direction::None,
    ] }
  }
}

