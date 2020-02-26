/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   game.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lfalkau <lfalkau@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2020/02/26 17:50:17 by lfalkau           #+#    #+#             */
/*   Updated: 2020/02/26 19:02:46 by lfalkau          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

// R H B Q K B H R
// P P P P P P P P
// . . . . . . . .
// . . . . . . . .
// . . . . . . . .
// . . . . . . . .
// p p p p p p p p
// r h b k q b h r

extern crate termion;

use super::menu;

pub fn shess(mode: menu::Mode) {
	let mut board: [[char; 8]; 8] = set_board();
	let turn = 0;
	while !someone_has_won() {
		play(turn % 2);
	}
	print_board(board);
}

fn set_board() -> [[char; 8]; 8] {
	[
		['R', 'H', 'B', 'Q', 'K', 'B', 'H', 'R'],
		['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'],
		['.', '.', '.', '.', '.', '.', '.', '.'],
		['.', '.', '.', '.', '.', '.', '.', '.'],
		['.', '.', '.', '.', '.', '.', '.', '.'],
		['.', '.', '.', '.', '.', '.', '.', '.'],
		['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'],
		['r', 'h', 'b', 'q', 'k', 'b', 'h', 'r']
	]
}

fn print_board(board: [[char; 8]; 8]) {
	for row in 0..8 {
		for column in 0..8 {
			print!("{}", board[row][column]);
		}
		println!("");
	}
}

fn play(turn: u8) {

}

fn someone_has_won() -> bool {
	true
}
