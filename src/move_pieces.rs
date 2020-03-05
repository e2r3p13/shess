/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   move_pieces.rs                                     :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lfalkau <lfalkau@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2020/02/28 08:46:17 by lfalkau           #+#    #+#             */
/*   Updated: 2020/03/05 03:22:32 by lfalkau          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::move_general::Move;
use crate::board::{Board, Player};

fn is_pawn_move(m: &Move, color: Player) -> bool
{
	if color == Player::Black && (m.from.y - m.to.y == -1 || (m.from.y - m.to.y == -2 && m.from.y == 1)) && m.from.x == m.to.x
	{
		return true;
	}
	if color == Player::White && (m.from.y - m.to.y == 1 || (m.from.y - m.to.y == 2 && m.from.y == 6)) && m.from.x == m.to.x
	{
		return true;
	}
	return false;
}

fn can_pawn_eat(m: &Move, color: Player) -> bool
{
	if color == Player::Black && m.to.y - m.from.y == 1 && (m.to.x - m.from.x).abs() == 1
	{
		return true;
	}
	else if color == Player::White && m.from.y - m.to.y == 1 && (m.to.x - m.from.x).abs() == 1
	{
		return true;
	}
	return false;
}

fn is_rock_move(m: &Move) -> bool
{
	m.from.y == m.to.y || m.from.x == m.to.x
}

fn is_bishop_move(m: &Move) -> bool
{
	(m.from.y - m.to.y).abs() == (m.from.x - m.to.x).abs()
}

fn is_knight_move(m: &Move) -> bool
{
	((m.from.y - m.to.y).abs() == 2 && (m.from.x - m.to.x).abs() == 1) ||
	((m.from.y - m.to.y).abs() == 1 && (m.from.x - m.to.x).abs() == 2)
}

fn is_queen_move(m: &Move) -> bool
{
	is_bishop_move(m) || is_rock_move(m)
}

fn is_king_move(m: &Move) -> bool
{
	(m.from.x - m.to.x).abs() <= 1 && (m.from.y - m.to.y).abs() <= 1
}

fn collides(m: &Move, b: &Board) -> bool
{
	let vv = {
		if m.from.y - m.to.y > 0 { -1 }
		else if m.from.y - m.to.y < 0 { 1 }
		else { 0 }
	};
	let vh = {
		if m.from.x - m.to.x > 0 { -1 }
		else if m.from.x - m.to.x < 0 { 1 }
		else { 0 }
	};
	let mut fy = m.from.y + vv;
	let mut fx = m.from.x + vh;
	while m.to.x != fx || m.to.y != fy
	{
		if b.color_at(fx, fy) != Player::None
		{
			return true;
		}
		fy += vv;
		fx += vh;
	}
	return false;
}

pub fn move_pawn(color: Player, m: &Move, b: &mut Board) -> bool
{
	if (is_pawn_move(m, color) && !collides(m, b) && b.color_at(m.to.x, m.to.y) == Player::None) ||
	(can_pawn_eat(m, color) && b.color_at(m.to.x, m.to.y) != Player::None)
	{
		return true;
	}
	return false;
}

pub fn move_rock(m: &Move, b: &mut Board) -> bool
{
	if is_rock_move(m) && !collides(m, b)
	{
		return true;
	}
	return false;
}

pub fn move_knight(m: &Move) -> bool
{
	if is_knight_move(m)
	{
		return true;
	}
	return false;
}

pub fn move_bishop(m: &Move, b: &mut Board) -> bool
{
	if is_bishop_move(m) && !collides(m, b)
	{
		return true;
	}
	return false;
}

pub fn move_queen(m: &Move, b: &mut Board) -> bool
{
	if is_queen_move(m) && !collides(m, b)
	{
		return true;
	}
	return false;
}

pub fn move_king(m: &Move) -> bool
{
	if is_king_move(m)
	{
		return true;
	}
	return false;
}
