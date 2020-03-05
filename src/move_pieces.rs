/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   move_pieces.rs                                     :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lfalkau <lfalkau@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2020/02/28 08:46:17 by lfalkau           #+#    #+#             */
/*   Updated: 2020/03/05 19:35:07 by lfalkau          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::board::{Board, Box, Player};
use crate::move_general::{Move};

pub fn pawn_legal_moves_for(player: Player, board: &Board, bx: Box) -> Vec<Move> {
	let mut legal_moves: Vec<Move> = Vec::new();
	if player == Player::Black {
		//Go straight by one
		if board.at(bx.x, bx.y + 1) == '.' {
			legal_moves.push(Move {from: bx, to: Box{x: bx.x, y: bx.y + 1}});
			//Go straight by two
			if board.at(bx.x, bx.y + 2) == '.' && bx.y == 1 {
				legal_moves.push(Move {from: bx, to: Box{x: bx.x, y: bx.y + 2}});
			}
		}
		//Eat
		if board.at(bx.x - 1, bx.y + 1).is_lowercase() {
			legal_moves.push(Move {from: bx, to: Box{x: bx.x - 1, y: bx.y + 1}});
		}
		if board.at(bx.x + 1, bx.y + 1).is_lowercase() {
			legal_moves.push(Move {from: bx, to: Box{x: bx.x + 1, y: bx.y + 1}});
		}
	} else {
		//Go straight by one
		if board.at(bx.x, bx.y - 1) == '.' {
			legal_moves.push(Move {from: bx, to: Box{x: bx.x, y: bx.y - 1}});
			//Go straight by two
			if board.at(bx.x, bx.y - 2) == '.' && bx.y == 6 {
				legal_moves.push(Move {from: bx, to: Box{x: bx.x, y: bx.y - 2}});
			}
		}
		//Eat
		if board.at(bx.x - 1, bx.y - 1).is_uppercase() {
			legal_moves.push(Move {from: bx, to: Box{x: bx.x - 1, y: bx.y - 1}});
		}
		if board.at(bx.x + 1, bx.y - 1).is_uppercase() {
			legal_moves.push(Move {from: bx, to: Box{x: bx.x + 1, y: bx.y - 1}});
		}
	}
	return legal_moves;
}

pub fn rock_legal_moves_for(player: Player, board: &Board, bx: Box) -> Vec<Move> {
	let mut legal_moves: Vec<Move> = Vec::new();
	let mut x; let mut y;
	//Up moves
	x = bx.x;
	y = bx.y - 1;
	while y >= 0 && board.color_at(x, y) != player {
		legal_moves.push(Move {from: bx, to: Box {x, y}});
		if board.color_at(x, y) != Player::None && board.color_at(x, y) != player {
			break;
		}
		y -= 1;
	}
	//Down moves
	x = bx.x;
	y = bx.y + 1;
	while y < 8 && board.color_at(x, y) != player {
		legal_moves.push(Move {from: bx, to: Box {x, y}});
		if board.color_at(x, y) != Player::None && board.color_at(x, y) != player {
			break;
		}
		y += 1;
	}
	//Left moves
	x = bx.x - 1;
	y = bx.y;
	while x >= 0 && board.color_at(x, y) != player {
		legal_moves.push(Move {from: bx, to: Box {x, y}});
		if board.color_at(x, y) != Player::None && board.color_at(x, y) != player {
			break;
		}
		x -= 1;
	}
	//Right moves
	x = bx.x + 1;
	y = bx.y;
	while x < 8 && board.color_at(x, y) != player {
		legal_moves.push(Move {from: bx, to: Box {x, y}});
		if board.color_at(x, y) != Player::None && board.color_at(x, y) != player {
			break;
		}
		x += 1;
	}
	return legal_moves;
}

pub fn horse_legal_moves_for(player: Player, board: &Board, bx: Box) -> Vec<Move> {
	let mut legal_moves: Vec<Move> = Vec::new();
	let mut boxes: Vec<Box> = Vec::new();
	//Add 8 potential dst Box for knight
	boxes.push(Box{x: bx.x + 1, y: bx.y + 2});
	boxes.push(Box{x: bx.x + 1, y: bx.y - 2});
	boxes.push(Box{x: bx.x - 1, y: bx.y + 2});
	boxes.push(Box{x: bx.x - 1, y: bx.y - 2});
	boxes.push(Box{x: bx.x + 2, y: bx.y + 1});
	boxes.push(Box{x: bx.x + 2, y: bx.y - 1});
	boxes.push(Box{x: bx.x - 2, y: bx.y + 1});
	boxes.push(Box{x: bx.x - 2, y: bx.y - 1});
	//Test and add them for specific player
	if player == Player::Black {
		for b in boxes {
			if !board.at(b.x, b.y).is_uppercase() &&
			b.x >= 0 && b.y >= 0 && b.x < 8 && b.y < 8 {
				legal_moves.push(Move {from: bx, to: b})
			}
		}
	} else {
		for b in boxes {
			if !board.at(b.x, b.y).is_lowercase() &&
			b.x >= 0 && b.y >= 0 && b.x < 8 && b.y < 8 {
				legal_moves.push(Move {from: bx, to: b})
			}
		}
	}
	return legal_moves;
}

pub fn bishop_legal_moves_for(player: Player, board: &Board, bx: Box) -> Vec<Move> {
	let mut legal_moves: Vec<Move> = Vec::new();
	let mut x; let mut y;
	//Top left diagonal moves
	x = bx.x - 1;
	y = bx.y - 1;
	while y >= 0 && x >= 0 && board.color_at(x, y) != player {
		legal_moves.push(Move {from: bx, to: Box {x, y}});
		if board.color_at(x, y) != Player::None && board.color_at(x, y) != player {
			break;
		}
		x -= 1;
		y -= 1;
	}
	//Top right diagonal moves
	x = bx.x + 1;
	y = bx.y - 1;
	while y >= 0 && x < 8 && board.color_at(x, y) != player {
		legal_moves.push(Move {from: bx, to: Box {x, y}});
		if board.color_at(x, y) != Player::None && board.color_at(x, y) != player {
			break;
		}
		x += 1;
		y -= 1;
	}
	//Bottom left diagonal moves
	x = bx.x - 1;
	y = bx.y + 1;
	while y < 8 && x >= 0 && board.color_at(x, y) != player {
		legal_moves.push(Move {from: bx, to: Box {x, y}});
		if board.color_at(x, y) != Player::None && board.color_at(x, y) != player {
			break;
		}
		x -= 1;
		y += 1;
	}
	//Bottom right diagonal moves
	x = bx.x + 1;
	y = bx.y + 1;
	while y < 8 && x < 8 && board.color_at(x, y) != player {
		legal_moves.push(Move {from: bx, to: Box {x, y}});
		if board.color_at(x, y) != Player::None && board.color_at(x, y) != player {
			break;
		}
		x += 1;
		y += 1;
	}
	return legal_moves;
}

pub fn queen_legal_moves_for(player: Player, board: &Board, bx: Box) -> Vec<Move> {
	let mut legal_moves: Vec<Move> = Vec::new();
	//Queen can move like a bishop and like a rock
	legal_moves.append(&mut bishop_legal_moves_for(player, board, bx));
	legal_moves.append(&mut rock_legal_moves_for(player, board, bx));
	return legal_moves;
}

pub fn king_legal_moves_for(player: Player, board: &Board, bx: Box) -> Vec<Move> {
	let mut legal_moves: Vec<Move> = Vec::new();
	let mut boxes: Vec<Box> = Vec::new();
	//Add 8 potential dst Box for king
	boxes.push(Box{x: bx.x, y: bx.y + 1});
	boxes.push(Box{x: bx.x, y: bx.y - 1});
	boxes.push(Box{x: bx.x + 1, y: bx.y - 1});
	boxes.push(Box{x: bx.x + 1, y: bx.y});
	boxes.push(Box{x: bx.x + 1, y: bx.y + 1});
	boxes.push(Box{x: bx.x - 1, y: bx.y - 1});
	boxes.push(Box{x: bx.x - 1, y: bx.y});
	boxes.push(Box{x: bx.x - 1, y: bx.y + 1});
	//Test and add them for specific player
	if player == Player::Black {
		for b in boxes {
			if !board.at(b.x, b.y).is_uppercase() &&
			b.x >= 0 && b.y >= 0 && b.x < 8 && b.y < 8 {
				legal_moves.push(Move {from: bx, to: b})
			}
		}
	} else {
		for b in boxes {
			if !board.at(b.x, b.y).is_lowercase() &&
			b.x >= 0 && b.y >= 0 && b.x < 8 && b.y < 8 {
				legal_moves.push(Move {from: bx, to: b})
			}
		}
	}
	return legal_moves;
}
