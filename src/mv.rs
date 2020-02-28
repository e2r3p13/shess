/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   mv.rs                                              :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lfalkau <lfalkau@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2020/02/27 22:37:02 by lfalkau           #+#    #+#             */
/*   Updated: 2020/02/28 08:42:53 by lfalkau          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::io;

pub struct Move
{
	from: [u8; 2],
	to: [u8; 2],
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

pub fn try_proceed(mv: &Move, board: &[[char; 8]; 8]) -> bool
{
	true
}

pub fn is_yours(m: &Move, board: &[[char; 8]; 8], turn: u8) -> bool
{
	if turn == 0 && board[m.from[0] as usize][m.from[1] as usize].is_lowercase()
	{
		return true;
	}
	if turn == 1 && board[m.from[0] as usize][m.from[1] as usize].is_uppercase()
	{
		return true;
	}
	return false;
}

fn input_to_pos(it: &str) -> Result<[u8; 2], io::Error>
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
	Ok(pos)
}
