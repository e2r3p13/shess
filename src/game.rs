/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   game.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lfalkau <lfalkau@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2020/02/26 17:50:17 by lfalkau           #+#    #+#             */
/*   Updated: 2020/03/02 21:19:48 by lfalkau          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use colored::*;
use crate::board;
use crate::mvg;
use crate::mvp;
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
	let player: board::Color = match turn {
		0 => board::Color::White,
		1 => board::Color::Black,
		_ => board::Color::None
	};
	loop
	{
		let mut input = String::new();
		if player == board::Color::White { print!("{}", format!("{}", "White's turn, what do you want to do ? ".bright_yellow())); }
		if player == board::Color::Black { print!("{}", format!("{}", "Black's turn, what do you want to do ? ".bright_yellow())); }
		io::stdout().flush().unwrap();
		io::stdin().read_line(&mut input).expect("Error: read error");
		input.pop();
		if input == "print"
		{
			board.print();
			continue;
		}
		if let Ok(m) = mvg::parse(&input)
		{
			if  !mvg::is_yours(&m, board, turn)
			{
				println!("{}", format!("{}", "You don't have any piece in here.".bright_red()));
				continue;
			}
			if  !mvg::try_proceed(&m, board)
			{
				continue;
			}
			let last_ate = board.perform_move(m);
			if chess_for(player, board)
			{
				board.cancel_move(m, last_ate);
				println!("{}", format!("{}", "You have to protect your king.".bright_red()));
				continue;
			}
			return;
		}
		println!("{}", format!("{}", "Format: XX to XX / XX from XX".bright_red()));
	}
}

fn chess_for(player: board::Color, b: &mut board::Board) -> bool
{
	let mut king_pos;

	if player == board::Color::White
	{
		king_pos = b.get_pos('k');
		for y in 0..8
		{
			for x in 0..8
			{
				let c = b.at(x, y);
				if c.is_uppercase()
				{
					let chess: bool;
					let m = mvg::Move {from: board::Box {x: x, y: y}, to: board::Box {x: king_pos.x, y: king_pos.y}};
					chess = match c
					{
						'P' => mvp::move_pawn(board::Color::Black, &m, b),
						'R' => mvp::move_rock(&m, b),
						'H' => mvp::move_knight(&m),
						'B' => mvp::move_bishop(&m, b),
						'Q' => mvp::move_queen(&m, b),
						'K' => mvp::move_king(&m),
						_ => false
					};
					if chess { return true; }
				}
			}
		}
	}
	if player == board::Color::Black
	{
		king_pos = b.get_pos('K');
		for y in 0..8
		{
			for x in 0..8
			{
				let c = b.at(x, y);
				if c.is_lowercase()
				{
					let chess: bool;
					let m = mvg::Move {from: board::Box {x: x, y: y}, to: board::Box {x: king_pos.x, y: king_pos.y}};
					chess = match c
					{
						'p' => mvp::move_pawn(board::Color::White, &m, b),
						'r' => mvp::move_rock(&m, b),
						'h' => mvp::move_knight(&m),
						'b' => mvp::move_bishop(&m, b),
						'q' => mvp::move_queen(&m, b),
						'k' => mvp::move_king(&m),
						_ => false
					};
					if chess { return true; }
				}
			}
		}
	}
	return false;
}

fn someone_has_won() -> bool
{
	false
}
