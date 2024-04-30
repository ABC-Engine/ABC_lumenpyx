//! The official ABC Game Engine implementation of lumenpyx

//use lumenpyx::animation::Animation;
mod drawables;
use std::collections::HashSet;
use std::ops::{Deref, DerefMut};

//pub use lumenpyx::*;
use drawables::lights::{AreaLight, DirectionalLight, PointLight};
use drawables::primitives::{
    Animation, AnimationStateMachine, Circle, Cylinder, Rectangle, Sphere, Sprite,
};
pub use drawables::*;
use lumenpyx::draw_all;
use ABC_Game_Engine::{self, World};
use ABC_Game_Engine::{EntitiesAndComponents, Input};
use ABC_Game_Engine::{Entity, KeyCode};

use winit::event::WindowEvent;
use winit::event_loop::EventLoop;

// pub use everything from lumenpyx but exclude the things we override in drawables
pub use lumenpyx::drawable_object::Drawable;
pub use lumenpyx::lights::LightDrawable;
pub use lumenpyx::primitives::Normal;
pub use lumenpyx::primitives::Texture;
pub use lumenpyx::Transform;

pub struct LumenpyxProgram {
    pub program: lumenpyx::LumenpyxProgram,
    keys_down: HashSet<KeyCode>,
}

impl LumenpyxProgram {
    pub fn new(resolution: [u32; 2], name: &str) -> (Self, EventLoop<()>) {
        let (program, event_loop) = lumenpyx::LumenpyxProgram::new(resolution, name);
        let keys_down = HashSet::new();

        (Self { program, keys_down }, event_loop)
    }

    /// run the program with the given update function
    pub fn run<F>(&mut self, event_loop: EventLoop<()>, world: &mut World, mut update: F)
    where
        F: FnMut(&mut Self, &mut World),
    {
        event_loop
            .run(move |ev, window_target| match ev {
                winit::event::Event::WindowEvent { event, .. } => match event {
                    winit::event::WindowEvent::CloseRequested => {
                        window_target.exit();
                    }
                    winit::event::WindowEvent::Resized(physical_size) => {
                        self.display.resize(physical_size.into());
                    }
                    winit::event::WindowEvent::RedrawRequested => {
                        {
                            let input = world
                                .entities_and_components
                                .get_resource_mut::<Input>()
                                .expect("failed to get input system");

                            input.clear_key_states();
                            for key in self.keys_down.iter() {
                                input.set_key_state(*key);
                            }
                            input.advance_frame();
                        }
                        update(self, world);
                    }
                    winit::event::WindowEvent::KeyboardInput { event, .. } => {
                        if event.state == winit::event::ElementState::Pressed {
                            // turn the key event into a key enum in winit
                            match event.physical_key {
                                winit::keyboard::PhysicalKey::Code(code) => {
                                    let key = winit_input_to_abc_input(code);
                                    if let Some(key) = key {
                                        self.keys_down.insert(key);
                                    }
                                }
                                // maybe we should log something here, once we have a logger...
                                // for now, we just ignore it.
                                winit::keyboard::PhysicalKey::Unidentified(_) => (),
                            }
                        } else if event.state == winit::event::ElementState::Released {
                            // turn the key event into a key enum in winit
                            match event.physical_key {
                                winit::keyboard::PhysicalKey::Code(code) => {
                                    let key = winit_input_to_abc_input(code);
                                    if let Some(key) = key {
                                        self.keys_down.remove(&key);
                                    }
                                }
                                // maybe we should log something here, once we have a logger...
                                // for now, we just ignore it.
                                winit::keyboard::PhysicalKey::Unidentified(_) => (),
                            }
                        }
                    }
                    _ => (),
                },
                winit::event::Event::AboutToWait => {
                    // RedrawRequested will only when we resize the window, so we need to manually
                    // request it.
                    self.window.request_redraw();
                }
                _ => (),
            })
            .expect("Failed to run event loop");
    }
}

impl Deref for LumenpyxProgram {
    type Target = lumenpyx::LumenpyxProgram;

    fn deref(&self) -> &Self::Target {
        &self.program
    }
}

impl DerefMut for LumenpyxProgram {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.program
    }
}

