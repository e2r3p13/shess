/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   menu.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lfalkau <lfalkau@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2020/02/26 17:50:19 by lfalkau           #+#    #+#             */
/*   Updated: 2020/02/26 19:17:49 by lfalkau          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

pub enum Mode {
	SinglePlayer, TwoPlayers
}

pub fn shess_menu() -> Mode {
	loop {
		let stdin = stdin();
		let mut stdout = stdout().into_raw_mode().unwrap();

		write!(stdout, "{}{}{}-------------------\n", termion::clear::All, termion::cursor::Goto(1, 1), termion::cursor::Show).unwrap();
		write!(stdout, "{}   Single player\n", termion::cursor::Goto(1, 2));
		write!(stdout, "{}   Two players\n", termion::cursor::Goto(1, 3));
		write!(stdout, "{}-------------------", termion::cursor::Goto(1, 4));
		stdout.flush().unwrap();

		for c in stdin.keys() {
			write!(stdout, "{}{}", termion::cursor::Goto(1, 1), termion::clear::CurrentLine).unwrap();

			match c.unwrap() {
				Key::Char('q') => break,
				Key::Char(c) => println!("{}", c),
				Key::Alt(c) => println!("^{}", c),
				Key::Ctrl(c) => {
					panic!("")},
				Key::Esc => println!("ESC"),
				Key::Left => println!("←"),
				Key::Right => println!("→"),
				Key::Up => println!("↑"),
				Key::Down => println!("↓"),
				_ => {}
			}
			stdout.flush().unwrap();
		}
	}
	Mode::SinglePlayer
}

fn print_menu(mode: u8) {


	println!("-------------------");
	println!("   Single player");
	println!("    Two players");
	println!("-------------------");

	if mode == 0 {

	}
	if mode == 1 {

	}
}
