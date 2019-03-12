/* 
    main.rs -> the entry point
    Copyright (C) 2019 - scarletkrath

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
use std::error::Error;

use game::Game;

mod game;
mod utils;
mod resources;

pub fn main() -> Result<(), Box<dyn Error>> {
    println!();

    let mut game = Game::new("I Wanna Be The Suleyth", 768, 432, 60)?;
    
    game.run()?;

    Ok(())
}