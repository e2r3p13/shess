/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   mv.rs                                              :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lfalkau <lfalkau@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2020/02/27 22:37:02 by lfalkau           #+#    #+#             */
/*   Updated: 2020/02/27 22:42:11 by lfalkau          ###   ########.fr       */
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
	let from;
	let to;
	let word;
	let fields: Vec<&str> = input.split_whitespace().collect();

	if fields.len() != 3
	{
		return Err(io::Error::new(io::ErrorKind::Other, "oh no!"));
	}
	from = input_to_pos(fields[0])?;
	word = fields[1];
	to = input_to_pos(fields[2])?;
	return if word == "to" {
		Ok(Move { from, to })
	} else if word == "from" {
		Ok(Move { to, from })
	} else {
		Err(io::Error::new(io::ErrorKind::Other, "oh no!"))
	};
}

pub fn try_proceed(mv: Move, board: &[[char; 8]; 8]) -> bool
{
	true
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
