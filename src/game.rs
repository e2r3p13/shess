/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   game.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lfalkau <lfalkau@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2020/02/26 17:50:17 by lfalkau           #+#    #+#             */
/*   Updated: 2020/03/10 15:02:58 by lfalkau          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use colored::*;
use crate::board::{Board, Box, DEFAULT_BOARD, Player};
use crate::move_general::{Move, is_legal_move_for, get_legal_moves_for, check_for};
use crate::ai_random;
use crate::ai_negamax;
use std::{io, process};
use std::io::{Write};

#[derive(PartialEq, Eq)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub enum Mode {
	PvP,
	RandomAI,
	NegamaxAI,
}

pub fn start(mode: Mode) {
	//Board init
	let mut board = Board {
		raw: DEFAULT_BOARD,
		black_eaten: [' '; 15],
		nb_black_eaten: 0,
		white_eaten: [' '; 15],
		nb_white_eaten: 0,
	};
	//turn is incremented each time someone plays, loser is used for the end of the game
	let mut turn = 0;
	let loser;
	//Here's the main loop, only ends when someone has won || PAT
	loop {
		//Print the chess board on standard output
		board.print();
		//Defines which player has to play
		let player = current_player_turn(turn);
		//Check if current player can't play, meaning he lost || PAT
		if get_legal_moves_for(player, &board, true).is_empty() {
			loser = if check_for(player, &board) {player} else {Player::None};
			break;
		}
		//If he can play, let's ask him for!
		if mode != Mode::PvP && player == Player::Black {
			match mode {
				Mode::RandomAI => ai_random::play(player, &mut board),
				Mode::NegamaxAI => ai_negamax::play(player, &mut board),
				_ => (),
			}
		} else {
			play(player, &mut board);
		}
		turn += 1;
	}
	//Print which player has won
	match loser {
		Player::Black => println!("{}", format!("{}", "White player won!!\n".bright_green())),
		Player::White => println!("{}", format!("{}", "Black player won!!\n".bright_green())),
		Player::None => println!("{}", format!("{}", "Draw game by PAT\n".bright_green())),
	}
}

fn play(player: Player, board: &mut Board)
{
	loop {
		//Ask player what does he wants to do until he does
		let input = get_input_for(player);
		//Check for special commands
		match &input[..] {
			"print" => { board.print(); continue },
			"eaten" => { board.print_eaten(); continue },
			"exit" | "quit" => { process::exit(0); },
			"help" => { println!("{}", format!("{}", "Commands: print, eaten, exit, quit or move (Format: 'e2 e4')".bright_purple())); continue },
			_ => (),
		}
		//If it's not a special command, try to parse a move
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

fn current_player_turn(turn: u64) -> Player {
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
	return read();
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

pub fn read() -> String {
	let mut input = String::new();
	io::stdout().flush().unwrap();
	io::stdin().read_line(&mut input).expect("Error: read error");
	//Remove trailing newline character
	input.pop();
	return input;
}
