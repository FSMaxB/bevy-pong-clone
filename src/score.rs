use crate::wall::Wall;
use bevy::asset::AssetServer;
use bevy::ecs::{Commands, Res};
use bevy::math::{Rect, Size};
use bevy::text::TextStyle;
use bevy::ui::entity::TextBundle;
use bevy::ui::widget::Text;
use bevy::ui::{Style, Val};

pub struct ScoreBoard;

pub fn spawn_score_board(commands: &mut Commands, asset_server: &Res<AssetServer>) {
	commands
		.spawn(TextBundle {
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
				value: "0 : 0".to_string(),
				font: asset_server.load("fonts/FiraSans-Bold.ttf"),
				style: TextStyle {
					font_size: 60.0,
					..Default::default()
				},
			},
			..Default::default()
		})
		.with(ScoreBoard);
}
