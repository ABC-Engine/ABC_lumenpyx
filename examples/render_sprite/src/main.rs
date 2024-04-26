use ABC_Game_Engine::Scene;
use ABC_Game_Engine::Transform;
use ABC_Game_Engine::{EntitiesAndComponents, System};
use ABC_lumenpyx::lights;
use ABC_lumenpyx::primitives::Circle;
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

fn main() {
    let mut scene = Scene::new();

    let (mut lumen_program, event_loop) =
        LumenpyxProgram::new([(128.0 * (16.0 / 9.0)) as u32, 128], "name of your program");

    {
        let entities_and_components = &mut scene.world.entities_and_components;

        entities_and_components.add_entity_with((
            lights::PointLight::new([0.0, 0.0, 1.0], [1.0, 1.0, 1.0], 1.0, 0.01),
            ABC_Game_Engine::Transform::default(),
        ));

        entities_and_components.add_entity_with((
            Circle::new(
                [1.0, 0.0, 0.0, 1.0],
                10.0,
                ABC_lumenpyx::Transform::new([0.0, 0.0, 0.0]),
            ),
            ABC_Game_Engine::Transform::default(),
        ));

        // make a camera, to specify the position we would like to view everything from
        entities_and_components.add_entity_with((
            Camera::new([0.0, 0.0, 0.0]),
            ABC_Game_Engine::Transform::default(),
        ));
    }

    scene.world.add_system(CameraMovementSystem);

    // this is to run the program for forever or until returned
    lumen_program.run(event_loop, |program| {
        render(&mut scene.world.entities_and_components, program);
        scene.world.run();
    });
}
