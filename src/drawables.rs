// this is all pretty terrible, but it's the best I can do for now
// All this only does one thing: exclude the transform from the drawable objects so that they can be used in the ECS without confusion

pub mod primitives {
    use lumenpyx::drawable_object::Drawable;
    use lumenpyx::lights::LightDrawable;
    use lumenpyx::primitives::Normal;
    use lumenpyx::primitives::Texture;
    use lumenpyx::LumenpyxProgram;
    use lumenpyx::Transform;
    use std::ops::Deref;
    use std::ops::DerefMut;

    #[derive(Clone, Copy, Debug)]
    pub struct BlendComponent {
        pub(crate) lumen_blend_mode: lumenpyx::blending::BlendMode,
        pub(crate) reverse: bool,
    }

    impl BlendComponent {
        pub fn new(lumen_blend_mode: lumenpyx::blending::BlendMode, reverse: bool) -> Self {
            Self {
                lumen_blend_mode,
                reverse,
            }
        }
    }

    /// An animation drawable object
    pub struct Animation {
        lumen_animation: lumenpyx::animation::Animation,
    }

    impl Animation {
        pub fn new_from_images(
            albedo: Texture,
            height: Texture,
            roughness: Texture,
            normal: Normal,
            num_frames: usize,
            time_between_frames: std::time::Duration,
            program: &LumenpyxProgram,
        ) -> Self {
            Self {
                lumen_animation: lumenpyx::animation::Animation::new_from_images(
                    albedo,
                    height,
                    roughness,
                    normal,
                    num_frames,
                    time_between_frames,
                    Transform::default(),
                    program,
                ),
            }
        }

        pub fn new_from_spritesheet(
            albedo: Texture,
            height: Texture,
            roughness: Texture,
            normal: Normal,
            num_frames: usize,
            time_between_frames: std::time::Duration,
            program: &LumenpyxProgram,
        ) -> Self {
            Self {
                lumen_animation: lumenpyx::animation::Animation::new_from_spritesheet(
                    albedo,
                    height,
                    roughness,
                    normal,
                    num_frames,
                    time_between_frames,
                    Transform::default(),
                    program,
                ),
            }
        }
    }

    impl Drawable for Animation {
        fn draw(
            &self,
            program: &LumenpyxProgram,
            transform_matrix: [[f32; 4]; 4],
            albedo_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
            height_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
            roughness_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
            normal_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        ) {
            self.lumen_animation.draw(
                program,
                transform_matrix,
                albedo_framebuffer,
                height_framebuffer,
                roughness_framebuffer,
                normal_framebuffer,
            );
        }

        fn set_transform(&mut self, transform: Transform) {
            self.lumen_animation.set_transform(transform);
        }

        fn try_load_shaders(&self, program: &mut LumenpyxProgram) {
            self.lumen_animation.try_load_shaders(program);
        }

        fn get_position(&self) -> [[f32; 4]; 4] {
            self.lumen_animation.get_position()
        }
    }

    impl Deref for Animation {
        type Target = lumenpyx::animation::Animation;

        fn deref(&self) -> &Self::Target {
            &self.lumen_animation
        }
    }

