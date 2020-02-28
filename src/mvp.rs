/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   mvp.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lfalkau <lfalkau@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2020/02/28 08:46:17 by lfalkau           #+#    #+#             */
/*   Updated: 2020/02/28 12:48:49 by lfalkau          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::mvg;

pub enum Piece
{
	Pawn, Rock, Knight, Bishop, Queen, King
}

fn piece_between(m: &mvg::Move, b: &[[char; 8]; 8]) -> bool
{
	let mut s;
	let mut g;

	if m.to[0] == m.from[0] // Straight horizontal
	{
		s = if m.to[1] > m.from[1] {m.from[1]} else {m.to[1]};
		g = if m.to[1] > m.from[1] {m.to[1]} else {m.from[1]};
		for i in s..g
		{
			if b[m.from[0] as usize][i as usize] != '.' && i != s
			{
				return true;
			}
		}
	}
	if m.to[1] == m.from[1] // Straight vertical
	{
		s = if m.to[0] > m.from[0] {m.from[0]} else {m.to[0]};
		g = if m.to[0] > m.from[0] {m.to[0]} else {m.from[0]};
		for i in s..g
		{
			if b[i as usize][m.to[1] as usize] != '.' && i != m.from[0]
			{
				return true;
			}
		}
	}
	return false;
}

pub fn move_black_pawn(m: &mvg::Move, b: &mut [[char; 8]; 8]) -> bool
{
	if b[m.to[0] as usize][m.to[1] as usize] == '.' && m.to[1] == m.from[1] && (m.to[0] - m.from[0] == 1 || (m.to[0] - m.from[0] == 2 && m.from[0] == 1 )) // Move straight forward by 1
	{
		b[m.from[0] as usize][m.from[1] as usize] = '.';
		b[m.to[0] as usize][m.to[1] as usize] = 'P';
		return true;
	}
	if b[m.to[0] as usize][m.to[1] as usize].is_lowercase() && m.to[0] - m.from[0] == 1 && (m.to[1] - m.from[1] == 1 || m.to[1] - m.from[1] == -1) // Eat
	{
		b[m.from[0] as usize][m.from[1] as usize] = '.';
		b[m.to[0] as usize][m.to[1] as usize] = 'P';
		return true;
	}
	false
}

pub fn move_black_rock(m: &mvg::Move, b: &mut [[char; 8]; 8]) -> bool
{
	if (m.from[0] == m.to[0] || m.from[1] == m.to[1]) && !piece_between(m, b) && !b[m.to[0] as usize][m.to[1] as usize].is_uppercase()
	{
		b[m.from[0] as usize][m.from[1] as usize] = '.';
		b[m.to[0] as usize][m.to[1] as usize] = 'R';
		return true;
	}
	return false;
}

pub fn move_black_knight(m: &mvg::Move, b: &mut [[char; 8]; 8]) -> bool
{
	true
}

pub fn move_black_bishop(m: &mvg::Move, b: &mut [[char; 8]; 8]) -> bool
{
	true
}

pub fn move_black_queen(m: &mvg::Move, b: &mut [[char; 8]; 8]) -> bool
{
	true
}

pub fn move_black_king(m: &mvg::Move, b: &mut [[char; 8]; 8]) -> bool
{
	true
}

pub fn move_white_pawn(m: &mvg::Move, b: &mut [[char; 8]; 8]) -> bool
{
	if b[m.to[0] as usize][m.to[1] as usize] == '.' && m.to[1] == m.from[1] && (m.from[0] - m.to[0] == 1 || (m.from[0] - m.to[0] == 2 && m.from[0] == 6)) // Move straight forward by 1
	{
		b[m.from[0] as usize][m.from[1] as usize] = '.';
		b[m.to[0] as usize][m.to[1] as usize] = 'p';
		return true;
	}
	if b[m.to[0] as usize][m.to[1] as usize].is_uppercase() && m.from[0] - m.to[0] == 1 && (m.to[1] - m.from[1] == 1 || m.to[1] - m.from[1] == -1) // Eat
	{
		b[m.from[0] as usize][m.from[1] as usize] = '.';
		b[m.to[0] as usize][m.to[1] as usize] = 'p';
		return true;
	}
	false
}

pub fn move_white_rock(m: &mvg::Move, b: &mut [[char; 8]; 8]) -> bool
{
	if (m.from[0] == m.to[0] || m.from[1] == m.to[1]) && !piece_between(m, b) && !b[m.to[0] as usize][m.to[1] as usize].is_lowercase()
	{
		b[m.from[0] as usize][m.from[1] as usize] = '.';
		b[m.to[0] as usize][m.to[1] as usize] = 'r';
		return true;
	}
	return false;
}

pub fn move_white_knight(m: &mvg::Move, b: &mut [[char; 8]; 8]) -> bool
{
	true
}

pub fn move_white_bishop(m: &mvg::Move, b: &mut [[char; 8]; 8]) -> bool
{
	true
}

pub fn move_white_queen(m: &mvg::Move, b: &mut [[char; 8]; 8]) -> bool
{
	true
}

pub fn move_white_king(m: &mvg::Move, b: &mut [[char; 8]; 8]) -> bool
{
	true
}
