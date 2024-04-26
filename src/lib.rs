//! The official ABC Game Engine implementation of lumenpyx

use lumenpyx::animation::Animation;
use lumenpyx::drawable_object::Drawable;
use lumenpyx::lights::{AreaLight, PointLight};
use lumenpyx::primitives::{Circle, Rectangle, Sprite};
use lumenpyx::primitives::{Cylinder, Sphere};
//pub use lumenpyx::*;
use lumenpyx::lights::DirectionalLight;
use lumenpyx::lights::LightDrawable;
pub use lumenpyx::*;
use ABC_Game_Engine::EntitiesAndComponents;
use ABC_Game_Engine::Entity;
use ABC_Game_Engine::{self};

#[derive(Clone, Copy)]
pub struct Camera {
    lumen_camera: lumenpyx::Camera,
    is_active: bool,
}

impl Camera {
    pub fn new(position: [f32; 3]) -> Self {
        Self {
            lumen_camera: lumenpyx::Camera::new(position),
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
    let (circle, rectangle, sprite, sphere, animation, cylinder, transform) =
        entities_and_components.try_get_components_mut::<(
            Circle,
            Rectangle,
            Sprite,
            Sphere,
            Animation,
            Cylinder,
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