fn winit_input_to_abc_input(
    input: winit::keyboard::KeyCode,
) -> Option<ABC_Game_Engine::input::KeyCode> {
    // convert the key event to just the key
    match input {
        winit::keyboard::KeyCode::Digit0 => Some(ABC_Game_Engine::input::KeyCode::Key0),
        winit::keyboard::KeyCode::Digit1 => Some(ABC_Game_Engine::input::KeyCode::Key1),
        winit::keyboard::KeyCode::Digit2 => Some(ABC_Game_Engine::input::KeyCode::Key2),
        winit::keyboard::KeyCode::Digit3 => Some(ABC_Game_Engine::input::KeyCode::Key3),
        winit::keyboard::KeyCode::Digit4 => Some(ABC_Game_Engine::input::KeyCode::Key4),
        winit::keyboard::KeyCode::Digit5 => Some(ABC_Game_Engine::input::KeyCode::Key5),
        winit::keyboard::KeyCode::Digit6 => Some(ABC_Game_Engine::input::KeyCode::Key6),
        winit::keyboard::KeyCode::Digit7 => Some(ABC_Game_Engine::input::KeyCode::Key7),
        winit::keyboard::KeyCode::Digit8 => Some(ABC_Game_Engine::input::KeyCode::Key8),
        winit::keyboard::KeyCode::Digit9 => Some(ABC_Game_Engine::input::KeyCode::Key9),
        winit::keyboard::KeyCode::KeyA => Some(ABC_Game_Engine::input::KeyCode::A),
        winit::keyboard::KeyCode::KeyB => Some(ABC_Game_Engine::input::KeyCode::B),
        winit::keyboard::KeyCode::KeyC => Some(ABC_Game_Engine::input::KeyCode::C),
        winit::keyboard::KeyCode::KeyD => Some(ABC_Game_Engine::input::KeyCode::D),
        winit::keyboard::KeyCode::KeyE => Some(ABC_Game_Engine::input::KeyCode::E),
        winit::keyboard::KeyCode::KeyF => Some(ABC_Game_Engine::input::KeyCode::F),
        winit::keyboard::KeyCode::KeyG => Some(ABC_Game_Engine::input::KeyCode::G),
        winit::keyboard::KeyCode::KeyH => Some(ABC_Game_Engine::input::KeyCode::H),
        winit::keyboard::KeyCode::KeyI => Some(ABC_Game_Engine::input::KeyCode::I),
        winit::keyboard::KeyCode::KeyJ => Some(ABC_Game_Engine::input::KeyCode::J),
        winit::keyboard::KeyCode::KeyK => Some(ABC_Game_Engine::input::KeyCode::K),
        winit::keyboard::KeyCode::KeyL => Some(ABC_Game_Engine::input::KeyCode::L),
        winit::keyboard::KeyCode::KeyM => Some(ABC_Game_Engine::input::KeyCode::M),
        winit::keyboard::KeyCode::KeyN => Some(ABC_Game_Engine::input::KeyCode::N),
        winit::keyboard::KeyCode::KeyO => Some(ABC_Game_Engine::input::KeyCode::O),
        winit::keyboard::KeyCode::KeyP => Some(ABC_Game_Engine::input::KeyCode::P),
        winit::keyboard::KeyCode::KeyQ => Some(ABC_Game_Engine::input::KeyCode::Q),
        winit::keyboard::KeyCode::KeyR => Some(ABC_Game_Engine::input::KeyCode::R),
        winit::keyboard::KeyCode::KeyS => Some(ABC_Game_Engine::input::KeyCode::S),
        winit::keyboard::KeyCode::KeyT => Some(ABC_Game_Engine::input::KeyCode::T),
        winit::keyboard::KeyCode::KeyU => Some(ABC_Game_Engine::input::KeyCode::U),
        winit::keyboard::KeyCode::KeyV => Some(ABC_Game_Engine::input::KeyCode::V),
        winit::keyboard::KeyCode::KeyW => Some(ABC_Game_Engine::input::KeyCode::W),
        winit::keyboard::KeyCode::KeyX => Some(ABC_Game_Engine::input::KeyCode::X),
        winit::keyboard::KeyCode::KeyY => Some(ABC_Game_Engine::input::KeyCode::Y),
        winit::keyboard::KeyCode::KeyZ => Some(ABC_Game_Engine::input::KeyCode::Z),
        winit::keyboard::KeyCode::Escape => Some(ABC_Game_Engine::input::KeyCode::Escape),
        winit::keyboard::KeyCode::F1 => Some(ABC_Game_Engine::input::KeyCode::F1),
        winit::keyboard::KeyCode::F2 => Some(ABC_Game_Engine::input::KeyCode::F2),
        winit::keyboard::KeyCode::F3 => Some(ABC_Game_Engine::input::KeyCode::F3),
        winit::keyboard::KeyCode::F4 => Some(ABC_Game_Engine::input::KeyCode::F4),
        winit::keyboard::KeyCode::F5 => Some(ABC_Game_Engine::input::KeyCode::F5),
        winit::keyboard::KeyCode::F6 => Some(ABC_Game_Engine::input::KeyCode::F6),
        winit::keyboard::KeyCode::F7 => Some(ABC_Game_Engine::input::KeyCode::F7),
        winit::keyboard::KeyCode::F8 => Some(ABC_Game_Engine::input::KeyCode::F8),
        winit::keyboard::KeyCode::F9 => Some(ABC_Game_Engine::input::KeyCode::F9),
        winit::keyboard::KeyCode::F10 => Some(ABC_Game_Engine::input::KeyCode::F10),
        winit::keyboard::KeyCode::F11 => Some(ABC_Game_Engine::input::KeyCode::F11),
        winit::keyboard::KeyCode::F12 => Some(ABC_Game_Engine::input::KeyCode::F12),
        winit::keyboard::KeyCode::F13 => Some(ABC_Game_Engine::input::KeyCode::F13),
        winit::keyboard::KeyCode::F14 => Some(ABC_Game_Engine::input::KeyCode::F14),
        winit::keyboard::KeyCode::F15 => Some(ABC_Game_Engine::input::KeyCode::F15),
        winit::keyboard::KeyCode::F16 => Some(ABC_Game_Engine::input::KeyCode::F16),
        winit::keyboard::KeyCode::F17 => Some(ABC_Game_Engine::input::KeyCode::F17),
        winit::keyboard::KeyCode::F18 => Some(ABC_Game_Engine::input::KeyCode::F18),
        winit::keyboard::KeyCode::F19 => Some(ABC_Game_Engine::input::KeyCode::F19),
        winit::keyboard::KeyCode::F20 => Some(ABC_Game_Engine::input::KeyCode::F20),
        winit::keyboard::KeyCode::F21 => Some(ABC_Game_Engine::input::KeyCode::F21),
        winit::keyboard::KeyCode::F22 => Some(ABC_Game_Engine::input::KeyCode::F22),
        winit::keyboard::KeyCode::F23 => Some(ABC_Game_Engine::input::KeyCode::F23),
        winit::keyboard::KeyCode::F24 => Some(ABC_Game_Engine::input::KeyCode::F24),
        winit::keyboard::KeyCode::PrintScreen => Some(ABC_Game_Engine::input::KeyCode::Snapshot),
        winit::keyboard::KeyCode::ScrollLock => Some(ABC_Game_Engine::input::KeyCode::Scroll),
        winit::keyboard::KeyCode::Pause => Some(ABC_Game_Engine::input::KeyCode::Pause),
        winit::keyboard::KeyCode::Insert => Some(ABC_Game_Engine::input::KeyCode::Insert),
        winit::keyboard::KeyCode::Home => Some(ABC_Game_Engine::input::KeyCode::Home),
        winit::keyboard::KeyCode::Delete => Some(ABC_Game_Engine::input::KeyCode::Delete),
        winit::keyboard::KeyCode::End => Some(ABC_Game_Engine::input::KeyCode::End),
        winit::keyboard::KeyCode::PageDown => Some(ABC_Game_Engine::input::KeyCode::PageDown),
        winit::keyboard::KeyCode::PageUp => Some(ABC_Game_Engine::input::KeyCode::PageUp),
        winit::keyboard::KeyCode::ArrowLeft => Some(ABC_Game_Engine::input::KeyCode::Left),
        winit::keyboard::KeyCode::ArrowUp => Some(ABC_Game_Engine::input::KeyCode::Up),
        winit::keyboard::KeyCode::ArrowRight => Some(ABC_Game_Engine::input::KeyCode::Right),
        winit::keyboard::KeyCode::ArrowDown => Some(ABC_Game_Engine::input::KeyCode::Down),
        winit::keyboard::KeyCode::Backspace => Some(ABC_Game_Engine::input::KeyCode::Backspace),
        winit::keyboard::KeyCode::Enter => Some(ABC_Game_Engine::input::KeyCode::Return),
        winit::keyboard::KeyCode::Space => Some(ABC_Game_Engine::input::KeyCode::Space),
        winit::keyboard::KeyCode::Comma => Some(ABC_Game_Engine::input::KeyCode::Comma),
        winit::keyboard::KeyCode::Minus => Some(ABC_Game_Engine::input::KeyCode::Minus),
        winit::keyboard::KeyCode::Period => Some(ABC_Game_Engine::input::KeyCode::Period),
        winit::keyboard::KeyCode::Slash => Some(ABC_Game_Engine::input::KeyCode::Slash),
        winit::keyboard::KeyCode::Semicolon => Some(ABC_Game_Engine::input::KeyCode::Semicolon),
        winit::keyboard::KeyCode::Equal => Some(ABC_Game_Engine::input::KeyCode::Equals),
        winit::keyboard::KeyCode::Quote => Some(ABC_Game_Engine::input::KeyCode::Apostrophe),
        winit::keyboard::KeyCode::Backslash => Some(ABC_Game_Engine::input::KeyCode::Backslash),
        winit::keyboard::KeyCode::BracketLeft => Some(ABC_Game_Engine::input::KeyCode::LBracket),
        winit::keyboard::KeyCode::BracketRight => Some(ABC_Game_Engine::input::KeyCode::RBracket),
        winit::keyboard::KeyCode::Backquote => Some(ABC_Game_Engine::input::KeyCode::Grave),
        winit::keyboard::KeyCode::ControlLeft => Some(ABC_Game_Engine::input::KeyCode::LControl),
        winit::keyboard::KeyCode::ShiftLeft => Some(ABC_Game_Engine::input::KeyCode::LShift),
        winit::keyboard::KeyCode::AltLeft => Some(ABC_Game_Engine::input::KeyCode::LAlt),
        winit::keyboard::KeyCode::ControlRight => Some(ABC_Game_Engine::input::KeyCode::RControl),
        winit::keyboard::KeyCode::ShiftRight => Some(ABC_Game_Engine::input::KeyCode::RShift),
        winit::keyboard::KeyCode::AltRight => Some(ABC_Game_Engine::input::KeyCode::RAlt),
        winit::keyboard::KeyCode::NumLock => Some(ABC_Game_Engine::input::KeyCode::Numlock),
        winit::keyboard::KeyCode::Numpad0 => Some(ABC_Game_Engine::input::KeyCode::Numpad0),
        winit::keyboard::KeyCode::Numpad1 => Some(ABC_Game_Engine::input::KeyCode::Numpad1),
        winit::keyboard::KeyCode::Numpad2 => Some(ABC_Game_Engine::input::KeyCode::Numpad2),
        winit::keyboard::KeyCode::Numpad3 => Some(ABC_Game_Engine::input::KeyCode::Numpad3),
        winit::keyboard::KeyCode::Numpad4 => Some(ABC_Game_Engine::input::KeyCode::Numpad4),
        winit::keyboard::KeyCode::Numpad5 => Some(ABC_Game_Engine::input::KeyCode::Numpad5),
        winit::keyboard::KeyCode::Numpad6 => Some(ABC_Game_Engine::input::KeyCode::Numpad6),
        winit::keyboard::KeyCode::Numpad7 => Some(ABC_Game_Engine::input::KeyCode::Numpad7),
        winit::keyboard::KeyCode::Numpad8 => Some(ABC_Game_Engine::input::KeyCode::Numpad8),
        winit::keyboard::KeyCode::Numpad9 => Some(ABC_Game_Engine::input::KeyCode::Numpad9),
        winit::keyboard::KeyCode::NumpadAdd => Some(ABC_Game_Engine::input::KeyCode::NumpadAdd),
        winit::keyboard::KeyCode::NumpadDecimal => {
            Some(ABC_Game_Engine::input::KeyCode::NumpadDecimal)
        }
        winit::keyboard::KeyCode::NumpadDivide => {
            Some(ABC_Game_Engine::input::KeyCode::NumpadDivide)
        }
        winit::keyboard::KeyCode::NumpadEnter => Some(ABC_Game_Engine::input::KeyCode::NumpadEnter),
        winit::keyboard::KeyCode::NumpadEqual => {
            Some(ABC_Game_Engine::input::KeyCode::NumpadEquals)
        }
        winit::keyboard::KeyCode::NumpadMultiply => {
            Some(ABC_Game_Engine::input::KeyCode::NumpadMultiply)
        }
        winit::keyboard::KeyCode::NumpadSubtract => {
            Some(ABC_Game_Engine::input::KeyCode::NumpadSubtract)
        }
        winit::keyboard::KeyCode::CapsLock => Some(ABC_Game_Engine::input::KeyCode::Capital),
        _ => None, // we don't care about the rest of the keys for now, if we are missing a key, file an issue
    }
}

