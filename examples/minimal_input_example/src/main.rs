use ABC_Game_Engine::DeltaTime;
use ABC_Game_Engine::Input;
use ABC_Game_Engine::Scene;
use ABC_Game_Engine::Transform;
use ABC_Game_Engine::Vk;
use ABC_Game_Engine::{EntitiesAndComponents, System};
use ABC_lumenpyx::lights;
use ABC_lumenpyx::primitives::Circle;
use ABC_lumenpyx::LumenpyxProgram;
use ABC_lumenpyx::{render, Camera};

struct InputTestSystem;

impl System for InputTestSystem {
    fn run(&mut self, entities_and_components: &mut EntitiesAndComponents) {
        let input = entities_and_components.get_resource::<Input>().unwrap();

        if input.is_key_pressed(Vk::Escape) {
            println!("exiting...");
            std::process::exit(0);
        }
    }
}

fn main() {
    let mut scene = Scene::new();

    {
        let entities_and_components = &mut scene.world.entities_and_components;

        // make a camera, to specify the position we would like to view everything from
        entities_and_components
            .add_entity_with((Camera::new(), ABC_Game_Engine::Transform::default()));
    }

    scene.world.add_system(InputTestSystem);

    loop {
        scene.world.run();
    }
}
