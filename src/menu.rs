/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   menu.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lfalkau <lfalkau@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2020/02/26 17:50:19 by lfalkau           #+#    #+#             */
/*   Updated: 2020/02/26 17:50:33 by lfalkau          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::io::{self, Write};

pub enum Mode {
	SinglePlayer, TwoPlayers
}

pub fn shess_menu() -> Mode {
	println!("Welcome to our brand new Shess game !\n");
	println!("1: Single player");
	println!("2: Two players\n");
	loop {
		print!("Choice >> ");
		io::stdout().flush().unwrap();
		let mut mode = String::new();
		io::stdin().read_line(&mut mode);
		if mode.starts_with('1') { return Mode::SinglePlayer; }
		if mode.starts_with('2') { return Mode::TwoPlayers; }
		println!("Please type 1 or 2, don't make mystic choice");
	}
}
