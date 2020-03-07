/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ai_minmax.rs                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lfalkau <lfalkau@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2020/03/06 10:37:14 by lfalkau           #+#    #+#             */
/*   Updated: 2020/03/07 01:53:50 by lfalkau          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::board::{Board, Player, Box};
use crate::move_general::{Move, get_legal_moves_for};
use rand::seq::SliceRandom;
use rand::Rng;

const DEPTH: u8 = 3;
const EVAL_MAX: i32 = 100_000_000;
const EVAL_MIN: i32 = -100_000_000;

pub fn play(player: Player, board: &mut Board) {

	let mut mv: Move = Move {from: Box{x: 0, y: 0}, to: Box{x: 0, y: 0}};
	let score = max(player, board, DEPTH, &mut mv);
	println!("score : {}\n{:?}", score, mv);
	board.perform_move(mv);
}

fn min(player: Player, board: &Board, depth: u8, best_move: &mut Move) -> i32 {
	if depth == 0 {
		return eval(board, player.opponent());
	}
	let mut min = EVAL_MAX;
	for mv in get_legal_moves_for(player, board, true) {
		let mut board_cpy = board.clone();
		board_cpy.perform_move(mv);
		let score = max(player.opponent(), &board_cpy, depth - 1, best_move);
		if score < min {
			min = score;
			*best_move = mv;
		}
	}
	return min;
}

fn max(player: Player, board: &Board, depth: u8, best_move: &mut Move) -> i32 {
	if depth == 0 {
		return eval(board, player.opponent());
	}
	let mut max = EVAL_MIN;
	for mv in get_legal_moves_for(player, board, true) {
		let mut board_cpy = board.clone();
		board_cpy.perform_move(mv);
		let score = min(player.opponent(), &board_cpy, depth - 1, best_move);
		if score > max {
			max = score;
			*best_move = mv;
		}
	}
	return max;
}

// fn mmin(player: Player, board: &Board, depth: u8, best_move: &mut Move) -> i32 {
// 	let moves: Vec<Move> = get_legal_moves_for(player, board, true);
// 	let mut best_moves: Vec<Move> = Vec::new();
// 	let mut moves_score: Vec<i32> = Vec::new();
// 	for mv in get_legal_moves_for(player, board, true) {
// 		let mut board_cpy = board.clone();
// 		board_cpy.perform_move(mv);
// 		let mut score;
// 		if depth == 0 {
// 			score = eval(board, player);
// 		} else {
// 			score = mmin(player.opponent(), &board_cpy, depth - 1, best_move);
// 		}
// 		moves_score.push(score);
// 	}
// 	//println!("min {:?}", moves_score);
// 	let min = get_min(&moves_score);
// 	for i in 0..moves.len() {
// 		if moves_score[i] == min {
// 			best_moves.push(moves[i]);
// 		}
// 	}
// 	let id = rand::thread_rng().gen_range(0, best_moves.len());
// 	if best_moves.len() > 0 {
// 		*best_move = best_moves[0];
// 	}
// 	let mut board_cc = board.clone();
// 	board_cc.perform_move(*best_move);
// 	//println!("d:{}   p:{:?}   s:{}", depth, player, min);
// 	//board_cc.print();
// 	return min;
// }
//
// fn mmax(player: Player, board: &Board, depth: u8, best_move: &mut Move) -> i32 {
// 	let moves: Vec<Move> = get_legal_moves_for(player, board, true);
// 	let mut best_moves: Vec<Move> = Vec::new();
// 	let mut moves_score: Vec<i32> = Vec::new();
// 	for mv in get_legal_moves_for(player, board, true) {
// 		let mut board_cpy = board.clone();
// 		board_cpy.perform_move(mv);
// 		let mut score;
// 		if depth == 0 {
// 			score = eval(board, player);
// 		} else {
// 			score = mmax(player.opponent(), &board_cpy, depth - 1, best_move);
// 		}
// 		moves_score.push(score);
// 	}
// 	//println!("max {:?}", moves_score);
// 	let max = get_max(&moves_score);
// 	for i in 0..moves.len() {
// 		if moves_score[i] == max {
// 			best_moves.push(moves[i]);
// 		}
// 	}
// 	let mut id = rand::thread_rng();//.gen_range(0, best_moves.len());
// 	best_moves.shuffle(&mut id);
// 	if best_moves.len() > 0 {
// 		*best_move = best_moves[0];
// 	}
// 	let mut board_cc = board.clone();
// 	board_cc.perform_move(*best_move);
// 	//println!("d:{}   p:{:?}   s:{}", depth, player, max);
// 	//board_cc.print();
// 	return max;
// }

fn eval(board: &Board, player: Player) -> i32 {
	let mut score = 0;
	//println!("{:?}", player);
	for x in 0..8 {
		for y in 0..8 {
			let value = match board.at(x, y) {
				'P' | 'p' => 1,
				'R' | 'r' => 5,
				'H' | 'h' => 3,
				'B' | 'b' => 3,
				'Q' | 'q' => 9,
				'K' | 'k' => 20,
				_ => 0,
			};
			if player == board.color_at(x, y) {
				score += value;
			} else if board.color_at(x, y) == player.opponent() {
				score -= value;
			}
			//print!("{}, ", score);
		}
	}
	//println!("");
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

fn get_min(values: &Vec<i32>) -> i32 {
	let mut min = EVAL_MAX;

	for value in values {
		if *value < min {
			min = *value;
		}
	}
	return min;
}
