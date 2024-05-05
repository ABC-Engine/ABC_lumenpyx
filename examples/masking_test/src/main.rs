use ABC_Game_Engine::DeltaTime;
use ABC_Game_Engine::Input;
use ABC_Game_Engine::Scene;
use ABC_Game_Engine::Transform;
use ABC_Game_Engine::{EntitiesAndComponents, System};
use ABC_Game_Engine::{KeyCode, KeyState};
use ABC_lumenpyx::lights;
use ABC_lumenpyx::primitives::BlendComponent;
use ABC_lumenpyx::primitives::Circle;
use ABC_lumenpyx::primitives::Rectangle;
use ABC_lumenpyx::BlendMode;
use ABC_lumenpyx::LumenpyxProgram;
use ABC_lumenpyx::{render, Camera};

struct CameraMovementSystem;

impl System for CameraMovementSystem {
    fn run(&mut self, entities_and_components: &mut EntitiesAndComponents) {
        let camera_entity = entities_and_components
            .get_entities_with_component::<Camera>()
            .next();

        let (transform,) = entities_and_components
            .get_components_mut::<(Transform,)>(*camera_entity.expect("camera not found"));

        transform.x += 0.01;
    }
}

struct CircleMovementSystem;

impl System for CircleMovementSystem {
    fn run(&mut self, entities_and_components: &mut EntitiesAndComponents) {
        let mut movement_dir: [f64; 2] = [0.0, 0.0];
        let delta_time;
        {
            let input = entities_and_components.get_resource::<Input>().unwrap();
            delta_time = entities_and_components
                .get_resource::<DeltaTime>()
                .unwrap()
                .delta_time;

            if input.get_key_state(KeyCode::W) == KeyState::Pressed {
                movement_dir[1] += 1.0;
            }
            if input.get_key_state(KeyCode::S) == KeyState::Pressed {
                movement_dir[1] += -1.0;
            }
            if input.get_key_state(KeyCode::A) == KeyState::Pressed {
                movement_dir[0] += -1.0;
            }
            if input.get_key_state(KeyCode::D) == KeyState::Pressed {
                movement_dir[0] += 1.0;
            }

            let magnitude = (movement_dir[0].powi(2) + movement_dir[1].powi(2)).sqrt();

            if magnitude != 0.0 {
                movement_dir[0] /= magnitude;
                movement_dir[1] /= magnitude;
            }
        }

        let circle_entity = entities_and_components
            .get_entities_with_component::<Circle>()
            .next();

        let (transform,) = entities_and_components
            .get_components_mut::<(Transform,)>(*circle_entity.expect("circle not found"));

        transform.x += movement_dir[0] * delta_time * 100.0;
        transform.y += movement_dir[1] * delta_time * 100.0;
    }
}

fn main() {
    let mut scene = Scene::new();

    let (mut lumen_program, event_loop) =
        LumenpyxProgram::new([(128.0 * (16.0 / 9.0)) as u32, 128], "name of your program");

    {
        let entities_and_components = &mut scene.world.entities_and_components;

        entities_and_components.add_entity_with((
            lights::PointLight::new([1.0, 1.0, 1.0], 1.0, 0.01),
            ABC_Game_Engine::Transform::default(),
        ));

        entities_and_components.add_entity_with((
            Circle::new([1.0, 1.0, 1.0, 1.0], 5.0),
            Rectangle::new([1.0, 1.0, 1.0, 1.0], 20.0, 20.0),
            BlendComponent::new(BlendMode::Subtractive, true),
            ABC_Game_Engine::Transform::default(),
        ));

        // make a camera, to specify the position we would like to view everything from
        entities_and_components
            .add_entity_with((Camera::new(), ABC_Game_Engine::Transform::default()));
    }

    //scene.world.add_system(CameraMovementSystem);
    scene.world.add_system(CircleMovementSystem);

    // this is to run the program for forever or until returned
    lumen_program.run(event_loop, &mut scene.world, |program, world| {
        world.run();
        render(&mut world.entities_and_components, program);
    });
}