#[derive(Clone, Copy)]
pub struct Camera {
    lumen_camera: lumenpyx::Camera,
    is_active: bool,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            lumen_camera: lumenpyx::Camera::new([0.0, 0.0, 0.0]), // should be set by the transform in the ecs
            is_active: true,
        }
    }
}

///  Renders the scene
pub fn render(scene: &mut EntitiesAndComponents, program: &mut LumenpyxProgram) {
    let camera_entities = scene
        .get_entities_with_component::<Camera>()
        .cloned()
        .collect::<Vec<Entity>>();

    if camera_entities.len() == 0 {
        panic!("renderer could not find a camera");
    } else {
        // this will not panic if no active camera is found
        for camera_entity in camera_entities {
            let mut camera_component: Camera;
            {
                let camera_component_ref = scene
                    .try_get_component::<Camera>(camera_entity)
                    .expect("renderer could not find a camera");
                camera_component = (&**camera_component_ref).clone();
            }

            if camera_component.is_active {
                let camera_transform = scene
                    .try_get_components::<(ABC_Game_Engine::Transform,)>(camera_entity)
                    .0
                    .expect("active camera does not have a transform!");

                camera_component.lumen_camera.position = [
                    camera_transform.x as f32,
                    camera_transform.y as f32,
                    camera_transform.z as f32,
                ];

                render_objects(scene, &camera_component.lumen_camera, program);
                break;
            }
        }
    }
}

