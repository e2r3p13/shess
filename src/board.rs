/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   board.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lfalkau <lfalkau@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2020/02/27 19:52:59 by lfalkau           #+#    #+#             */
/*   Updated: 2020/02/27 20:45:31 by lfalkau          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use colored::*;

pub fn set() -> [[char; 8]; 8]
{
	[
		['R', 'H', 'B', 'Q', 'K', 'B', 'H', 'R'],
		['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'],
		['.', '.', '.', '.', '.', '.', '.', '.'],
		['.', '.', '.', '.', '.', '.', '.', '.'],
		['.', '.', '.', '.', '.', '.', '.', '.'],
		['.', '.', '.', '.', '.', '.', '.', '.'],
		['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'],
		['r', 'h', 'b', 'q', 'k', 'b', 'h', 'r']
	]
}

pub fn print(board: [[char; 8]; 8])
{

	let mut c: char;
	let mut is_black: bool;

	println!("");
	for row in 0..9 {
		for column in 0..9 {
			if row == 0 && column == 0
			{
				print!(" # "); continue
			}
			if row == 0
			{
				print!(" {} ", column); continue
			}
			if column == 0
			{
				print!(" {} ", to_row_letter(row)); continue
			}
			is_black = board[row - 1][column - 1].is_uppercase();
			c = p_from_c(board[row - 1][column - 1]);
			if (row + column) % 2 == 0
			{
				if is_black
				{
					print!("{}", format!(" {} ", c).bright_blue().on_bright_yellow().bold());
				}
				else
				{
					print!("{}", format!(" {} ", c).black().on_bright_yellow());
				}
			} else {
				if is_black
				{
					print!("{}", format!(" {} ", c).bright_yellow().on_bright_blue().bold());
				}
				else
				{
					print!("{}", format!(" {} ", c).black().on_bright_blue());
				}
			}
		}
		println!("");
	}
	println!("");
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
