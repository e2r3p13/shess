/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   game.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lfalkau <lfalkau@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2020/02/26 17:50:17 by lfalkau           #+#    #+#             */
/*   Updated: 2020/03/05 03:23:27 by lfalkau          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use colored::*;
use crate::board::{Board, Box, DEFAULT_BOARD, Player};
use crate::move_general::{Move, is_legal_move_for};
use std::io;
use std::io::{Write};

pub fn start() {
	let mut board = Board {
		raw: DEFAULT_BOARD, black_king_has_moved: false, white_king_has_moved: false
	};
	let mut turn = 0;
	let mut loser = Player::None;
	while loser == Player::None {
		board.print();
		let player = current_player_turn(turn);
		play(player, &mut board);
		turn += 1;
	}
	match loser {
		Player::Black => print!("{}", format!("{}", "Whites won!!".bright_green())),
		Player::White => print!("{}", format!("{}", "Blacks won!!".bright_green())),
		Player::None => print!("{}", format!("{}", "Draw game..".bright_green())),
	}
}

fn play(player: Player, board: &mut Board)
{
	loop {
		let input = get_input_for(player);
		if let Ok(mv) = parse_move(&input) {
			if is_legal_move_for(player, mv, board) {
				board.perform_move(mv);
				return;
			}
			println!("{}", format!("{}", "Forbidden move".bright_red()));
			continue;
		}
		println!("{}", format!("{}", "Format: 'e2 e4'".bright_red()));
	}
}

fn current_player_turn(turn: u8) -> Player {
	//As white player begins, each even turn has to be played by white player
	//Same way, odd turn will be played by black player
	match turn % 2 {
		0 => Player::White,
		1 => Player::Black,
		_ => Player::None
	}
}

fn get_input_for(player: Player) -> String {
	//This funtion returns the player's input as taken in command line
	//User friendly message
	if player == Player::White {
		print!("{}", format!("{}", "White's turn, what do you want to do ? ".bright_yellow()));
	}
	if player == Player::Black {
		print!("{}", format!("{}", "Black's turn, what do you want to do ? ".bright_yellow()));
	}
	//Read from command line
	let mut input = String::new();
	io::stdout().flush().unwrap();
	io::stdin().read_line(&mut input).expect("Error: read error");
	//Remove trailing newline character
	input.pop();
	return input;
}

fn parse_move(input: &str) -> Result<Move, io::Error> {
	//Return a move from user input
	//Can throw Error
	let input = input.to_lowercase();
	let fields: Vec<&str> = input.split_whitespace().collect();
	if fields.len() == 2 {
		let f = parse_box(fields[0])?;
		let t = parse_box(fields[1])?;
		return Ok(Move { from: f, to: t });
	}
	return Err(io::Error::new(io::ErrorKind::InvalidInput, "A move only contains two fields"));
}

fn parse_box(input: &str) -> Result<Box, io::Error> {
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
					return Err(io::Error::new(io::ErrorKind::InvalidInput, "Unrecognized box"));
				}
			};
		}
		return Ok(Box {x: pos[0], y: pos[1]});
	}
	return Err(io::Error::new(io::ErrorKind::InvalidInput, "A box only contains two characters"));
}
