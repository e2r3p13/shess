/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ai_random.rs                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lfalkau <lfalkau@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2020/03/06 00:30:56 by lfalkau           #+#    #+#             */
/*   Updated: 2020/03/11 00:34:07 by lfalkau          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::board::{Board, Player};
use crate::move_general::{get_legal_moves_for};
use rand::Rng;

pub fn play(player: Player, board: &mut Board) {
	let legal_moves = get_legal_moves_for(player, board, true);
	let mv_id = rand::thread_rng().gen_range(0, legal_moves.len());
	let mv = legal_moves[mv_id];
	board.perform_move(mv);
	if board.pawn_upgrade() {
		let q = if player == Player::Black {'Q'} else {'q'};
		board.set_at(mv.to.x, mv.to.y, q);
	}
}
