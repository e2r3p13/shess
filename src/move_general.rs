/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   move_general.rs                                    :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lfalkau <lfalkau@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2020/02/27 22:37:02 by lfalkau           #+#    #+#             */
/*   Updated: 2020/03/05 03:41:30 by lfalkau          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::board::{Board, Box, Player};

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
pub struct Move
{
	pub from: Box,
	pub to: Box,
}

pub fn is_legal_move_for(player: Player, mv: Move, board: &Board) -> bool {
	//Make sure the wanted move is legal
	let legal_moves = get_legal_moves_for(player, board);
	for legal_move in legal_moves.iter() {
		if mv == *legal_move {
			return true;
		}
	}
	return false;
}

pub fn get_legal_moves_for(player: Player, board: &Board) -> Vec<Move> {
	let mut owned_boxes: Vec<Box> = Vec::new();
	let mut legal_moves: Vec<Move> = Vec::new();
	//Get position of all player's pieces
	for x in 0..8 {
		for y in 0..8 {
			if board.color_at(x, y) == player {
				owned_boxes.push(Box {x, y});
			}
		}
	}
	//Get legal_moves for each player's pieces

	//Remove moves that leads to self check

	return legal_moves;
}
