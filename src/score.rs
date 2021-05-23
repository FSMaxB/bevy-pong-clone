use crate::wall::Wall;
use bevy::asset::AssetServer;
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
		write!(formatter, "{}:{}", self.left, self.right)
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
					value: "0 : 0".to_string(),
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