    impl DerefMut for Animation {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.lumen_animation
        }
    }

    pub struct AnimationStateMachine {
        lumen_animation_state_machine: lumenpyx::animation::AnimationStateMachine,
    }

    impl AnimationStateMachine {
        pub fn new(animations: Vec<Animation>) -> Self {
            let mut animations_to_use = Vec::new();
            for animation in animations {
                animations_to_use.push(animation.lumen_animation);
            }

            Self {
                lumen_animation_state_machine: lumenpyx::animation::AnimationStateMachine::new(
                    animations_to_use,
                ),
            }
        }

        pub fn set_current_animation(&mut self, current_animation: usize) {
            self.lumen_animation_state_machine
                .set_current_animation(current_animation);
        }
    }

    impl Drawable for AnimationStateMachine {
        fn draw(
            &self,
            program: &LumenpyxProgram,
            transform_matrix: [[f32; 4]; 4],
            albedo_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
            height_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
            roughness_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
            normal_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        ) {
            self.lumen_animation_state_machine.draw(
                program,
                transform_matrix,
                albedo_framebuffer,
                height_framebuffer,
                roughness_framebuffer,
                normal_framebuffer,
            );
        }

        fn set_transform(&mut self, transform: Transform) {
            self.lumen_animation_state_machine.set_transform(transform);
        }

        fn try_load_shaders(&self, program: &mut LumenpyxProgram) {
            self.lumen_animation_state_machine.try_load_shaders(program);
        }

        fn get_position(&self) -> [[f32; 4]; 4] {
            self.lumen_animation_state_machine.get_position()
        }
    }

    impl Deref for AnimationStateMachine {
        type Target = lumenpyx::animation::AnimationStateMachine;

        fn deref(&self) -> &Self::Target {
            &self.lumen_animation_state_machine
        }
    }

    impl DerefMut for AnimationStateMachine {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.lumen_animation_state_machine
        }
    }

    pub struct Circle {
        lumen_circle: lumenpyx::primitives::Circle,
    }

    impl Circle {
        pub fn new(color: [f32; 4], radius: f32) -> Self {
            Self {
                lumen_circle: lumenpyx::primitives::Circle::new(
                    color,
                    radius,
                    Transform::default(),
                ),
            }
        }
    }

    impl Drawable for Circle {
        fn draw(
            &self,
            program: &LumenpyxProgram,
            transform_matrix: [[f32; 4]; 4],
            albedo_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
            height_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
            roughness_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
            normal_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        ) {
            self.lumen_circle.draw(
                program,
                transform_matrix,
                albedo_framebuffer,
                height_framebuffer,
                roughness_framebuffer,
                normal_framebuffer,
            );
        }

        fn set_transform(&mut self, transform: Transform) {
            self.lumen_circle.set_transform(transform);
        }

        fn try_load_shaders(&self, program: &mut LumenpyxProgram) {
            self.lumen_circle.try_load_shaders(program);
        }

        fn get_position(&self) -> [[f32; 4]; 4] {
            self.lumen_circle.get_position()
        }
    }

    impl Deref for Circle {
        type Target = lumenpyx::primitives::Circle;

        fn deref(&self) -> &Self::Target {
            &self.lumen_circle
        }
    }

    impl DerefMut for Circle {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.lumen_circle
        }
    }

    pub struct Rectangle {
        lumen_rectangle: lumenpyx::primitives::Rectangle,
    }

    impl Rectangle {
        pub fn new(color: [f32; 4], width: f32, height: f32) -> Self {
            Self {
                lumen_rectangle: lumenpyx::primitives::Rectangle::new(
                    color,
                    width,
                    height,
                    Transform::default(),
                ),
            }
        }
    }

    impl Drawable for Rectangle {
        fn draw(
            &self,
            program: &LumenpyxProgram,
            transform_matrix: [[f32; 4]; 4],
            albedo_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
            height_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
            roughness_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
            normal_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        ) {
            self.lumen_rectangle.draw(
                program,
                transform_matrix,
                albedo_framebuffer,
                height_framebuffer,
                roughness_framebuffer,
                normal_framebuffer,
            );
        }

        fn set_transform(&mut self, transform: Transform) {
            self.lumen_rectangle.set_transform(transform);
        }

        fn try_load_shaders(&self, program: &mut LumenpyxProgram) {
            self.lumen_rectangle.try_load_shaders(program);
        }

        fn get_position(&self) -> [[f32; 4]; 4] {
            self.lumen_rectangle.get_position()
        }
    }

    impl Deref for Rectangle {
        type Target = lumenpyx::primitives::Rectangle;

        fn deref(&self) -> &Self::Target {
            &self.lumen_rectangle
        }
    }

    impl DerefMut for Rectangle {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.lumen_rectangle
        }
    }

    pub struct Sprite {
        lumen_sprite: lumenpyx::primitives::Sprite,
    }

    impl Sprite {
        pub fn new(
            albedo: Texture,
            height: Texture,
            roughness: Texture,
            normal: Normal,
            program: &LumenpyxProgram,
        ) -> Self {
            Self {
                lumen_sprite: lumenpyx::primitives::Sprite::new(
                    albedo,
                    height,
                    roughness,
                    normal,
                    program,
                    Transform::default(),
                ),
            }
        }
    }

    impl Drawable for Sprite {
        fn draw(
            &self,
            program: &LumenpyxProgram,
            transform_matrix: [[f32; 4]; 4],
            albedo_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
            height_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
            roughness_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
            normal_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        ) {
            self.lumen_sprite.draw(
                program,
                transform_matrix,
                albedo_framebuffer,
                height_framebuffer,
                roughness_framebuffer,
                normal_framebuffer,
            );
        }

        fn set_transform(&mut self, transform: Transform) {
            self.lumen_sprite.set_transform(transform);
        }

        fn try_load_shaders(&self, program: &mut LumenpyxProgram) {
            self.lumen_sprite.try_load_shaders(program);
        }

        fn get_position(&self) -> [[f32; 4]; 4] {
            self.lumen_sprite.get_position()
        }
    }

    impl Deref for Sprite {
        type Target = lumenpyx::primitives::Sprite;

        fn deref(&self) -> &Self::Target {
            &self.lumen_sprite
        }
    }

    impl DerefMut for Sprite {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.lumen_sprite
        }
    }

    pub struct Cylinder {
        lumen_cylinder: lumenpyx::primitives::Cylinder,
    }

    impl Cylinder {
        pub fn new(color: [f32; 4], radius: f32, height: f32) -> Self {
            Self {
                lumen_cylinder: lumenpyx::primitives::Cylinder::new(
                    color,
                    radius,
                    height,
                    Transform::default(),
                ),
            }
        }
    }

    impl Drawable for Cylinder {
        fn draw(
            &self,
            program: &LumenpyxProgram,
            transform_matrix: [[f32; 4]; 4],
            albedo_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
            height_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
            roughness_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
            normal_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        ) {
            self.lumen_cylinder.draw(
                program,
                transform_matrix,
                albedo_framebuffer,
                height_framebuffer,
                roughness_framebuffer,
                normal_framebuffer,
            );
        }

        fn set_transform(&mut self, transform: Transform) {
            self.lumen_cylinder.set_transform(transform);
        }

        fn try_load_shaders(&self, program: &mut LumenpyxProgram) {
            self.lumen_cylinder.try_load_shaders(program);
        }

        fn get_position(&self) -> [[f32; 4]; 4] {
            self.lumen_cylinder.get_position()
        }
    }

    impl Deref for Cylinder {
        type Target = lumenpyx::primitives::Cylinder;

        fn deref(&self) -> &Self::Target {
            &self.lumen_cylinder
        }
    }

    impl DerefMut for Cylinder {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.lumen_cylinder
        }
    }

    pub struct Sphere {
        lumen_sphere: lumenpyx::primitives::Sphere,
    }

    impl Sphere {
        pub fn new(color: [f32; 4], radius: f32) -> Self {
            Self {
                lumen_sphere: lumenpyx::primitives::Sphere::new(
                    color,
                    radius,
                    Transform::default(),
                ),
            }
        }
    }

    impl Drawable for Sphere {
        fn draw(
            &self,
            program: &LumenpyxProgram,
            transform_matrix: [[f32; 4]; 4],
            albedo_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
            height_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
            roughness_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
            normal_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        ) {
            self.lumen_sphere.draw(
                program,
                transform_matrix,
                albedo_framebuffer,
                height_framebuffer,
                roughness_framebuffer,
                normal_framebuffer,
            );
        }

        fn set_transform(&mut self, transform: Transform) {
            self.lumen_sphere.set_transform(transform);
        }

        fn try_load_shaders(&self, program: &mut LumenpyxProgram) {
            self.lumen_sphere.try_load_shaders(program);
        }

        fn get_position(&self) -> [[f32; 4]; 4] {
            self.lumen_sphere.get_position()
        }
    }

    impl Deref for Sphere {
        type Target = lumenpyx::primitives::Sphere;

        fn deref(&self) -> &Self::Target {
            &self.lumen_sphere
        }
    }

    impl DerefMut for Sphere {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.lumen_sphere
        }
    }
}

