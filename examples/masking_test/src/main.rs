use std::time::Instant;

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
use ABC_lumenpyx::LumenpyxEventLoop;
use ABC_lumenpyx::LumenpyxProgram;
use ABC_lumenpyx::{render, Camera};

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
                .get_delta_time();

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

        let circle_parent_entity = entities_and_components
            .get_entities_with_component::<EntitiesAndComponents>()
            .next()
            .expect("circle parent not found");

        let circle_parent = entities_and_components
            .get_components_mut::<(EntitiesAndComponents,)>(*circle_parent_entity)
            .0;

        let circle_entity = circle_parent
            .get_entities_with_component::<Circle>()
            .next()
            .expect("circle not found");

        let (transform,) = circle_parent.get_components_mut::<(Transform,)>(*circle_entity);

        transform.x += movement_dir[0] * delta_time * 100.0;
        transform.y += movement_dir[1] * delta_time * 100.0;

        // shouldn't be noticeable, but just to show that rotation isn't doing anything weird
        transform.rotation += delta_time * 100.0;

        //println!("x: {}, y: {}", transform.x, transform.y);
    }
}

struct SceneMoveSystem {
    start_time: Instant,
}

impl System for SceneMoveSystem {
    fn run(&mut self, entities_and_components: &mut EntitiesAndComponents) {
        let time = self.start_time.elapsed().as_secs_f64();
        let x = time.sin() * 100.0;

        let camera_entity = entities_and_components
            .get_entities_with_component::<Camera>()
            .next()
            .expect("camera not found");

        let (transform,) =
            entities_and_components.get_components_mut::<(Transform,)>(*camera_entity);

        transform.x = x;
    }
}

fn main() {
    let mut scene = Scene::new();
    let lumen_event_loop = LumenpyxEventLoop::new(&mut scene.world, [128, 128], "Masking Test");

    {
        let entities_and_components = &mut scene.world.entities_and_components;

        entities_and_components.add_entity_with((
            lights::PointLight::new([1.0, 1.0, 1.0], 1.0, 0.0),
            ABC_Game_Engine::Transform::default(),
        ));

        let background_transform = Transform {
            x: 0.0,
            y: 0.0,
            z: -1.0,
            ..Transform::default()
        };
        entities_and_components.add_entity_with((
            Rectangle::new([0.0, 1.0, 0.0, 1.0], 128.0, 128.0),
            background_transform,
        ));

        let mut children = EntitiesAndComponents::new();

        let circle_transform = Transform {
            x: 0.0,
            y: 0.0,
            z: 1.0,
            ..Transform::default()
        };
        children.add_entity_with((Circle::new([1.0, 1.0, 1.0, 1.0], 5.0), circle_transform));
        children.add_entity_with((
            Rectangle::new([1.0, 1.0, 1.0, 1.0], 128.0, 128.0),
            Transform::default(),
        ));

        entities_and_components.add_entity_with((
            BlendComponent::new(BlendMode::Subtractive),
            ABC_Game_Engine::Transform::default(),
            children,
        ));

        // make a camera, to specify the position we would like to view everything from
        entities_and_components
            .add_entity_with((Camera::new(), ABC_Game_Engine::Transform::default()));
    }

    scene.world.add_system(CircleMovementSystem {});
    scene.world.add_system(SceneMoveSystem {
        start_time: Instant::now(),
    });

    // this is to run the program for forever or until returned
    lumen_event_loop.run(&mut scene.world, |program, world| {
        world.run();
        render(&mut world.entities_and_components, program);
    });
}
