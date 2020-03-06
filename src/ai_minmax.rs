/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ai_minmax.rs                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lfalkau <lfalkau@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2020/03/06 10:37:14 by lfalkau           #+#    #+#             */
/*   Updated: 2020/03/06 12:54:49 by lfalkau          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::board::{Board, Player, Box};
use crate::move_general::{Move, get_legal_moves_for};

const DEPTH: u8 = 3;
const EVAL_MAX: i32 = 100_000_000;
const EVAL_MIN: i32 = -100_000_000;

pub fn play(player: Player, board: &mut Board) {

	let mut mv: Move = Move {from: Box{x: 0, y: 0}, to: Box{x: 0, y: 0}};
	let score = max(player, board, DEPTH, &mut mv);
	println!("{}  {:?}", score, mv);
	//board.perform_move(mv);
}

fn min(player: Player, board: &Board, depth: u8, best_move: &mut Move) -> i32 {
	if depth == 0 {
		return eval(board);
	}
	let mut min = EVAL_MAX;
	for mv in get_legal_moves_for(player, board, true) {
		let mut board_cpy = board.clone();
		board_cpy.perform_move(mv);
		let score = max(player, &board_cpy, depth - 1, best_move);
		if score < min {
			min = score;
			*best_move = mv;
		}
	}
	return min;
}

fn max(player: Player, board: &Board, depth: u8, best_move: &mut Move) -> i32 {
	if depth == 0 {
		return eval(board);
	}
	let mut max = EVAL_MIN;
	for mv in get_legal_moves_for(player, board, true) {
		let mut board_cpy = board.clone();
		board_cpy.perform_move(mv);
		let score = min(player, &board_cpy, depth - 1, best_move);
		if score > max {
			max = score;
			*best_move = mv;
		}
	}
	return max;
}

fn eval(board: &Board) -> i32 {
	let mut score = 0;
	for x in 0..8 {
		for y in 0..8 {
			score += match board.at(x, y) {
				'P' | 'p' => 100,
				'R' | 'r' => 500,
				'H' | 'h' => 320,
				'B' | 'b' => 330,
				'Q' | 'q' => 900,
				'K' | 'k' => 20000,
				_ => 0,
			};
		}
	}
	return score;
}
