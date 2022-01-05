use std::fmt::{Display, Formatter, Result};
use std::io::{stdin, stdout, Write};
use std::process;

const WIDTH: usize = 80;
const HEIGHT: usize = 15;
const BLOCK: char = '*';
const SPACE: char = ' ';

type PlayingArea<const X: usize, const Y: usize> = [[char; X]; Y];

/// Generates the world's PlayingArea while giving a closure that decides what to set each cell char
/// The cell_generation has an argument of a tuple (x, y) that is the point that represents the cell
fn generate_world<F, const X: usize, const Y: usize>(cell_generation: F) -> PlayingArea<X, Y>
where
  F: Fn((usize, usize)) -> bool,
{
  let mut world = [[SPACE; X]; Y];

  for x in 0..X {
    for y in 0..Y {
      world[y][x] = if cell_generation((x, y)) {
        BLOCK
      } else {
        SPACE
      };
    }
  }

  world
}

/// Count the cells around origin cell and return if it should be alive or dead
fn count_neighbors<const X: usize, const Y: usize>(
  world: PlayingArea<X, Y>,
  x: usize,
  y: usize,
) -> u8 {
  let mut count = 0;

  for i in -1..=1 {
    for j in -1..=1 {
      let rel_x = x as isize + i;
      let rel_y = y as isize + j;

      // make sure the value can fit in the PlayingArea and is not the origin
      if (rel_x >= 0 && rel_x < X as isize)
        && (rel_y >= 0 && rel_y < Y as isize)
        && !(rel_x == x as isize && rel_y == y as isize)
        && world[rel_y as usize][rel_x as usize] == BLOCK
      {
        count += 1;
      }
    }
  }

  count
}

struct World {
  world: PlayingArea<WIDTH, HEIGHT>,
}

impl World {
  fn new() -> Self {
    Self {
      world: generate_world(|_| rand::random::<bool>()),
    }
  }

  /// Generate the next generation of the world
  fn next_generation(&mut self) {
    self.world = generate_world(|(x, y)| {
      let count = count_neighbors(self.world, x, y);

      // https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life#Rules
      if self.world[y][x] == BLOCK {
        count == 2 || count == 3
      } else {
        count == 3
      }
    });
  }
}

impl Display for World {
  fn fmt(&self, formater: &mut Formatter<'_>) -> Result {
    for i in self.world {
      for j in i {
        write!(formater, "{}", j)?;
      }
      writeln!(formater, "")?;
    }

    Ok(())
  }
}

fn main() {
  let mut world = World::new();
  print!("{}", world);

  loop {
    let mut string = String::new();
    print!("> ");
    stdout().flush().unwrap();
    stdin().read_line(&mut string).unwrap();
    let string = string.to_lowercase();
    let string = string.as_str().trim();

    match string {
      "q" => process::exit(0),
      _ => {
        world.next_generation();
        print!("{}", world);
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn world_generation() {
    let empty_world = [
      [SPACE, SPACE, SPACE, SPACE, SPACE],
      [SPACE, SPACE, SPACE, SPACE, SPACE],
      [SPACE, SPACE, SPACE, SPACE, SPACE],
      [SPACE, SPACE, SPACE, SPACE, SPACE],
      [SPACE, SPACE, SPACE, SPACE, SPACE],
    ];
    let filled_world = [
      [BLOCK, BLOCK, BLOCK, BLOCK, BLOCK],
      [BLOCK, BLOCK, BLOCK, BLOCK, BLOCK],
      [BLOCK, BLOCK, BLOCK, BLOCK, BLOCK],
      [BLOCK, BLOCK, BLOCK, BLOCK, BLOCK],
      [BLOCK, BLOCK, BLOCK, BLOCK, BLOCK],
    ];
    assert_eq!(generate_world::<_, 5, 5>(|_| false), empty_world);
    assert_eq!(generate_world::<_, 5, 5>(|_| true), filled_world);
  }

  #[test]
  fn number_of_neighbors() {
    let empty_world = generate_world::<_, WIDTH, HEIGHT>(|_| false);
    let filled_world = generate_world::<_, WIDTH, HEIGHT>(|_| true);

    assert_eq!(count_neighbors(empty_world, 0, 0), 0);
    assert_eq!(count_neighbors(empty_world, 1, 1), 0);
    assert_eq!(count_neighbors(empty_world, WIDTH - 1, HEIGHT - 1), 0);

    assert_eq!(count_neighbors(filled_world, 0, 0), 3);
    assert_eq!(count_neighbors(filled_world, WIDTH - 1, HEIGHT - 1), 3);
    assert_eq!(count_neighbors(filled_world, 1, 1), 8);
  }
}
