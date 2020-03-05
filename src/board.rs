/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   board.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lfalkau <lfalkau@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2020/02/27 19:52:59 by lfalkau           #+#    #+#             */
/*   Updated: 2020/03/05 02:09:31 by lfalkau          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use colored::*;
use crate::mvg;

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

pub struct Board
{
	pub raw: [[char; 8]; 8],
}

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
pub struct Box
{
	pub x: i8,
	pub y: i8,
}

#[derive(PartialEq, Eq)]
#[derive(Copy, Clone)]
pub enum Color
{
	Black, White, None
}

impl Board
{
	pub fn print(&self)
	{

		let mut c: char;
		let mut is_black: bool;

		println!("");
		for row in 0..9
		{
			for column in 0..9
			{
				if row == 8 && column == 0
				{
					print!("   "); continue
				}
				if row == 8
				{
					print!(" {} ", to_row_letter(column)); continue
				}
				if column == 0
				{
					print!(" {} ", ((8 - row) as i32).abs()); continue
				}
				is_black = self.raw[row][column - 1].is_uppercase();
				c = p_from_c(self.raw[row][column - 1]);
				if (row + column) % 2 == 0
				{
					if is_black
					{
						print!("{}", format!(" {} ", c).black().on_bright_yellow());
					}
					else
					{
						print!("{}", format!(" {} ", c).bright_blue().on_bright_yellow().bold());
					}
				}
				else
				{
					if is_black
					{
						print!("{}", format!(" {} ", c).black().on_bright_blue());
					}
					else
					{
						print!("{}", format!(" {} ", c).bright_yellow().on_bright_blue().bold());
					}
				}
			}
			println!("");
		}
		println!("");
	}

	pub fn get_pos(&self, piece: char) -> Box
	{
		for i in 0..8
		{
			for j in 0..8
			{
				if self.at(i, j) == piece
				{
					return Box { x: i, y: j };
				}
			}
		}
		return Box { x: 0, y: 0 };
	}

	pub fn at(&self, x: i8, y: i8) -> char
	{
		self.raw[y as usize][x as usize]
	}

	pub fn color_at(&self, x: i8, y: i8) -> Color
	{
		let piece = self.at(x, y);
		if piece == '.'
		{
			return Color::None;
		}
		else if piece.is_uppercase()
		{
			return Color::Black;
		}
		else
		{
			return Color::White;
		}
	}

	pub fn perform_move(&mut self, m: mvg::Move) -> char
	{
		let last_ate = self.raw[m.to.y as usize][m.to.x as usize];
		self.raw[m.to.y as usize][m.to.x as usize] = self.raw[m.from.y as usize][m.from.x as usize];
		self.raw[m.from.y as usize][m.from.x as usize] = '.';
		return last_ate;
	}

	pub fn cancel_move(&mut self, m: mvg::Move, last_ate: char)
	{
		self.raw[m.from.y as usize][m.from.x as usize] = self.raw[m.to.y as usize][m.to.x as usize];
		self.raw[m.to.y as usize][m.to.x as usize] = last_ate;
	}
}


fn to_row_letter(id: usize) -> char
{
	match id {
		1 => 'A',
		2 => 'B',
		3 => 'C',
		4 => 'D',
		5 => 'E',
		6 => 'F',
		7 => 'G',
		8 => 'H',
		_ => ' ',
	}
}

fn p_from_c(c: char) -> char
{

	let c = c.to_lowercase().next().unwrap();

	match c
	{
		'p' => '♙',
		'r' => '♖',
		'h' => '♘',
		'b' => '♗',
		'q' => '♕',
		'k' => '♔',
		_ => ' '
	}
}
