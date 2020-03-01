/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   mvg.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lfalkau <lfalkau@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2020/02/27 22:37:02 by lfalkau           #+#    #+#             */
/*   Updated: 2020/03/01 21:32:30 by lfalkau          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::io;
use crate::mvp;

pub struct Move
{
	pub from: Box,
	pub to: Box,
}

pub struct Box
{
	pub x: i8,
	pub y: i8,
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

pub fn try_proceed(m: &Move, board: &mut [[char; 8]; 8]) -> bool
{
	let color = board[m.from.y as usize][m.from.x as usize]is_uppercase();
	return match board[m.from.y as usize][m.from.x as usize]
	{
		'P' => mvp::move_black_pawn(m, board, color),
		'R' => mvp::move_black_rock(m, board, color),
		'H' => mvp::move_black_knight(m, board, color),
		'B' => mvp::move_black_bishop(m, board, color),
		'Q' => mvp::move_black_queen(m, board, color),
		'K' => mvp::move_black_king(m, board, color),
		_ => panic!("Impossible statement")
	};
}

pub fn is_yours(m: &Move, board: &[[char; 8]; 8], turn: u8) -> bool
{
	if turn == 0 && board[m.from.y as usize][m.from.x as usize].is_lowercase()
	{
		return true;
	}
	if turn == 1 && board[m.from.y as usize][m.from.x as usize].is_uppercase()
	{
		return true;
	}
	return false;
}

fn input_to_pos(it: &str) -> Result<Box, io::Error>
{
	let mut pos = [0, 0];

	if it.len() != 2
	{
		return Err(io::Error::new(io::ErrorKind::Other, "oh no!"));
	}
	let mut it = it.chars();
	pos[0] = match it.next().unwrap()
	{
		'a' => 0,
		'b' => 1,
		'c' => 2,
		'd' => 3,
		'e' => 4,
		'f' => 5,
		'g' => 6,
		'h' => 7,
		_ => { return  Err(io::Error::new(io::ErrorKind::Other, "oh no!")); }
	};
	pos[1] = match it.next().unwrap()
	{
		'1' => 0,
		'2' => 1,
		'3' => 2,
		'4' => 3,
		'5' => 4,
		'6' => 5,
		'7' => 6,
		'8' => 7,
		_ => { return  Err(io::Error::new(io::ErrorKind::Other, "oh no!")); }
	};
	Ok(Box {x: pos[1], y: pos[0]})
}
