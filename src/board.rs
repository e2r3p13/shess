/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   board.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lfalkau <lfalkau@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2020/02/27 19:52:59 by lfalkau           #+#    #+#             */
/*   Updated: 2020/03/10 15:32:53 by lfalkau          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use colored::*;
use crate::move_general::{Move};

pub const DEFAULT_BOARD: [[char; 8]; 8] = [
	['R', 'H', 'B', 'Q', 'K', 'B', 'H', 'R'],
	['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'],
	['.', '.', '.', '.', '.', '.', '.', '.'],
	['.', '.', '.', '.', '.', '.', '.', '.'],
	['.', '.', '.', '.', '.', '.', '.', '.'],
	['.', '.', '.', '.', '.', '.', '.', '.'],
	['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'],
	['r', 'h', 'b', 'q', 'k', 'b', 'h', 'r']
];

#[derive(Copy, Clone)]
pub struct Board {
	pub raw: [[char; 8]; 8],
	pub nb_black_eaten: usize,
	pub black_eaten: [char; 15],
	pub nb_white_eaten: usize,
	pub white_eaten: [char; 15],
}

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
#[derive(Debug)]
pub struct Box {
	pub x: i8, pub y: i8,
}

#[derive(PartialEq, Eq)]
#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum Player {
	Black, White, None
}

impl Player {

	pub fn opponent(&self) -> Player {
		match self {
			Player::Black => Player::White,
			Player::White => Player::Black,
			Player::None => Player::None,
		}
	}

}

impl Board {

	pub fn print(&self) {
		let mut c: char;
		let mut is_black: bool;

		std::process::Command::new("clear").status().unwrap();
		println!("\n       {}", format!("{}", "S  H  E  S  S  !".bright_magenta().bold()));
		println!("");
		for row in 0..10 {
			for column in 0..10 {
				if (row == 9 && column == 0) || (row == 0 && column == 9) || (row == 0 && column == 0) || (row == 9 && column == 9) {
					print!("   "); continue
				}
				if row == 0 || row == 9 {
					print!(" {} ", to_row_letter(column)); continue
				}
				if column == 0 || column == 9 {
					print!(" {} ", ((9 - row) as i32).abs()); continue
				}
				is_black = self.raw[row - 1][column - 1].is_uppercase();
				c = p_from_c(self.raw[row - 1][column - 1]);
				if (row + column) % 2 == 1 {
					if is_black {
						print!("{}", format!(" {} ", c).black().on_bright_white());
					} else {
						print!("{}", format!(" {} ", c).white().on_bright_white());
					}
				} else {
					if is_black {
						print!("{}", format!(" {} ", c).black().on_bright_black());
					} else {
						print!("{}", format!(" {} ", c).bright_white().on_bright_black());
					}
				}
			}
			if row == 4 {
				print!("  ");
				self.print_eaten(Player::Black);
			} else if row == 5 {
				print!("  ");
				self.print_eaten(Player::White)
			}
			println!("");
		}
		println!("");
	}

	pub fn print_eaten(&self, player: Player) {
		if player == Player::Black {
			for i in 0..15 {
				print!("{}", format!("{} ", p_from_c(self.white_eaten[i])).white().on_bright_black());
			}
			print!(" {:+}  ", self.get_score_for(Player::Black));
		} else {
			for i in 0..15 {
				print!("{}", format!("{} ", p_from_c(self.black_eaten[i])).black().on_bright_white());
			}
			print!(" {:+}  ", self.get_score_for(Player::White));
		}
	}

	pub fn get_king_pos_for(&self, player: Player) -> Box {
		let king = match player {
			Player::White => 'k',
			Player::Black => 'K',
			_ => ' ',
		};
		for i in 0..8 {
			for j in 0..8 {
				if self.at(i, j) == king {
					return Box {x: i, y: j};
				}
			}
		}
		return Box { x: 0, y: 0 };
	}

	pub fn at(&self, x: i8, y: i8) -> char {
		if x >= 0 && y >= 0 && x <8 && y < 8 {
			return self.raw[y as usize][x as usize];
		} else {
			return '#';
		}
	}

	pub fn color_at(&self, x: i8, y: i8) -> Player {
		let piece = self.at(x, y);
		if piece.is_lowercase() {
			return Player::White;
		} else if piece.is_uppercase() {
			return Player::Black;
		} else {
			return Player::None;
		}
	}

	pub fn perform_move(&mut self, m: Move) {
		//Check for eaten pieces
		if self.color_at(m.to.x, m.to.y) == Player::Black {
			self.black_eaten[self.nb_black_eaten] = self.at(m.to.x, m.to.y);
			self.nb_black_eaten += 1;
		}
		if self.color_at(m.to.x, m.to.y) == Player::White {
			self.white_eaten[self.nb_white_eaten] = self.at(m.to.x, m.to.y);
			self.nb_white_eaten += 1;
		}
		//Perform move
		self.raw[m.to.y as usize][m.to.x as usize] = self.raw[m.from.y as usize][m.from.x as usize];
		self.raw[m.from.y as usize][m.from.x as usize] = '.';
		//Check for upgrading pawn
		if m.to.y == 0 &&  self.at(m.to.x, m.to.y) == 'p'{
			self.raw[m.to.y as usize][m.to.x as usize] = 'q';
		}
		if m.to.y == 7 &&  self.at(m.to.x, m.to.y) == 'P'{
			self.raw[m.to.y as usize][m.to.x as usize] = 'Q';
		}
	}

	pub fn get_score_for(&self, p: Player) -> i32 {
		let mut score = 0;
		//println!("{:?}", player);
		for x in 0..8 {
			for y in 0..8 {
				let value = match self.at(x, y) {
					'P' | 'p' => 100, 'R' | 'r' => 500, 'H' | 'h' => 320,
					'B' | 'b' => 330, 'Q' | 'q' => 900, 'K' | 'k' => 20_000,
					_ => 0,
				};
				if p == self.color_at(x, y) {
					score += value;
				} else if self.color_at(x, y) == p.opponent() {
					score -= value;
				}
				//print!("{}, ", score);
			}
		}
		//println!("");
		return score;
	}

}

fn to_row_letter(id: usize) -> char {
	match id {
		1 => 'A', 2 => 'B', 3 => 'C',
		4 => 'D', 5 => 'E', 6 => 'F',
		7 => 'G', 8 => 'H', _ => ' ',
	}
}

fn p_from_c(c: char) -> char {
	match c.to_lowercase().next().unwrap() {
		'p' => '♙', 'r' => '♖', 'h' => '♘',
		'b' => '♗', 'q' => '♕', 'k' => '♔',
		_ => ' '
	}
}
