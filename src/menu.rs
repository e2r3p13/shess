/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   menu.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lfalkau <lfalkau@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2020/02/26 17:50:19 by lfalkau           #+#    #+#             */
/*   Updated: 2020/02/27 19:48:18 by lfalkau          ###   ########.fr       */
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

	let mut mode = Mode::SinglePlayer;

	loop {

		let stdin = stdin();
		let mut stdout = stdout().into_raw_mode().unwrap();

		print_menu(&mode);

		if let Some(Ok(c)) = stdin.keys().next() {
			match c {
				Key::Up | Key::Down => {
					mode = switch_mode(&mode);
				},
				Key::Ctrl(_) => {
					write!(stdout, "{}{}{}", termion::cursor::Up(3), termion::clear::AfterCursor, termion::cursor::Show).unwrap();
					panic!("");
				},
				Key::Char(ch) => {
					if ch as u8 == 10 {
						write!(stdout, "{}{}", termion::cursor::Show, termion::clear::AfterCursor).unwrap();
						break;
					}
				},
				_ => {}
			}
		}
	}
	mode
}

fn print_menu(mode: &Mode) {

	let mut stdout = stdout().into_raw_mode().unwrap();
	write!(stdout, "{}", termion::cursor::Hide).unwrap();

	match mode {
		Mode::SinglePlayer => {
			print!("┏━━━━━━━━━━━━━━━━━━━━━━━━━━━┓\n\r");
			print!("┃                           ┃ \n\r");
			print!("┃      -- SHEEEEESS --      ┃ \n\r");
			print!("┃                           ┃ \n\r");
			print!("┃     > Single player <     ┃ \n\r");
			print!("┃        Two players        ┃ \n\r");
			print!("┃                           ┃ \n\r");
			print!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━┛\r");
		}
		Mode::TwoPlayers => {
			print!("┏━━━━━━━━━━━━━━━━━━━━━━━━━━━┓\n\r");
			print!("┃                           ┃ \n\r");
			print!("┃      -- SHEEEEESS --      ┃ \n\r");
			print!("┃                           ┃ \n\r");
			print!("┃       Single player       ┃ \n\r");
			print!("┃     >  Two players  <     ┃ \n\r");
			print!("┃                           ┃ \n\r");
			print!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━┛\r");
		}
	}
	write!(stdout, "{}", termion::cursor::Up(7)).unwrap();
	stdout.flush().unwrap();

}

fn switch_mode(mode: &Mode) -> Mode {
	match mode {
		Mode::SinglePlayer => { Mode::TwoPlayers },
		Mode::TwoPlayers => { Mode::SinglePlayer },
	}
}