fn get_all_lights_on_object_mut(
    entities_and_components: &mut EntitiesAndComponents,
    entity: Entity,
) -> (
    Vec<&mut dyn LightDrawable>,
    Option<&mut ABC_Game_Engine::Transform>,
) {
    let (point_light, spot_light, directional_light, transform) = entities_and_components
        .try_get_components_mut::<(
            PointLight,
            AreaLight,
            DirectionalLight,
            ABC_Game_Engine::Transform,
        )>(entity);

    let mut lights = vec![];
    match point_light {
        Some(point_light) => lights.push(point_light as &mut dyn LightDrawable),
        None => (),
    }
    match spot_light {
        Some(spot_light) => lights.push(spot_light as &mut dyn LightDrawable),
        None => (),
    }
    match directional_light {
        Some(directional_light) => lights.push(directional_light as &mut dyn LightDrawable),
        None => (),
    }

    (lights, transform)
}

fn get_all_drawables_on_object_mut(
    entities_and_components: &mut EntitiesAndComponents,
    entity: Entity,
) -> (
    Vec<&mut dyn Drawable>,
    Option<&mut ABC_Game_Engine::Transform>,
) {
    let (
        circle,
        rectangle,
        sprite,
        sphere,
        animation,
        cylinder,
        animation_state_machine,
        transform,
    ) = entities_and_components.try_get_components_mut::<(
        Circle,
        Rectangle,
        Sprite,
        Sphere,
        Animation,
        Cylinder,
        AnimationStateMachine,
        ABC_Game_Engine::Transform,
    )>(entity);

    // this is abhorrent, but I can't think of any better way to do this
    let mut drawables = vec![];
    match circle {
        Some(circle) => drawables.push(circle as &mut dyn Drawable),
        None => (),
    }
    match rectangle {
        Some(rectangle) => drawables.push(rectangle as &mut dyn Drawable),
        None => (),
    }
    match sprite {
        Some(sprite) => drawables.push(sprite as &mut dyn Drawable),
        None => (),
    }
    match sphere {
        Some(sphere) => drawables.push(sphere as &mut dyn Drawable),
        None => (),
    }
    match animation {
        Some(animation) => drawables.push(animation as &mut dyn Drawable),
        None => (),
    }
    match cylinder {
        Some(cylinder) => drawables.push(cylinder as &mut dyn Drawable),
        None => (),
    }
    match animation_state_machine {
        Some(animation_state_machine) => {
            drawables.push(animation_state_machine as &mut dyn Drawable)
        }
        None => (),
    }

    (drawables, transform)
}

