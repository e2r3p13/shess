/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   move_general.rs                                    :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lfalkau <lfalkau@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2020/02/27 22:37:02 by lfalkau           #+#    #+#             */
/*   Updated: 2020/03/06 12:55:04 by lfalkau          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::board::{Board, Box, Player};
use crate::move_pieces::{
	pawn_legal_moves_for,
	rock_legal_moves_for,
	horse_legal_moves_for,
	bishop_legal_moves_for,
	queen_legal_moves_for,
	king_legal_moves_for
};

#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
#[derive(Debug)]
pub struct Move {
	pub from: Box, pub to: Box,
}

pub fn is_legal_move_for(player: Player, mv: Move, board: &Board) -> bool {
	//Make sure the wanted move is legal
	let legal_moves = get_legal_moves_for(player, board, true);
	for legal_move in legal_moves.iter() {
		if mv == *legal_move {
			return true;
		}
	}
	return false;
}

pub fn get_legal_moves_for(player: Player, board: &Board, check: bool) -> Vec<Move> {
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
	for bx in owned_boxes.iter() {
		match board.at(bx.x, bx.y) {
			'P' | 'p' => legal_moves.append(&mut pawn_legal_moves_for(player, board, *bx)),
			'R' | 'r' => legal_moves.append(&mut rock_legal_moves_for(player, board, *bx)),
			'H' | 'h' => legal_moves.append(&mut horse_legal_moves_for(player, board, *bx)),
			'B' | 'b' => legal_moves.append(&mut bishop_legal_moves_for(player, board, *bx)),
			'Q' | 'q' => legal_moves.append(&mut queen_legal_moves_for(player, board, *bx)),
			'K' | 'k' => legal_moves.append(&mut king_legal_moves_for(player, board, *bx)),
			_ => continue,
		}
	}

	//Check bool indicates weither we have to avoid moves that leads us to check or not
	//Add id of checks leading moves in a vector
	if check {
		let mut check_id: Vec<usize> = Vec::new();
		for i in 0..legal_moves.len() {
			let mut cpy_board = board.clone();
			cpy_board.perform_move(legal_moves[i]);
			if check_for(player, &cpy_board) {
				check_id.push(i);
			}
		}
		//Remove those elements from legal moves
		while let Some(id) = check_id.pop() {
			legal_moves.remove(id);
		}
	}
	return legal_moves;
}

//Return true if a specific player is checked for a specific board
pub fn check_for(player: Player, board: &Board) -> bool {
	let opponent = if player == Player::Black { Player::White } else { Player::Black };
	let opponent_moves = get_legal_moves_for(opponent, board, false);
	let king_position = board.get_king_pos_for(player);
	for mv in opponent_moves {
		if mv.to == king_position {
			return true;
		}
	}
	return false;
}