// now for the lights

pub mod lights {
    use lumenpyx::drawable_object::Drawable;
    use lumenpyx::lights::LightDrawable;
    use lumenpyx::primitives::Normal;
    use lumenpyx::primitives::Texture;
    use lumenpyx::LumenpyxProgram;
    use lumenpyx::Transform;
    use std::ops::Deref;
    use std::ops::DerefMut;

    pub struct DirectionalLight {
        lumen_directional_light: lumenpyx::lights::DirectionalLight,
    }

    impl DirectionalLight {
        pub fn new(
            direction: [f32; 3],
            color: [f32; 3],
            intensity: f32,
            angular_falloff: f32,
            distance_falloff: f32,
        ) -> Self {
            Self {
                lumen_directional_light: lumenpyx::lights::DirectionalLight::new(
                    [0.0, 0.0, 0.0],
                    direction,
                    color,
                    intensity,
                    angular_falloff,
                    distance_falloff,
                ),
            }
        }
    }

    impl LightDrawable for DirectionalLight {
        fn draw(
            &self,
            program: &LumenpyxProgram,
            matrix_transform: [[f32; 4]; 4],
            albedo_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
            height_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
            albedo_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
            roughness_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
            shadow_strength_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
        ) {
            self.lumen_directional_light.draw(
                program,
                matrix_transform,
                albedo_framebuffer,
                height_uniform,
                albedo_uniform,
                roughness_uniform,
                shadow_strength_uniform,
            );
        }