fn get_all_entities_with_drawables(entities_and_components: &EntitiesAndComponents) -> Vec<Entity> {
    // get all entities with sprite, circle, rectangle, sphere, animation, cylinder then get rid of the duplicates
    let mut entities = vec![];

    let entities_with_sprite = entities_and_components
        .get_entities_with_component::<Sprite>()
        .cloned()
        .collect::<Vec<Entity>>();
    let entities_with_circle = entities_and_components
        .get_entities_with_component::<Circle>()
        .cloned()
        .collect::<Vec<Entity>>();
    let entities_with_rectangle = entities_and_components
        .get_entities_with_component::<Rectangle>()
        .cloned()
        .collect::<Vec<Entity>>();
    let entities_with_sphere = entities_and_components
        .get_entities_with_component::<Sphere>()
        .cloned()
        .collect::<Vec<Entity>>();
    let entities_with_animation = entities_and_components
        .get_entities_with_component::<Animation>()
        .cloned()
        .collect::<Vec<Entity>>();
    let entities_with_cylinder = entities_and_components
        .get_entities_with_component::<Cylinder>()
        .cloned()
        .collect::<Vec<Entity>>();

    // lights are counted as drawables in this case

    let entities_with_point_light = entities_and_components
        .get_entities_with_component::<PointLight>()
        .cloned()
        .collect::<Vec<Entity>>();
    let entities_with_area_light = entities_and_components
        .get_entities_with_component::<AreaLight>()
        .cloned()
        .collect::<Vec<Entity>>();
    let entities_with_directional_light = entities_and_components
        .get_entities_with_component::<DirectionalLight>()
        .cloned()
        .collect::<Vec<Entity>>();

    entities.extend(entities_with_sprite);
    entities.extend(entities_with_circle);
    entities.extend(entities_with_rectangle);
    entities.extend(entities_with_sphere);
    entities.extend(entities_with_animation);
    entities.extend(entities_with_cylinder);
    entities.extend(entities_with_point_light);
    entities.extend(entities_with_area_light);
    entities.extend(entities_with_directional_light);

    // remove duplicates
    entities.dedup();

    entities
}

