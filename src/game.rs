/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   game.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lfalkau <lfalkau@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2020/02/26 17:50:17 by lfalkau           #+#    #+#             */
/*   Updated: 2020/03/05 02:12:51 by lfalkau          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use colored::*;
use crate::board;
use crate::mvg;
use std::io;
use std::io::{Write};

pub fn start() {
	let mut board = board::Board { raw: board::DEFAULT_BOARD };
	let mut turn = 0;
	let mut loser = board::Color::None;
	while loser == board::Color::None {
		board.print();
		let player = current_player_turn(turn);
		play(player, &mut board);
		turn += 1;
	}
	match loser {
		board::Color::Black => print!("{}", format!("{}", "Whites won!!".bright_green())),
		board::Color::White => print!("{}", format!("{}", "Blacks won!!".bright_green())),
		board::Color::None => print!("{}", format!("{}", "Draw game..".bright_green())),
	}
}

fn play(player: board::Color, board: &mut board::Board)
{
	loop {
		let input = get_input_for(player);
		if let Ok(mv) = parse_move(player, &input) {
			if mvg::is_legal_move_for(player, mv, board) {
				board.perform_move(mv);
				return;
			}
		}
		println!("{}", format!("{}", "Format: 'e2 e4'".bright_red()));
	}
}

fn current_player_turn(turn: u8) -> board::Color {
	match turn % 2 {
		0 => board::Color::White,
		1 => board::Color::Black,
		_ => board::Color::None
	}
}

fn get_input_for(player: Board::Color) -> String {
	let mut input = String::new();
	if player == board::Color::White {
		print!("{}", format!("{}", "White's turn, what do you want to do ? ".bright_yellow()));
	}
	if player == board::Color::Black {
		print!("{}", format!("{}", "Black's turn, what do you want to do ? ".bright_yellow()));
	}
	io::stdout().flush().unwrap();
	io::stdin().read_line(&mut input).expect("Error: read error");
	input.pop();
	return input;
}

fn parse_move(player: board::Color, input: &str) -> Result<mvg::Move, io::Error> {
	let input = input.to_lowercase();
	let fields: Vec<&str> = input.split_whitespace().collect();
	if fields.len() == 2 {
		let from = parse_box(fields[0])?;
		let to = parse_box(fields[2])?;
		return Ok(mvg::Move { from, to });
	}
	return Err(io::Error::new(io::ErrorKind::Other, "oh no!"));
}

fn parse_box(input: &str) -> Result<board::Box, io::Error> {
	let mut pos = [0, 0];
	if input.len() == 2 {
		let mut input = input.chars();
		for i in 0..2 {
			pos[i] = match input.next().unwrap() {
				'a' | '8' => 0,
				'b' | '7' => 1,
				'c' | '6' => 2,
				'd' | '5' => 3,
				'e' | '4' => 4,
				'f' | '3' => 5,
				'g' | '2' => 6,
				'h' | '1' => 7,
				_ => {
					return Err(io::Error::new(io::ErrorKind::Other, "oh no!"));
				}
			};
		}
		return Ok(board::Box {x: pos[0], y: pos[1]});
	}
	return Err(io::Error::new(io::ErrorKind::Other, "oh no!"));
}
