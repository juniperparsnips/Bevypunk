use bevy::{prelude::*, sprite::Anchor};
use crate::*;


// #=========================#
// #=== EXPOSED COMPONENT ===#

/// When this component is added, a UI system is built
#[derive(Component, Debug, Default, Clone, PartialEq)]
pub struct MainMenuRoute;


// #===============================#
// #=== SANDBOXED USER INTEFACE ===#

/// System that builds the route
fn build_route(mut commands: Commands, assets: Res<AssetCache>, query: Query<Entity, Added<MainMenuRoute>>) {
    for entity in &query {
        // #======================#
        // #=== USER INTERFACE ===#

        // Spawn the master ui tree
        commands.entity(entity).insert((
            UiTreeBundle::<MenuUi>::from(UiTree::new("MainMenu")),
        )).with_children(|ui| {

            // Spawn the root div
            let root = UiLink::<MenuUi>::path("Root");  // Here we can define the name of the node
            ui.spawn((
                root.clone(),                           // Here we add the link
                UiLayout::window_full().pack(),         // This is where we define layout
            ));

            // Spawn the background
            ui.spawn((
                root.add("Background"), // You can see here that we used existing "root" link to create chained link (same as "Root/Background")
                UiLayout::solid().size((2968.0, 1656.0)).scaling(Scaling::Fill).pack(),
                UiImage2dBundle::from(assets.main_background.clone()),  // We use this bundle to add background image to our node
            ));


            // Spawn the board
            let board = root.add("Solid");
            ui.spawn((
                board.clone(),
                UiLayout::solid().size((881.0, 1600.0)).align_x(-0.74).pack(), // Just different layout type that preserves aspect ratio
            ));

            let board = board.add("Board");
            ui.spawn((
                board.clone(),
                UiLayout::window().x(Rl(50.0)).anchor(Anchor::TopCenter).size(Rl(105.0)).pack(),
                UiImage2dBundle::from(assets.main_board.clone())
            ));


            // Spawn the logo
            ui.spawn((
                board.add("Boundary"),
                UiLayout::window().y(Rl(11.0)).size(Rl((105.0, 20.0))).pack(),
            ));
            ui.spawn((
                board.add("Boundary/Logo"),
                UiLayout::solid().size((1240.0, 381.0)).pack(),
                UiImage2dBundle::from(assets.main_logo.clone())
            ));


            // #=========================#
            // #=== MAIN MENU BUTTONS ===#

            // Spawn button boundary
            let list = board.add("List");
            ui.spawn((
                list.clone(),
                UiLayout::window().pos(Rl((22.0, 33.0))).size(Rl((55.0, 34.0))).pack(),
            ));

            // Spawn buttons
            let gap = 3.0;
            let size = 14.0;
            let mut offset = 0.0;
            for button in [
                MainMenuButton::Continue,
                MainMenuButton::NewGame,
                MainMenuButton::LoadGame,
                MainMenuButton::Settings,
                MainMenuButton::AdditionalContent,
                MainMenuButton::Credits,
                MainMenuButton::QuitGame,
            ] {

                ui.spawn((
                    list.add(button.str()),
                    UiLayout::window().y(Rl(offset)).size(Rl((100.0, size))).pack(),
                    MainButton { text: button.str().into() },
                    button.clone(),
                ));

                offset += gap + size;
            }
        });
    }
}


// #=====================#
// #=== INTERACTIVITY ===#

/// Good practice is to use custom component for buttons, so we can easily know what type of button was pressed
#[derive(Component, Clone)]
enum MainMenuButton {
    Continue,
    NewGame,
    LoadGame,
    Settings,
    AdditionalContent,
    Credits,
    QuitGame,
}
impl MainMenuButton {
    fn str(&self) -> String {
        match self {
            MainMenuButton::Continue => "CONTINUE".into(),
            MainMenuButton::NewGame => "NEW GAME".into(),
            MainMenuButton::LoadGame => "LOAD GAME".into(),
            MainMenuButton::Settings => "SETTINGS".into(),
            MainMenuButton::AdditionalContent => "ADDITIONAL CONTENT".into(),
            MainMenuButton::Credits => "CREDITS".into(),
            MainMenuButton::QuitGame => "QUIT GAME".into(),
        }
    }
}

/// System that will resolve our event
fn main_menu_button_action_system(mut events: EventReader<MainButtonClick>, query: Query<&MainMenuButton, With<MainButton>>, mut exit: EventWriter<bevy::app::AppExit>, mut commands: Commands,
    main_menu_route: Query<Entity, With<MainMenuRoute>>,
) {
    for event in events.read() {
        if let Ok(button) = query.get(event.target) {

            info!("Pressed: {}", button.str());

            // Here we can do our logic for each button
            match button {
                MainMenuButton::Continue => {},
                MainMenuButton::NewGame => {
                    commands.entity(main_menu_route.single()).despawn_recursive();
                    commands.spawn((CharacterCreatorRoute, MovableByCamera));
                },
                MainMenuButton::LoadGame => {
                    commands.entity(main_menu_route.single()).despawn_recursive();
                    commands.spawn((LoadGameRoute, MovableByCamera));
                },
                MainMenuButton::Settings => {
                    commands.entity(main_menu_route.single()).despawn_recursive();
                    commands.spawn((SettingsRoute, MovableByCamera));
                },
                MainMenuButton::AdditionalContent => {},
                MainMenuButton::Credits => {},
                MainMenuButton::QuitGame => {
                    exit.send(bevy::app::AppExit);
                },
            }
        }
    }
}


// #====================#
// #=== ROUTE PLUGIN ===#

/// Plugin adding all our logic
pub struct MainMenuRoutePlugin;
impl Plugin for MainMenuRoutePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, build_route.before(UiSystems::Compute))
            .add_systems(Update, main_menu_button_action_system.run_if(on_event::<MainButtonClick>()));
    }
}