fn render_objects(
    entities_and_components: &mut EntitiesAndComponents,
    camera: &lumenpyx::Camera,
    lumen_program: &mut LumenpyxProgram,
) {
    let mut entity_depth_array = vec![];

    collect_renderable_entities(
        &entities_and_components,
        vec![],
        &ABC_Game_Engine::Transform::default(),
        &mut entity_depth_array,
    );

    entity_depth_array.sort();
    let mut sprites = vec![];
    let mut lights_in_scene = vec![];

    let entities_and_components_ptr = entities_and_components as *mut EntitiesAndComponents;
    // could possibly be done multithreaded and combine layers afterward
    for entity_depth_item in entity_depth_array {
        let entities = entity_depth_item.entity;

        // SAFETY: the only reason this is a problem is because of the sprites vector,
        // which is only used after the loop so there is no double mutable borrow
        let (current_entities_and_components, entity) =
            get_entities_and_components_from_entity_list(
                unsafe { &mut *entities_and_components_ptr },
                entities,
            );

        let current_entities_and_components_ptr =
            current_entities_and_components as *mut EntitiesAndComponents;

        // SAFETY: THis is safe as long as we don't use sprites or lights_in scene before the loop ends
        // AND the types of get_all_lights_on_object_mut and get_all_drawables_on_object_mut are non overlapping
        let (lights, _) = get_all_lights_on_object_mut(
            unsafe { &mut *current_entities_and_components_ptr },
            entity,
        );

        let (drawables, transform) =
            get_all_drawables_on_object_mut(current_entities_and_components, entity);
        {
            if let Some(_transform) = transform {
                let transform = &(entity_depth_item.transform);

                for drawable in drawables {
                    drawable.set_transform(abc_transform_to_lumen_transform(transform.clone()));
                    sprites.push(&*drawable);
                }

                for light in lights {
                    light.set_transform(abc_transform_to_lumen_transform(transform.clone()));
                    lights_in_scene.push(&*light);
                }
            }
        }
    }

    // for now, we will only have one light
    draw_all(lights_in_scene, sprites, lumen_program, camera)
}

