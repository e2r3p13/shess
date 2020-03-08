/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lfalkau <lfalkau@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2020/02/26 16:36:03 by lfalkau           #+#    #+#             */
/*   Updated: 2020/03/08 14:17:03 by lfalkau          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod game;
mod board;
mod move_pieces;
mod move_general;
mod ai_random;
mod ai_minmax;

use crate::game::Mode::{PvP, RandomAI, MinMaxAI, MinMaxAIVS};
use colored::*;

fn main() {
	loop {
		std::process::Command::new("clear").status().unwrap();
		println!("{}", format!("{}", "Welcome to SHESS, the chess game you can play in a shell!\n".bright_yellow()));
		println!("{}", format!("{}", "   1: Two players (boring)"));
		println!("{}", format!("{}", "   2: Play against random IA"));
		println!("{}", format!("{}", "   3: Play against MinMax IA"));
		println!("{}", format!("{}", "   4: MinMax VS MinMax"));
		print!("{}", format!("{}", "\n>>> ".bright_yellow()));

		let input = game::read();

		if input == "1" { game::start(PvP); }
		if input == "2" { game::start(RandomAI); }
		if input == "3" { game::start(MinMaxAI); }
		if input == "4" { game::start(MinMaxAIVS); }
		if input == "quit" || input == "exit" { break; }
	}
}
