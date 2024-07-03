//! The official ABC Game Engine implementation of lumenpyx

//use lumenpyx::animation::Animation;
mod drawables;
use std::collections::HashSet;
use std::ops::{Deref, DerefMut};

//pub use lumenpyx::*;
use drawables::lights::{AreaLight, DirectionalLight, PointLight};
use drawables::primitives::{
    Animation, AnimationStateMachine, Circle, Cylinder, Rectangle, Sphere, Sprite, TextBox,
};

pub use drawables::*;
use lumenpyx::draw_all;
use lumenpyx::Transform;
use mouse_position::mouse_position::Mouse;
use winit::dpi::PhysicalPosition;
use ABC_Game_Engine::{self, get_transform, DeltaTime, Resource, World};
use ABC_Game_Engine::{EntitiesAndComponents, Input};
use ABC_Game_Engine::{Entity, KeyCode};

use winit::event::Event::DeviceEvent;
use winit::event_loop::EventLoop;

// pub use everything from lumenpyx but exclude the things we override in drawables
pub use lumenpyx::blending::BlendMode;
pub use lumenpyx::blending::BlendObject;
pub use lumenpyx::drawable_object::Drawable;
pub use lumenpyx::lights::LightDrawable;
pub use lumenpyx::primitives::Normal;
pub use lumenpyx::primitives::Texture;
pub use lumenpyx::text::{FontFamily, FontStack, GenericFamily};
pub use lumenpyx::DebugOption;
pub use lumenpyx::RenderSettings;
pub use lumenpyx::TextureHandle;

use crate::primitives::{BlendComponent, LumenBlendObject};

pub struct LumenpyxProgram {
    pub internal_program: lumenpyx::LumenpyxProgram,
    keys_down: HashSet<KeyCode>,
}

impl Resource for LumenpyxProgram {
    fn update(&mut self) {}
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// The event loop for the lumenpyx program
pub struct LumenpyxEventLoop {
    event_loop: EventLoop<()>,
}

impl LumenpyxEventLoop {
    /// create a new lumenpyx event loop
    /// puts the lumenpyx program in the world as a resource so it can be accessed by the update function
    pub fn new(world: &mut World, resolution: [u32; 2], name: &str) -> Self {
        let (program, event_loop) = lumenpyx::LumenpyxProgram::new(resolution, name);

        world.entities_and_components.add_resource(LumenpyxProgram {
            internal_program: program,
            keys_down: HashSet::new(),
        });

        Self { event_loop }
    }

    /// set the render settings for the program
    /// this is a convenience function for setting the render settings
    pub fn set_render_settings(&mut self, world: &mut World, settings: RenderSettings) {
        let lumen_program = world
            .entities_and_components
            .get_resource_mut::<LumenpyxProgram>()
            .expect("failed to get lumen program");

        lumen_program.internal_program.set_render_settings(settings);
    }

    /// set the debug options for the program
    /// this is a convenience function for setting the debug options
    pub fn set_debug(&mut self, world: &mut World, options: DebugOption) {
        let lumen_program = world
            .entities_and_components
            .get_resource_mut::<LumenpyxProgram>()
            .expect("failed to get lumen program");

        lumen_program.internal_program.set_debug(options);
    }

    /// set the name of the window
    /// this is a convenience function for setting the name of the window
    pub fn set_name(&mut self, world: &mut World, name: &str) {
        let lumen_program = world
            .entities_and_components
            .get_resource_mut::<LumenpyxProgram>()
            .expect("failed to get lumen program");

        lumen_program.internal_program.set_name(name);
    }

    /// set the resolution of the window
    /// this is a convenience function for setting the resolution of the window
    pub fn set_resolution(&mut self, world: &mut World, resolution: [u32; 2]) {
        let lumen_program = world
            .entities_and_components
            .get_resource_mut::<LumenpyxProgram>()
            .expect("failed to get lumen program");

        lumen_program.internal_program.set_resolution(resolution);
    }

    /// get the resolution of the window
    /// this is a convenience function for getting the resolution of the window
    /// returns the resolution as a [u32; 2]
    pub fn get_resolution(&self, world: &World) -> [u32; 2] {
        let lumen_program = world
            .entities_and_components
            .get_resource::<LumenpyxProgram>()
            .expect("failed to get lumen program");

        lumen_program.internal_program.get_resolution()
    }