        fn set_transform(&mut self, transform: Transform) {
            self.lumen_directional_light.set_transform(transform);
        }

        fn try_load_shaders(&self, program: &mut LumenpyxProgram) {
            self.lumen_directional_light.try_load_shaders(program);
        }

        fn get_transform(&self) -> [[f32; 4]; 4] {
            self.lumen_directional_light.get_transform()
        }
    }

    impl Deref for DirectionalLight {
        type Target = lumenpyx::lights::DirectionalLight;

        fn deref(&self) -> &Self::Target {
            &self.lumen_directional_light
        }
    }

    impl DerefMut for DirectionalLight {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.lumen_directional_light
        }
    }

    pub struct PointLight {
        lumen_point_light: lumenpyx::lights::PointLight,
    }

    impl PointLight {
        pub fn new(color: [f32; 3], intensity: f32, falloff: f32) -> Self {
            Self {
                lumen_point_light: lumenpyx::lights::PointLight::new(
                    [0.0, 0.0, 0.0],
                    color,
                    intensity,
                    falloff,
                ),
            }
        }
    }

    impl LightDrawable for PointLight {
        fn draw(
            &self,
            program: &LumenpyxProgram,
            matrix_transform: [[f32; 4]; 4],
            albedo_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
            height_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
            albedo_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
            roughness_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
            shadow_strength_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
        ) {
            self.lumen_point_light.draw(
                program,
                matrix_transform,
                albedo_framebuffer,
                height_uniform,
                albedo_uniform,
                roughness_uniform,
                shadow_strength_uniform,
            );
        }

        fn get_transform(&self) -> [[f32; 4]; 4] {
            self.lumen_point_light.get_transform()
        }

        fn set_transform(&mut self, transform: Transform) {
            self.lumen_point_light.set_transform(transform);
        }

        fn try_load_shaders(&self, program: &mut LumenpyxProgram) {
            self.lumen_point_light.try_load_shaders(program);
        }
    }

    impl Deref for PointLight {
        type Target = lumenpyx::lights::PointLight;

        fn deref(&self) -> &Self::Target {
            &self.lumen_point_light
        }
    }

    impl DerefMut for PointLight {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.lumen_point_light
        }
    }

    pub struct AreaLight {
        lumen_area_light: lumenpyx::lights::AreaLight,
    }

    impl AreaLight {
        pub fn new(color: [f32; 3], intensity: f32, falloff: f32, width: f32, height: f32) -> Self {
            Self {
                lumen_area_light: lumenpyx::lights::AreaLight::new(
                    [0.0, 0.0, 0.0],
                    color,
                    intensity,
                    falloff,
                    width,
                    height,
                ),
            }
        }
    }

    impl LightDrawable for AreaLight {
        fn draw(
            &self,
            program: &LumenpyxProgram,
            matrix_transform: [[f32; 4]; 4],
            albedo_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
            height_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
            albedo_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
            roughness_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
            shadow_strength_uniform: glium::uniforms::Sampler<glium::texture::Texture2d>,
        ) {
            self.lumen_area_light.draw(
                program,
                matrix_transform,
                albedo_framebuffer,
                height_uniform,
                albedo_uniform,
                roughness_uniform,
                shadow_strength_uniform,
            );
        }

        fn get_transform(&self) -> [[f32; 4]; 4] {
            self.lumen_area_light.get_transform()
        }

        fn set_transform(&mut self, transform: Transform) {
            self.lumen_area_light.set_transform(transform);
        }

        fn try_load_shaders(&self, program: &mut LumenpyxProgram) {
            self.lumen_area_light.try_load_shaders(program);
        }
    }

    impl Deref for AreaLight {
        type Target = lumenpyx::lights::AreaLight;

        fn deref(&self) -> &Self::Target {
            &self.lumen_area_light
        }
    }

    impl DerefMut for AreaLight {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.lumen_area_light
        }
    }
}
