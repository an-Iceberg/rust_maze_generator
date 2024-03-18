use macroquad::{window::clear_background, color::{Color, RED, LIME}, shapes::{draw_rectangle, draw_line}};
use rand::{rngs::ThreadRng, Rng};

use crate::{MAZE_WIDTH, BG_COLOR, LEFT_MAZE_OFFSET, WALL_WIDTH, TOP_MAZE_OFFSET, CELL_WIDTH, WALL_COLOR, MAZE_HEIGHT, UNVISITED_CELL_COLOR, CELL_COLOR};

pub(crate) fn paint(
  maze: &mut Maze,
  stack: &mut Vec<(usize, usize)>,
  generating: &bool
)
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

  // TODO: draw the walls according to the cell direction
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

      // Only draw top of stack when generating maze
      if *generating
      {
        if let Some(top_of_stack) = stack.last()
        {
          draw_rectangle(
            (LEFT_MAZE_OFFSET + (top_of_stack.0 as u32 * (CELL_WIDTH + WALL_WIDTH))) as f32,
            (TOP_MAZE_OFFSET + (top_of_stack.1 as u32 * (CELL_WIDTH + WALL_WIDTH))) as f32,
            CELL_WIDTH as f32,
            CELL_WIDTH as f32,
            LIME
          );
        }
      }
      // Wall intersection in the lower right
      draw_rectangle(
        (LEFT_MAZE_OFFSET + ((x * (CELL_WIDTH + WALL_WIDTH)) + CELL_WIDTH)) as f32,
        (TOP_MAZE_OFFSET + ((y * (CELL_WIDTH + WALL_WIDTH)) + CELL_WIDTH)) as f32,
        WALL_WIDTH as f32,
        WALL_WIDTH as f32,
        Color::from_hex(WALL_COLOR)
      );

      // Lower wall
      draw_rectangle(
        (LEFT_MAZE_OFFSET + (x * (CELL_WIDTH + WALL_WIDTH))) as f32,
        (TOP_MAZE_OFFSET + ((y * (CELL_WIDTH + WALL_WIDTH)) + CELL_WIDTH)) as f32,
        CELL_WIDTH as f32,
        WALL_WIDTH as f32,
        if maze.get_direction(x, y) == Direction::Down
        { Color::from_hex(CELL_COLOR) }
        else
        { Color::from_hex(WALL_COLOR) }
      );
      // Right wall
      draw_rectangle(
        (LEFT_MAZE_OFFSET + ((x * (CELL_WIDTH + WALL_WIDTH)) + CELL_WIDTH)) as f32,
        (TOP_MAZE_OFFSET + (y * (CELL_WIDTH + WALL_WIDTH))) as f32,
        WALL_WIDTH as f32,
        CELL_WIDTH as f32,
        if maze.get_direction(x, y) == Direction::Right
        { Color::from_hex(CELL_COLOR) }
        else
        { Color::from_hex(WALL_COLOR) }
      );

      // Top wall
      if maze.get_direction(x, y) == Direction::Up
      {
        draw_rectangle(
          (LEFT_MAZE_OFFSET + (x * (CELL_WIDTH + WALL_WIDTH))) as f32,
          (TOP_MAZE_OFFSET + (((y-1) * (CELL_WIDTH + WALL_WIDTH)) + CELL_WIDTH)) as f32,
          CELL_WIDTH as f32,
          WALL_WIDTH as f32,
          Color::from_hex(CELL_COLOR)
        );
      }

      // Left wall
      if maze.get_direction(x, y) == Direction::Left
      {
        draw_rectangle(
          (LEFT_MAZE_OFFSET + (((x-1) * (CELL_WIDTH + WALL_WIDTH)) + CELL_WIDTH)) as f32,
          (TOP_MAZE_OFFSET + (y * (CELL_WIDTH + WALL_WIDTH))) as f32,
          WALL_WIDTH as f32,
          CELL_WIDTH as f32,
          Color::from_hex(CELL_COLOR)
        );
      }
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Direction
{
  None, Up, Left, Down, Right
}

impl Direction
{
  pub(crate) fn inverse(&self) -> Self
  {
    match *self
    {
      Direction::Down  => Direction::Up,
      Direction::Up    => Direction::Down,
      Direction::Left  => Direction::Right,
      Direction::Right => Direction::Left,
      Direction::None  => Direction::None
    }
  }
}

