/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: lfalkau <lfalkau@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2020/02/26 16:36:03 by lfalkau           #+#    #+#             */
/*   Updated: 2020/03/05 21:12:45 by lfalkau          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

mod game;
mod board;
mod move_pieces;
mod move_general;

fn main() {
	//What a small main !
	//We just call the start function that is responsable for all the program
	game::start();
}
