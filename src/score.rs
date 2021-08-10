use crate::wall::Wall;
use crate::Reset;
use bevy::app::EventReader;
use bevy::asset::AssetServer;
use bevy::ecs::prelude::Query;
use bevy::ecs::system::{Commands, Res};
use bevy::math::{Rect, Size};
use bevy::text::TextStyle;
use bevy::text::{Text, TextSection};
use bevy::ui::entity::TextBundle;
use bevy::ui::{Style, Val};
use std::fmt::{Display, Formatter};

#[derive(Default)]
pub struct Score {
	pub left: usize,
	pub right: usize,
}

impl Display for Score {
	fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
		write!(formatter, "{} : {}", self.left, self.right)
	}
}

pub struct ScoreBoard;

pub fn spawn_score_board(commands: &mut Commands, asset_server: &Res<AssetServer>) {
	commands
		.spawn()
		.insert_bundle(TextBundle {
			style: Style {
				size: Size::new(Val::Px(100.0), Val::Px(50.0)),
				// center
				margin: Rect {
					top: Val::Px(2.0 * Wall::THICKNESS),
					..Rect::all(Val::Auto)
				},
				..Default::default()
			},
			text: Text {
				sections: vec![TextSection {
					value: Score::default().to_string(),
					style: TextStyle {
						font_size: 60.0,
						font: asset_server.load("fonts/FiraSans-Bold.ttf"),
						..Default::default()
					},
				}],
				alignment: Default::default(),
			},
			..Default::default()
		})
		.insert(ScoreBoard);
}

pub fn scoreboard_update_system(
	mut reset_reader: EventReader<Reset>,
	score: Res<Score>,
	mut query: Query<(&mut Text, &ScoreBoard)>,
) {
	if reset_reader.iter().last().is_none() {
		return;
	}

	for (mut text, _score_board) in query.iter_mut() {
		if let Some(section) = text.sections.get_mut(0) {
			section.value = score.to_string();
		}
	}
}