/// A recursive function that collects all renderable entities in the scene
fn collect_renderable_entities(
    entities_and_components: &EntitiesAndComponents,
    // the list of parent entities to get to the EntitiesAndComponents that is passed, starting with the root
    parent_entities: Vec<Entity>,
    transform_offset: &ABC_Game_Engine::Transform,
    out_list: &mut Vec<EntityDepthItem>,
) {
    let entities_with_drawables = get_all_entities_with_drawables(entities_and_components);

    for entity in entities_with_drawables {
        let (transform,) =
            entities_and_components.try_get_components::<(ABC_Game_Engine::Transform,)>(entity);

        if let Some(transform) = transform {
            let mut new_parents = parent_entities.clone();
            new_parents.push(entity);
            let new_transform = &*transform + transform_offset;
            out_list.push(EntityDepthItem {
                entity: new_parents,
                transform: new_transform,
            });
        }
    }

    let entities_with_children = entities_and_components
        .get_entities_with_component::<EntitiesAndComponents>()
        .cloned()
        .collect::<Vec<Entity>>();

    for entity in entities_with_children {
        let (transform, children) = entities_and_components
            .try_get_components::<(ABC_Game_Engine::Transform, EntitiesAndComponents)>(entity);

        match (transform, children) {
            (Some(transform), Some(children)) => {
                let mut new_parents = parent_entities.clone();
                new_parents.push(entity);
                collect_renderable_entities(
                    children,
                    new_parents,
                    &(transform_offset + transform),
                    out_list,
                )
            }
            (None, Some(children)) => {
                let mut new_parents = parent_entities.clone();
                new_parents.push(entity);
                collect_renderable_entities(children, new_parents, transform_offset, out_list)
            }
            _ => (),
        }
    }
}

/// takes a Vec<Entity> and returns the EntitiesAndComponents and Entity that it points to
fn get_entities_and_components_from_entity_list(
    entities_and_components: &mut EntitiesAndComponents,
    mut entity_list: Vec<Entity>,
) -> (&mut EntitiesAndComponents, Entity) {
    if entity_list.len() == 0 {
        panic!("entity list is empty, this should never happen, please report this as a bug");
    }
    if entity_list.len() == 1 {
        return (entities_and_components, entity_list[0]);
    }

    let mut current_entities_and_components = entities_and_components;
    let mut current_entity = entity_list[0];
    // the last entity in the list is the one we want to return, and it's not a parent so no need to check for children
    let last_entity = entity_list.pop().unwrap();

    for entity in entity_list {
        let children = current_entities_and_components
            .try_get_components_mut::<(EntitiesAndComponents,)>(current_entity)
            .0
            .expect(
                "failed to get children, this should never happen, please report this as a bug",
            );

        current_entities_and_components = children;
        current_entity = entity;
    }
    (current_entities_and_components, last_entity)
}

struct EntityDepthItem {
    /// ordered by child depth, so entity1 has entity2 as a child which has entity3 as a child
    /// entity1 will not be rendered as part of the pass for this object just entity3.
    /// entity1 and entity 2 will have its own pass
    entity: Vec<Entity>,
    transform: ABC_Game_Engine::Transform,
}

impl Eq for EntityDepthItem {}

impl PartialEq for EntityDepthItem {
    fn eq(&self, other: &Self) -> bool {
        self.transform.z == other.transform.z
    }
}

impl PartialOrd for EntityDepthItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.transform.z.partial_cmp(&other.transform.z)
    }
}

impl Ord for EntityDepthItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.transform
            .z
            .partial_cmp(&other.transform.z)
            .expect("failed to compare entity depth")
    }
}

fn abc_transform_to_lumen_transform(transform: ABC_Game_Engine::Transform) -> Transform {
    let mut lumen_transform = Transform::new([0.0, 0.0, 0.0]);
    lumen_transform.set_x(transform.x as f32);
    lumen_transform.set_y(transform.y as f32);
    lumen_transform.set_rotation(transform.rotation as f32);
    lumen_transform.set_scale(transform.scale as f32, transform.scale as f32, 1.0);
    lumen_transform
}