    /// run the program with the given update function
    pub fn run<F>(self, world: &mut World, mut update: F)
    where
        F: FnMut(&mut World),
    {
        self.event_loop
            .run(move |ev, window_target| match ev {
                winit::event::Event::WindowEvent { event, .. } => match event {
                    winit::event::WindowEvent::Focused(_) => {
                        let input = world
                            .entities_and_components
                            .get_resource_mut::<Input>()
                            .expect("failed to get input system probably a version mismatch");

                        input.clear_mouse_states();

                        input.clear_key_states();
                    }
                    winit::event::WindowEvent::CloseRequested => {
                        window_target.exit();
                    }
                    winit::event::WindowEvent::Resized(physical_size) => {
                        let lumen_program = world
                            .entities_and_components
                            .get_resource_mut::<LumenpyxProgram>()
                            .expect("failed to get lumen program");
                        lumen_program.display.resize(physical_size.into());
                    }
                    winit::event::WindowEvent::RedrawRequested => {
                        let keys_down;
                        {
                            let lumen_program = world
                                .entities_and_components
                                .get_resource::<LumenpyxProgram>()
                                .expect("failed to get lumen program");

                            keys_down = lumen_program.keys_down.clone();
                        }

                        {
                            let input = world
                                .entities_and_components
                                .get_resource_mut::<Input>()
                                .expect("failed to get input system probably a version mismatch");

                            input.clear_key_states();
                            for key in keys_down.iter() {
                                input.set_key_down(*key);
                            }
                        }

                        update_mouse_pos(world);

                        update(world);
                    }
                    winit::event::WindowEvent::KeyboardInput { event, .. } => {
                        let lumen_program = world
                            .entities_and_components
                            .get_resource_mut::<LumenpyxProgram>()
                            .expect("failed to get lumen program");

                        if event.state == winit::event::ElementState::Pressed {
                            // turn the key event into a key enum in winit
                            match event.physical_key {
                                winit::keyboard::PhysicalKey::Code(code) => {
                                    let key = winit_input_to_abc_input(code);
                                    if let Some(key) = key {
                                        lumen_program.keys_down.insert(key);
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
                                        lumen_program.keys_down.remove(&key);
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

                // mouse events
                DeviceEvent { event, device_id } => match event {
                    // click events
                    winit::event::DeviceEvent::Button { button, state } => {
                        let input = world
                            .entities_and_components
                            .get_resource_mut::<Input>()
                            .expect("failed to get input system probably a version mismatch");

                        input.clear_mouse_states();

                        match button {
                            // left click
                            0 => {
                                if state == winit::event::ElementState::Pressed {
                                    input.set_mouse_down(ABC_Game_Engine::MouseButton::Left);
                                }
                            }
                            // right click
                            1 => {
                                if state == winit::event::ElementState::Pressed {
                                    input.set_mouse_down(ABC_Game_Engine::MouseButton::Right);
                                }
                            }
                            // middle click
                            2 => {
                                if state == winit::event::ElementState::Pressed {
                                    input.set_mouse_down(ABC_Game_Engine::MouseButton::Middle);
                                }
                            }
                            other => {
                                let input = world
                                    .entities_and_components
                                    .get_resource_mut::<Input>()
                                    .expect(
                                        "failed to get input system probably a version mismatch",
                                    );

                                if state == winit::event::ElementState::Pressed {
                                    input
                                        .set_mouse_down(ABC_Game_Engine::MouseButton::Other(other));
                                }
                            }
                        }
                    }
                    _ => (),
                },
                winit::event::Event::AboutToWait => {
                    let lumen_program = world
                        .entities_and_components
                        .get_resource_mut::<LumenpyxProgram>()
                        .expect("failed to get lumen program");

                    // RedrawRequested will only when we resize the window, so we need to manually
                    // request it.
                    lumen_program.window.request_redraw();
                }
                _ => (),
            })
            .expect("Failed to run event loop");
    }
}

fn update_mouse_pos(world: &mut World) {
    let lumen_program = world
        .entities_and_components
        .get_resource::<LumenpyxProgram>()
        .expect("failed to get lumen program");

    let inner_pos = lumen_program
        .window
        .inner_position()
        .unwrap_or(PhysicalPosition::new(0, 0));

    let inner_size = lumen_program.window.inner_size();

    let mouse_position = Mouse::get_mouse_position();

    match mouse_position {
        Mouse::Position { x, y } => {
            let local_x = x - inner_pos.x;
            let local_y = y - inner_pos.y;

            let mut local_x = local_x as f64 / inner_size.width as f64;
            let mut local_y = local_y as f64 / inner_size.height as f64;

            local_x = local_x.clamp(0.0, 1.0);
            local_x -= 0.5;
            local_y = local_y.clamp(0.0, 1.0);
            local_y -= 0.5;

            // now we have the local x and y relative to the window
            // but we need to convert it to the local x and y relative to the world
            // so we need to get the camera and the camera's position

            let camera_entity = get_camera(&world.entities_and_components)
                .expect("failed to get camera for mouse position");

            let resolution = lumen_program.get_dimensions();

            // find the dimension that is cropped
            let width_ratio = resolution[0] as f64 / inner_size.width as f64;
            let height_ratio = resolution[1] as f64 / inner_size.height as f64;

            if width_ratio > height_ratio {
                // we are cropping the height
                local_y *= width_ratio / height_ratio;
            } else {
                // we are cropping the width
                local_x *= height_ratio / width_ratio;
            }

            local_x *= resolution[0] as f64;
            local_y *= resolution[1] as f64;

            let camera_transform = get_transform(camera_entity, &world.entities_and_components);
            local_x += camera_transform.x;
            local_y += camera_transform.y;

            let input = world
                .entities_and_components
                .get_resource_mut::<Input>()
                .expect("failed to get input system probably a version mismatch");

            input.set_mouse_position(local_x as f32, local_y as f32);
        }
        Mouse::Error => {}
    }
}

impl LumenpyxProgram {
    /// create a new lumenpyx program
    pub(crate) fn new(resolution: [u32; 2], name: &str) -> (Self, EventLoop<()>) {
        let (program, event_loop) = lumenpyx::LumenpyxProgram::new(resolution, name);
        let keys_down = HashSet::new();

        (
            Self {
                internal_program: program,
                keys_down,
            },
            event_loop,
        )
    }
}

impl Deref for LumenpyxProgram {
    type Target = lumenpyx::LumenpyxProgram;

    fn deref(&self) -> &Self::Target {
        &self.internal_program
    }
}

impl DerefMut for LumenpyxProgram {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.internal_program
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

/// A component that prevents an entity and it's children from being rendered
#[derive(Clone, Copy)]
pub struct NotActive;

pub fn get_camera(scene: &EntitiesAndComponents) -> Option<Entity> {
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

                return Some(camera_entity);
            }
        }
    }

    None
}

///  Renders the scene
pub fn render(scene: &mut EntitiesAndComponents) {
    let camera_entity = get_camera(scene).expect("failed to get camera");

    let camera_component = scene
        .try_get_component::<Camera>(camera_entity)
        .expect("failed to get camera")
        .clone();

    render_objects(scene, &camera_component.lumen_camera);
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

enum OwnedOrMutableDrawable<'a> {
    Owned(Box<dyn Drawable + 'a>),
    Mutable(&'a mut dyn Drawable),
}

impl<'a> Deref for OwnedOrMutableDrawable<'a> {
    type Target = dyn Drawable + 'a;

    fn deref(&self) -> &Self::Target {
        match self {
            OwnedOrMutableDrawable::Mutable(mutable) => {
                let dyn_obj: &dyn Drawable = *mutable;
                dyn_obj
            }
            OwnedOrMutableDrawable::Owned(owned) => &(**owned),
        }
    }
}

impl DerefMut for OwnedOrMutableDrawable<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            OwnedOrMutableDrawable::Mutable(mutable) => *mutable,
            OwnedOrMutableDrawable::Owned(owned) => &mut (**owned),
        }
    }
}

fn get_blend_object(
    entities_and_components: &mut EntitiesAndComponents,
    entity: Entity,
    total_time: f64,
) -> Option<Box<LumenBlendObject<'_>>> {
    let children = entities_and_components.get_children(entity);

    let entities_and_components_ptr = entities_and_components as *mut EntitiesAndComponents;

    // SAFETY: the transform and blend mode are dropped at the end of this block before they are mutably borrowed again
    let (blend_mode, transform) = unsafe { &mut *entities_and_components_ptr }
        .try_get_components::<(BlendComponent, ABC_Game_Engine::Transform)>(entity);

    let transform_clone = if let Some(ref transform) = transform {
        Some(transform)
    } else {
        None
    };

    let mut final_drawable = None;

    match (blend_mode, transform_clone) {
        (Some(blend_mode), Some(transform)) => {
            let mut drawables_in_children = vec![];

            // collect the drawables in the children
            for child in children {
                // SAFETY: This doesn't intersect with our only other borrow which is of transform and blend mode
                let (drawables, transform) = get_all_drawables_on_object_mut(
                    unsafe { &mut *entities_and_components_ptr },
                    child,
                    total_time,
                    true,
                );

                // only the amount of drawables is checked
                if drawables.len() != 0 {
                    drawables_in_children.push(EntityDepthItem {
                        entity: child,
                        transform: transform.cloned().unwrap_or_default(),
                    });
                    if drawables_in_children.len() >= 2 {
                        // we only need 2 drawables
                        break;
                    }
                }
            }
            // by now all the drawables in the above loop are dropped

            // sort the drawables so we can blend the first two
            drawables_in_children.sort();

            if drawables_in_children.len() < 2 {
                // TODO: when we have a logger, log this as a warning
                // we need at least 2 drawables to blend
            } else {
                // this would be a bug in the ecs if this happened but, it would be a huge safety issue so we check
                assert!(drawables_in_children[0] != drawables_in_children[1]);

                let drawable_entity_1 = drawables_in_children.remove(0).entity;
                let drawable_entity_2 = drawables_in_children.remove(0).entity;

                // SAFETY: we checked that the entities are different
                // SAFETY: we also know this is not intersecting with the transform and blend mode borrow
                let (mut drawables_on_1, transform_1) = get_all_drawables_on_object_mut(
                    unsafe { &mut *entities_and_components_ptr },
                    drawable_entity_1,
                    total_time,
                    true,
                );

                // SAFETY: we checked that the entities are different
                // SAFETY: we also know this is not intersecting with the transform and blend mode borrow
                let (mut drawables_on_2, transform_2) = get_all_drawables_on_object_mut(
                    unsafe { &mut *entities_and_components_ptr },
                    drawable_entity_2,
                    total_time,
                    true,
                );

                let mut drawable_1 = drawables_on_1.remove(0);
                let mut drawable_2 = drawables_on_2.remove(0);

                // we set the position of the children rather than the parent
                // it's a bit weird, but i tried the other way and it didn't work so this is fine
                if let Some(transform_1) = transform_1 {
                    drawable_1.set_transform(abc_transform_to_lumen_transform(*transform_1));
                }

                if let Some(transform_2) = transform_2 {
                    drawable_2.set_transform(abc_transform_to_lumen_transform(*transform_2));
                }

                let mut new_blend_obj =
                    LumenBlendObject::new(drawable_1, drawable_2, blend_mode.lumen_blend_mode);

                new_blend_obj.set_transform(abc_transform_to_lumen_transform(**transform));

                final_drawable = Some(Box::new(new_blend_obj));
            }
        }
        _ => (), // no blend mode or no children
    }

    final_drawable
}

fn get_all_drawables_on_object_mut<'a>(
    entities_and_components: &'a mut EntitiesAndComponents,
    entity: Entity,
    total_time: f64,
    ignore_precheck: bool,
) -> (
    Vec<OwnedOrMutableDrawable<'a>>,
    Option<&mut ABC_Game_Engine::Transform>,
) {
    // needs to be benchmarked, but i can't think of a better way to do this...
    if !ignore_precheck {
        let mut current_entity = entity;
        while let Some(parent) = entities_and_components.get_parent(current_entity) {
            current_entity = parent;

            let not_active = entities_and_components.try_get_component::<NotActive>(current_entity);
            if not_active.is_some() {
                return (vec![], None);
            }
            let blend_object =
                get_blend_object(entities_and_components, current_entity, total_time);

            // remember that we are looking at the parent so we don't draw that right now, when the blend object is the original entity it will be drawn
            if blend_object.is_some() {
                return (vec![], None);
            }
        }
    }

    let mut final_drawables = vec![];

    let entities_and_components_ptr = entities_and_components as *mut EntitiesAndComponents;

    // SAFETY: this only borrows children (at the end) and the transform and blend mode are dropped at the end of the function
    // SAFETY: So as long as we don't try to get the children of the entity this is safe
    let blend_object = get_blend_object(
        unsafe { &mut *entities_and_components_ptr },
        entity,
        total_time,
    );

    if let Some(blend_object) = blend_object {
        final_drawables.push(OwnedOrMutableDrawable::Owned(blend_object));
    }

    let (
        circle,
        rectangle,
        sprite,
        sphere,
        animation,
        cylinder,
        animation_state_machine,
        text_box,
        transform,
        not_active,
    ) = entities_and_components.try_get_components_mut::<(
        Circle,
        Rectangle,
        Sprite,
        Sphere,
        Animation,
        Cylinder,
        AnimationStateMachine,
        TextBox,
        ABC_Game_Engine::Transform,
        NotActive,
    )>(entity);

    if not_active.is_some() {
        return (vec![], None);
    }

    // this is abhorrent, but I can't think of any better way to do this
    let mut mut_drawables = vec![];
    match circle {
        Some(circle) => mut_drawables.push(circle as &mut dyn Drawable),
        None => (),
    }
    match rectangle {
        Some(rectangle) => mut_drawables.push(rectangle as &mut dyn Drawable),
        None => (),
    }
    match sprite {
        Some(sprite) => mut_drawables.push(sprite as &mut dyn Drawable),
        None => (),
    }
    match sphere {
        Some(sphere) => mut_drawables.push(sphere as &mut dyn Drawable),
        None => (),
    }
    match animation {
        Some(animation) => mut_drawables.push(animation as &mut dyn Drawable),
        None => (),
    }
    match cylinder {
        Some(cylinder) => mut_drawables.push(cylinder as &mut dyn Drawable),
        None => (),
    }
    match animation_state_machine {
        Some(animation_state_machine) => {
            mut_drawables.push(animation_state_machine as &mut dyn Drawable)
        }
        None => (),
    }
    match text_box {
        Some(text_box) => mut_drawables.push(text_box as &mut dyn Drawable),
        None => (),
    }

    for drawable in mut_drawables {
        final_drawables.push(OwnedOrMutableDrawable::Mutable(drawable));
    }

    (final_drawables, transform)
}

// gets all the entities that are drawable
// needs to be mutable to update the animation time
fn get_all_entities_with_drawables(
    entities_and_components: &mut EntitiesAndComponents,
    total_time: f64,
) -> Vec<Entity> {
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

    for entity in entities_with_animation.iter() {
        let (animation,) = entities_and_components.get_components_mut::<(Animation,)>(*entity);

        animation.set_total_time_f64(total_time);
    }

    let entities_with_animation_state_machine = entities_and_components
        .get_entities_with_component::<AnimationStateMachine>()
        .cloned()
        .collect::<Vec<Entity>>();

    for entity in entities_with_animation_state_machine.iter() {
        let (animation_state_machine,) =
            entities_and_components.get_components_mut::<(AnimationStateMachine,)>(*entity);

        animation_state_machine.set_total_time_f64(total_time);
    }

    let entities_with_cylinder = entities_and_components
        .get_entities_with_component::<Cylinder>()
        .cloned()
        .collect::<Vec<Entity>>();

    let entities_with_blend_component = entities_and_components
        .get_entities_with_component::<BlendComponent>()
        .cloned()
        .collect::<Vec<Entity>>();

    let entities_with_text_box = entities_and_components
        .get_entities_with_component::<TextBox>()
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
    entities.extend(entities_with_animation_state_machine);
    entities.extend(entities_with_blend_component);
    entities.extend(entities_with_text_box);
    entities.extend(entities_with_point_light);
    entities.extend(entities_with_area_light);
    entities.extend(entities_with_directional_light);

    // remove duplicates
    entities.sort();
    entities.dedup();

    entities
}

fn render_objects(entities_and_components: &mut EntitiesAndComponents, camera: &lumenpyx::Camera) {
    let mut entity_depth_array = vec![];

    let total_time = entities_and_components
        .get_resource::<DeltaTime>()
        .expect("failed to get delta time")
        .get_total_time();

    collect_renderable_entities(entities_and_components, &mut entity_depth_array, total_time);

    entity_depth_array.sort();
    let mut sprites = vec![];
    let mut lights_in_scene = vec![];

    let entities_and_components_ptr = entities_and_components as *mut EntitiesAndComponents;
    // could possibly be done multithreaded and combine layers afterward
    for entity_depth_item in entity_depth_array {
        let entity = entity_depth_item.entity;

        // SAFETY: the only reason this is a problem is because of the sprites vector,
        // which is only used after the loop so there is no double mutable borrow
        let entities_and_components = unsafe { &mut *entities_and_components_ptr };

        let entities_and_components_ptr = entities_and_components as *mut EntitiesAndComponents;

        // SAFETY: This is safe as long as we don't use sprites or lights_in scene before the loop ends
        // AND the types of get_all_lights_on_object_mut and get_all_drawables_on_object_mut are non overlapping
        let (lights, _) =
            get_all_lights_on_object_mut(unsafe { &mut *entities_and_components_ptr }, entity);

        let (drawables, transform) =
            get_all_drawables_on_object_mut(entities_and_components, entity, total_time, false);
        {
            if let Some(_) = transform {
                let transform = &(entity_depth_item.transform);

                for mut drawable in drawables {
                    drawable.set_transform(abc_transform_to_lumen_transform(transform.clone()));
                    sprites.push(drawable);
                }

                for light in lights {
                    light.set_transform(abc_transform_to_lumen_transform(transform.clone()));
                    lights_in_scene.push(&*light);
                }
            }
        }
    }

    // re-borrow everything while retaining the owned objects
    let mut sprite_borrows = vec![];
    for sprite in &sprites {
        sprite_borrows.push(sprite.deref());
    }

    let lumen_program = entities_and_components
        .get_resource_mut::<LumenpyxProgram>()
        .expect("failed to get lumen program");

    draw_all(lights_in_scene, sprite_borrows, lumen_program, camera)
}

/// A recursive function that collects all renderable entities in the scene
/// mutable to update the animation time
fn collect_renderable_entities(
    entities_and_components: &mut EntitiesAndComponents,
    out_list: &mut Vec<EntityDepthItem>,
    total_time: f64,
) {
    let entities_with_drawables =
        get_all_entities_with_drawables(entities_and_components, total_time);

    for entity in entities_with_drawables {
        let transform = ABC_Game_Engine::get_transform(entity, entities_and_components);

        out_list.push(EntityDepthItem { entity, transform });
    }
}

struct EntityDepthItem {
    /// ordered by child depth, so entity1 has entity2 as a child which has entity3 as a child
    /// entity1 will not be rendered as part of the pass for this object just entity3.
    /// entity1 and entity 2 will have its own pass
    entity: Entity,
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
    let mut lumen_transform =
        Transform::new([transform.x as f32, transform.y as f32, transform.z as f32]);
    lumen_transform.set_scale(transform.scale as f32, transform.scale as f32, 1.0);
    lumen_transform.set_rotation(transform.rotation as f32);
    lumen_transform
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abc_transform_to_lumen_transform() {
        let abc_transform = ABC_Game_Engine::Transform {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            scale: 4.0,
            rotation: 5.0,
            origin_x: 0.0,
            origin_y: 0.0,
        };

        let lumen_transform = abc_transform_to_lumen_transform(abc_transform);

        assert_eq!(lumen_transform.get_x(), 1.0);
        assert_eq!(lumen_transform.get_y(), 2.0);
        assert_eq!(lumen_transform.get_z(), 3.0);
        assert_eq!(lumen_transform.get_scale(), [4.0, 4.0, 1.0]);
        assert_eq!(lumen_transform.get_rotation(), 5.0);
    }
}
