/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   game.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lfalkau <lfalkau@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2020/02/26 17:50:17 by lfalkau           #+#    #+#             */
/*   Updated: 2020/02/27 22:43:04 by lfalkau          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use colored::*;
use crate::board;
use crate::mv;
use std::io;
use std::io::{Write};

pub fn start()
{
	let mut board: [[char; 8]; 8] = board::set();
	let mut turn = 0;
	while !someone_has_won()
	{
		board::print(board);
		play(turn % 2, &board);
		turn += 1;
	}
}

fn play(turn: u8, board: &[[char; 8]; 8])
{
	loop
	{
		print!("{}", format!("{}", "White's turn, what do you want to do ? ".bright_yellow()));
		io::stdout().flush().unwrap();
		let mut input = String::new();
		io::stdin().read_line(&mut input).expect("Error: read error");
		if let Ok(m) = mv::parse(&input)
		{
			if mv::try_proceed(m, board)
			{
				return ;
			}
			println!("{}", format!("{}", "It's not reasonable.".bright_red()));
		}
		println!("{}", format!("{}", "Format: e6 to a4".bright_red()));
	}

}

fn someone_has_won() -> bool
{
	false
}
