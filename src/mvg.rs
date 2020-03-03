/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   mvg.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lfalkau <lfalkau@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2020/02/27 22:37:02 by lfalkau           #+#    #+#             */
/*   Updated: 2020/03/03 19:02:31 by lfalkau          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::io;
use colored::*;
use crate::mvp;
use crate::board;

#[derive(Copy, Clone)]
pub struct Move
{
	pub from: board::Box,
	pub to: board::Box,
}

pub fn parse(input: &str) -> Result<Move, io::Error>
{
	let input = input.to_lowercase();
	let f: Vec<&str> = input.split_whitespace().collect();

	if f.len() != 3
	{
		return Err(io::Error::new(io::ErrorKind::Other, "oh no!"));
	}
	let from = input_to_pos(f[0])?;
	let to = input_to_pos(f[2])?;
	return if f[1] == "to"
	{
		Ok(Move { from: from, to: to })
	}
	else if f[1] == "from"
	{
		Ok(Move { from: to, to: from })
	}
	else
	{
		Err(io::Error::new(io::ErrorKind::Other, "oh no!"))
	};
}

pub fn try_proceed(m: &Move, board: &mut board::Board) -> bool
{
	let src_color = board.color_at(m.from.x, m.from.y);
	let dst_color = board.color_at(m.to.x, m.to.y);

	if src_color == dst_color
	{
		println!("{}", format!("{}", "Suicide is unauthaurized.".bright_red()));
		return false;
	}
	let success = match board.at(m.from.x, m.from.y)
	{
		'P' | 'p' => mvp::move_pawn(src_color, m, board),
		'R' | 'r' => mvp::move_rock(m, board),
		'H' | 'h' => mvp::move_knight(m),
		'B' | 'b' => mvp::move_bishop(m, board),
		'Q' | 'q' => mvp::move_queen(m, board),
		'K' | 'k' => mvp::move_king(m),
		_ => panic!("Impossible statement")
	};
	if !success
	{
		println!("{}", format!("{}", "You can't do that.".bright_red()));
	}
	return success;
}

pub fn is_yours(m: &Move, board: &board::Board, turn: u8) -> bool
{
	if turn == 0 && board.color_at(m.from.x, m.from.y) == board::Color::White
	{
		return true;
	}
	if turn == 1 && board.color_at(m.from.x, m.from.y) == board::Color::Black
	{
		return true;
	}
	return false;
}

fn input_to_pos(it: &str) -> Result<board::Box, io::Error>
{
	let mut pos = [0, 0];

	if it.len() != 2
	{
		return Err(io::Error::new(io::ErrorKind::Other, "oh no!"));
	}
	let mut it = it.chars();
	for i in 0..2
	{
		pos[i] = match it.next().unwrap()
		{
			'a' | '8' => 0,
			'b' | '7' => 1,
			'c' | '6' => 2,
			'd' | '5' => 3,
			'e' | '4' => 4,
			'f' | '3' => 5,
			'g' | '2' => 6,
			'h' | '1' => 7,
			_ => { return  Err(io::Error::new(io::ErrorKind::Other, "oh no!")); }
		};
	}
	Ok(board::Box {x: pos[0], y: pos[1]})
}
