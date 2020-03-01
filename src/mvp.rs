/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   mvp.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lfalkau <lfalkau@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2020/02/28 08:46:17 by lfalkau           #+#    #+#             */
/*   Updated: 2020/03/01 21:33:20 by lfalkau          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::mvg;

fn is_pawn_move(m: &mvg::Move) -> bool
{
	let is_black = b[m.from.y][m.from.x].is_uppercase();
	if is_black && (m.from.y - m.to.y == -1 || m.from.y - m.to.y == -2) && m.from.y == m.from.x && b[m.to.y as usize][m.to.x as usize] == '.'
	{
		return true;
	}
	if !is_black && (m.from.y - m.to.y == 1 || m.from.y - m.to.y == 2) && m.from.y == m.from.x && b[m.to.y as usize][m.to.x as usize] == '.'
	{
		return true;
	}
	return false;
}

fn can_pawn_eat(m: &mvg::Move) -> bool
{
	let self_color = b[m.from.y][m.from.x].is_uppercase();
	let target_color = b[m.to.y][m.to.x].is_uppercase();
	if self_color
}

fn is_rock_move(m: &mvg::Move) -> bool
{
	m.from.y == m.to.y || m.from.x == m.to.x
}

fn is_bishop_move(m: &mvg::Move) -> bool
{
	(m.from.y - m.to.y).abs() == (m.from.x - m.to.x).abs()
}

fn is_knight_move(m: &mvg::Move) -> bool
{
	((m.from.y - m.to.y).abs() == 2 && (m.from.x - m.to.x).abs() == 1) ||
	((m.from.y - m.to.y).abs() == 1 && (m.from.x - m.to.x).abs() == 2)
}

fn is_queen_move(m: &mvg::Move) -> bool
{

}

fn is_king_move(m: &mvg::Move) -> bool
{

}

fn end_conflict(m: &mvg::Move, b: &[[char; 8]; 8]) -> bool
{
	let color_fr = b[m.from.y][m.from.x].is_uppercase();
	let color_to = b[m.to.y][m.to.x].is_uppercase();
	return (color_fr == color_to);
}

fn collides(m: &mvg::Move, b: &[[char; 8]; 8]) -> bool
{
	let vv = {
		if (m.from.y - m.to.y > 0) { -1 }
		else if (m.from.y - m.to.y < 0) { 1 }
		else { 0 }
	};
	let vh = {
		if (m.from.x - m.to.x > 0) { -1 }
		else if (m.from.x - m.to.x < 0) { 1 }
		else { 0 }
	};
	let mut fy = m.from.y + vv;
	let mut fx = m.from.x + vh;
	while m.to != [fy, fx]
	{
		if b[fy][fx] != '.'
		{
			return true;
		}
		fy += vv;
		fx += vh;
	}
	return false;
}

pub fn move_pawn(m: &mvg::Move, b: &mut [[char; 8]; 8]) -> bool
{
	if b[m.to.y][m.to.x] == '.' && m.to.x == m.from.x && (m.to.y - m.from.y == 1 || (m.to.y - m.from.y == 2 && m.from.y == 1 )) // Move straight forward by 1
	{
		b[m.from.y][m.from.x] = '.';
		b[m.to.y][m.to.x] = 'P';
		return true;
	}
	if b[m.to.y][m.to.x].is_lowercase() && m.to.y - m.from.y == 1 && (m.to.x - m.from.x == 1 || m.to.x - m.from.x == -1) // Eat
	{
		b[m.from.y][m.from.x] = '.';
		b[m.to.y][m.to.x] = 'P';
		return true;
	}
	false
}

pub fn move_rock(m: &mvg::Move, b: &mut [[char; 8]; 8]) -> bool
{
	if is_rock_move(m) && !collides(m, b) && !end_conflict(m, b)
	{
		b[m.from.y][m.from.x] = '.';
		b[m.to.y][m.to.x] = 'R';
		return true;
	}
	return false;
}

pub fn move_knight(m: &mvg::Move, b: &mut [[char; 8]; 8]) -> bool
{
	if is_knight_move(m) && !end_conflict(m, b)
	{
		b[m.from.y][m.from.x] = '.';
		b[m.to.y][m.to.x] = 'H';
		return true;
	}
	return false;
}

pub fn move_bishop(m: &mvg::Move, b: &mut [[char; 8]; 8]) -> bool
{
	if is_bishop_move(m) && !collides(m, b) && !end_conflict(m, b)
	{
		b[m.from.y][m.from.x] = '.';
		b[m.to.y][m.to.x] = 'B';
		return true;
	}
	return false;
}

pub fn move_queen(m: &mvg::Move, b: &mut [[char; 8]; 8]) -> bool
{
	true
}

pub fn move_king(m: &mvg::Move, b: &mut [[char; 8]; 8]) -> bool
{
	true
}
