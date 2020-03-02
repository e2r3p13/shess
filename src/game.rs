/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   game.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lfalkau <lfalkau@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2020/02/26 17:50:17 by lfalkau           #+#    #+#             */
/*   Updated: 2020/03/02 16:33:24 by lfalkau          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use colored::*;
use crate::board;
use crate::mvg;
use std::io;
use std::io::{Write};

const STARTING_BOARD: [[char; 8]; 8] =
[
	['R', 'H', 'B', 'Q', 'K', 'B', 'H', 'R'],
	['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'],
	['.', '.', '.', '.', '.', '.', '.', '.'],
	['.', '.', '.', '.', '.', '.', '.', '.'],
	['.', '.', '.', '.', '.', '.', '.', '.'],
	['.', '.', '.', '.', '.', '.', '.', '.'],
	['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'],
	['r', 'h', 'b', 'q', 'k', 'b', 'h', 'r']
];

pub fn start()
{
	let mut board = board::Board { raw: STARTING_BOARD };
	let mut turn = 0;

	board.set();
	while !someone_has_won()
	{
		board.print();
		play(turn % 2, &mut board);
		turn += 1;
	}
}

fn play(turn: u8, board: &mut board::Board)
{
	loop
	{
		match turn
		{
			0 => print!("{}", format!("{}", "White's turn, what do you want to do ? ".bright_yellow())),
			1 => print!("{}", format!("{}", "Black's turn, what do you want to do ? ".bright_yellow())),
			_ => panic!("Impossible statement")
		}
		io::stdout().flush().unwrap();
		let mut input = String::new();
		io::stdin().read_line(&mut input).expect("Error: read error");
		if let Ok(m) = mvg::parse(&input)
		{
			if  !mvg::is_yours(&m, board, turn)
			{
				println!("{}", format!("{}", "You don't have any piece in here.".bright_red()));
				continue;
			}
			if  !mvg::try_proceed(&m, board)
			{
				println!("{}", format!("{}", "It's not reasonable.".bright_red()));
				continue;
			}
			board.perform_move(m);
			return;
		}
		println!("{}", format!("{}", "Format: XX to XX / XX from XX".bright_red()));
	}

}

fn someone_has_won() -> bool
{
	false
}
