/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lfalkau <lfalkau@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2020/02/26 16:36:03 by lfalkau           #+#    #+#             */
/*   Updated: 2020/02/27 17:05:38 by lfalkau          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod menu;
mod game;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

fn main() {

    let mode = menu::shess_menu();
	game::shess(mode);

}