pub(crate) struct Maze
{
  maze: [Direction; 1768],
}

impl Maze
{
  pub(crate) fn clear(&mut self)
  {
    self.maze = Maze::new().maze;
  }

  pub(crate) fn get_direction(&self, x: u32, y: u32) -> Direction
  {
    return self.maze[(y as usize * MAZE_WIDTH as usize) + x as usize];
  }

  pub(crate) fn visit_and_set(&mut self, x: usize, y: usize, direction: Direction)
  {
    self.maze[(y * MAZE_WIDTH as usize) + x] = direction;
  }

  pub(crate) fn get_neighbour_coords(&self, x: usize, y: usize, direction: Direction) -> Option<(usize, usize)>
  {
    match direction
    {
      Direction::Up =>
      {
        if y <= 0
        { return None; }
        return Some((x, y - 1));
      }
      Direction::Down =>
      {
        if y >= (MAZE_HEIGHT - 1) as usize
        { return None; }
        return Some((x, y + 1));
      }
      Direction::Left =>
      {
        if x <= 0
        { return None; }
        return Some((x - 1, y));
      }
      Direction::Right =>
      {
        if x >= (MAZE_WIDTH - 1) as usize
        { return None; }
        return Some((x + 1, y));
      }
      // Direction::Terminal => return Some((x,y)),
      Direction::None => return None
    }
  }

  // TODO: refactor this, maybe?
  fn unvisited_neighbours(&self, x: usize, y: usize) -> Vec<((usize, usize), Direction)>
  {
    let mut neighbours: Vec<((usize, usize), Direction)> = vec![];

    let up_neighbour = self.get_neighbour_coords(x, y, Direction::Up);
    let down_neighbour = self.get_neighbour_coords(x, y, Direction::Down);
    let left_neighbour = self.get_neighbour_coords(x, y, Direction::Left);
    let right_neighbour = self.get_neighbour_coords(x, y, Direction::Right);

    if let Some(up) = up_neighbour
    {
      if self.get_direction(up.0 as u32, up.1 as u32) == Direction::None
      {
        neighbours.push(((up), Direction::Up));
      }
    }

    if let Some(down) = down_neighbour
    {
      if self.get_direction(down.0 as u32, down.1 as u32) == Direction::None
      {
        neighbours.push(((down), Direction::Down));
      }
    }

    if let Some(left) = left_neighbour
    {
      if self.get_direction(left.0 as u32, left.1 as u32) == Direction::None
      {
        neighbours.push(((left), Direction::Left));
      }
    }

    if let Some(right) = right_neighbour
    {
      if self.get_direction(right.0 as u32, right.1 as u32) == Direction::None
      {
        neighbours.push(((right), Direction::Right));
      }
    }

    return neighbours;
  }

  pub(crate) fn step_dfs(&mut self, stack: &mut Vec<(usize, usize)>, rng: &mut ThreadRng) -> bool
  {
    // Pop cell on top of stack
    let current_cell = stack.pop().unwrap();

    // Determine, which directions can be chosen from
    let neighbours = self.unvisited_neighbours(current_cell.0, current_cell.1);

    // Backtracking if there are no unvisited neighbours
    if neighbours.is_empty()
    {
      // self.visit_and_set(current_cell.0, current_cell.1, Direction::Terminal);
      return !stack.is_empty();
    }

    // Choose a random valid neighbour
    let neighbour = rng.gen_range(0..neighbours.len());

    // Set the chose neighbour to point to the current cell
    self.visit_and_set(
      neighbours.get(neighbour).unwrap().0.0,
      neighbours.get(neighbour).unwrap().0.1,
      neighbours.get(neighbour).unwrap().1.inverse()
    );

    // Push all other neighbours onto stack
    neighbours.iter().enumerate()
      // Skipping the randomly chosen neighbour
      .filter(|(index, _)| *index == neighbour)
      .for_each(|(_, (coordinates, _))|
      {
        stack.push(*coordinates);
      });

    // Push chosen direction onto stack last
    stack.push(neighbours.get(neighbour).unwrap().0);

    // Generate the maze while the stack is not empty
    return !stack.is_empty();
  }

  pub(crate) fn step_kruskal(&mut self) -> bool
  {
    todo!()
  }

  pub(crate) fn step_prim(&mut self) -> bool
  {
    todo!()
  }

  pub(crate) fn step_wilson(&mut self) -> bool
  {
    todo!()
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

