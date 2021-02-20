use bevy::asset::AssetServer;
use bevy::ecs::{Commands, Res};
use bevy::text::TextStyle;
use bevy::ui::entity::TextBundle;
use bevy::ui::widget::Text;
use bevy::ui::{AlignItems, AlignSelf, Style};

pub struct ScoreBoard;

pub fn spawn_score_board(commands: &mut Commands, asset_server: &Res<AssetServer>) {
	commands
		.spawn(TextBundle {
			style: Style {
				align_self: AlignSelf::Center,
				align_items: AlignItems::Center,
				..Default::default()
			},
			text: Text {
				value: "0 : 0".to_string(),
				font: asset_server.load("fonts/FiraSans-Bold.ttf"),
				style: TextStyle {
					font_size: 60.0,
					..Default::default()
				},
				..Default::default()
			},
			..Default::default()
		})
		.with(ScoreBoard);
}
