/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ai_negamax.rs                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lfalkau <lfalkau@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2020/03/06 10:37:14 by lfalkau           #+#    #+#             */
/*   Updated: 2020/03/11 00:34:02 by lfalkau          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::pieces_table;
use crate::board::{Board, Player, Box};
use crate::move_general::{Move, get_legal_moves_for};
use rand::seq::SliceRandom;
use rand::thread_rng;

const DEPTH: u8 = 3;
const EVAL_MIN: i32 = -100_000_000;

pub fn play(player: Player, board: &mut Board) {

	let mut mv: Move = Move {from: Box{x: 0, y: 0}, to: Box{x: 0, y: 0}};
	negamax(player, board, DEPTH, &mut mv);
	board.perform_move(mv);
	if board.pawn_upgrade() {
		let q = if player == Player::Black {'Q'} else {'q'};
		board.set_at(mv.to.x, mv.to.y, q);
	}
}

fn negamax(player: Player, board: &Board, depth: u8, mv: &mut Move) -> i32 {
	//End of recursion
	if depth == 0 {
		return eval(board, player);
	}
	//Create vectors to choose randomly the best move
	let am: Vec<Move> = get_legal_moves_for(player, board, true);
	let mut s: Vec<i32> = Vec::new();
	let mut bm: Vec<Move> = Vec::new();
	//Start exploring move tree
	for m in &am {
		let mut board_cpy = board.clone();
		board_cpy.perform_move(*m);
		s.push(-negamax(player.opponent(), &board_cpy, depth - 1, mv));
	}
	//Find out max value
	let max = get_max(&s);
	//Add all equal moves
	for i in 0..s.len() {
		if s[i] == max {
			bm.push(am[i]);
		}
	}
	//Choose one randomly
	let mut rng = thread_rng();
	if let Some(m) = bm.choose(&mut rng) {
		*mv = *m;
	}
	return max;
}

fn eval(board: &Board, player: Player) -> i32 {
	let mut score = board.get_score_for(player);
	for x in 0..8 {
		for y in 0..8 {
			match board.at(x, y) {
				'P' | 'p' => {
					if board.color_at(x, y) == player {
						score += pieces_table::PAWN_TABLE[y as usize][x as usize];
					} else if board.color_at(x, y) == player.opponent() {
						score -= pieces_table::PAWN_TABLE[y as usize][x as usize];
					}
				},
				'R' | 'r' => {
					if board.color_at(x, y) == player {
						score += pieces_table::ROOK_TABLE[y as usize][x as usize];
					} else if board.color_at(x, y) == player.opponent() {
						score -= pieces_table::ROOK_TABLE[y as usize][x as usize];
					}
				},
				'H' | 'h' => {
					if board.color_at(x, y) == player {
						score += pieces_table::KNIGHT_TABLE[y as usize][x as usize];
					} else if board.color_at(x, y) == player.opponent() {
						score -= pieces_table::KNIGHT_TABLE[y as usize][x as usize];
					}
				},
				'B' | 'b' => {
					if board.color_at(x, y) == player {
						score += pieces_table::BISHOP_TABLE[y as usize][x as usize];
					} else if board.color_at(x, y) == player.opponent() {
						score -= pieces_table::BISHOP_TABLE[y as usize][x as usize];
					}
				},
				'Q' | 'q' => {
					if board.color_at(x, y) == player {
						score += pieces_table::QUEEN_TABLE[y as usize][x as usize];
					} else if board.color_at(x, y) == player.opponent() {
						score -= pieces_table::QUEEN_TABLE[y as usize][x as usize];
					}
				},
				_ => (),
			}
		}
	}
	return score;
}

fn get_max(values: &Vec<i32>) -> i32 {
	let mut max = EVAL_MIN;

	for value in values {
		if *value > max {
			max = *value;
		}
	}
	return max;
}
