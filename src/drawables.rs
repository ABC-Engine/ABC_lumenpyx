// this is all pretty terrible, but it's the best I can do for now
// All this only does one thing: exclude the transform from the drawable objects so that they can be used in the ECS without confusion

pub mod primitives {
    use lumenpyx::drawable_object::Drawable;
    use lumenpyx::lights::LightDrawable;
    use lumenpyx::primitives::Normal;
    use lumenpyx::primitives::NormalInput;
    use lumenpyx::primitives::Texture;
    use lumenpyx::primitives::TextureInput;
    use lumenpyx::LumenpyxProgram;
    use lumenpyx::TextureHandle;
    use lumenpyx::Transform;
    use std::ops::Deref;
    use std::ops::DerefMut;

    use crate::OwnedOrMutableDrawable;

    pub(crate) struct LumenBlendObject<'a> {
        blend_1: OwnedOrMutableDrawable<'a>,
        blend_2: OwnedOrMutableDrawable<'a>,
        blend_mode: lumenpyx::blending::BlendMode,
        transform: Transform,
    }

    impl<'a> LumenBlendObject<'a> {
        pub(crate) fn new(
            blend_1: OwnedOrMutableDrawable<'a>,
            blend_2: OwnedOrMutableDrawable<'a>,
            blend_mode: lumenpyx::blending::BlendMode,
        ) -> Self {
            Self {
                blend_1,
                blend_2,
                blend_mode,
                transform: Transform::default(),
            }
        }
    }

    impl<'a> Drawable for LumenBlendObject<'a> {
        fn draw_albedo(
            &self,
            program: &LumenpyxProgram,
            transform: &Transform,
            albedo_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        ) {
            // construct the blend object and draw it
            let mut blend_object = lumenpyx::blending::BlendObject::new(
                &*self.blend_1,
                &*self.blend_2,
                self.blend_mode,
            );

            // this broke it, so as long as no one tries to use the transform directly, which the user can't because it's private, this should be fine
            //blend_object.set_transform(self.transform);

            blend_object.draw_albedo(program, transform, albedo_framebuffer);
        }

        fn draw_height(
            &self,
            program: &LumenpyxProgram,
            transform: &Transform,
            height_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        ) {
            // construct the blend object and draw it
            let mut blend_object = lumenpyx::blending::BlendObject::new(
                &*self.blend_1,
                &*self.blend_2,
                self.blend_mode,
            );

            //blend_object.set_transform(self.transform);

            blend_object.draw_height(program, transform, height_framebuffer);
        }

        fn draw_normal(
            &self,
            program: &LumenpyxProgram,
            transform: &Transform,
            normal_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        ) {
            // construct the blend object and draw it
            let mut blend_object = lumenpyx::blending::BlendObject::new(
                &*self.blend_1,
                &*self.blend_2,
                self.blend_mode,
            );

            //blend_object.set_transform(self.transform);

            blend_object.draw_normal(program, transform, normal_framebuffer);
        }

        fn draw_roughness(
            &self,
            program: &LumenpyxProgram,
            transform: &Transform,
            roughness_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        ) {
            // construct the blend object and draw it
            let mut blend_object = lumenpyx::blending::BlendObject::new(
                &*self.blend_1,
                &*self.blend_2,
                self.blend_mode,
            );

            //blend_object.set_transform(self.transform);

            blend_object.draw_roughness(program, transform, roughness_framebuffer);
        }

        fn set_transform(&mut self, transform: Transform) {
            self.transform = transform;
        }

        fn try_load_shaders(&self, program: &mut LumenpyxProgram) {
            lumenpyx::blending::BlendObject::new(&*self.blend_1, &*self.blend_2, self.blend_mode)
                .try_load_shaders(program);
        }

        fn get_transform(&self) -> Transform {
            self.transform
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub struct BlendComponent {
        pub(crate) lumen_blend_mode: lumenpyx::blending::BlendMode,
    }

    impl BlendComponent {
        pub fn new(lumen_blend_mode: lumenpyx::blending::BlendMode) -> Self {
            Self { lumen_blend_mode }
        }
    }

    /// An animation drawable object
    /// make sure to restart the animation when adding it to the ECS if you don't have loop animation enabled
    #[derive(Clone)]
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
            program: &mut LumenpyxProgram,
            loop_animation: bool,
        ) -> (
            Self,
            Vec<TextureHandle>,
            Vec<TextureHandle>,
            Vec<TextureHandle>,
            Vec<TextureHandle>,
        ) {
            let (anim, albedo, height, roughness, normal) =
                lumenpyx::animation::Animation::new_from_images(
                    albedo,
                    height,
                    roughness,
                    normal,
                    num_frames,
                    time_between_frames,
                    Transform::default(),
                    program,
                    loop_animation,
                );

            (
                Self {
                    lumen_animation: anim,
                },
                albedo,
                height,
                roughness,
                normal,
            )
        }

        pub fn new_from_spritesheet(
            albedo: Texture,
            height: Texture,
            roughness: Texture,
            normal: Normal,
            num_frames: usize,
            time_between_frames: std::time::Duration,
            program: &mut LumenpyxProgram,
            loop_animation: bool,
        ) -> (
            Self,
            Vec<TextureHandle>,
            Vec<TextureHandle>,
            Vec<TextureHandle>,
            Vec<TextureHandle>,
        ) {
            let (anim, albedo, height, roughness, normal) =
                lumenpyx::animation::Animation::new_from_spritesheet(
                    albedo,
                    height,
                    roughness,
                    normal,
                    num_frames,
                    time_between_frames,
                    Transform::default(),
                    program,
                    loop_animation,
                );

            (
                Self {
                    lumen_animation: anim,
                },
                albedo,
                height,
                roughness,
                normal,
            )
        }

        pub fn new_from_handles(
            albedo: Vec<TextureHandle>,
            height: Vec<TextureHandle>,
            roughness: Vec<TextureHandle>,
            normal: Vec<TextureHandle>,
            time_between_frames: std::time::Duration,
            program: &mut LumenpyxProgram,
            loop_animation: bool,
        ) -> Self {
            Self {
                lumen_animation: lumenpyx::animation::Animation::new_from_handles(
                    albedo,
                    height,
                    roughness,
                    normal,
                    program,
                    time_between_frames,
                    Transform::default(),
                    loop_animation,
                ),
            }
        }

        pub fn restart_animation(&mut self) {
            self.lumen_animation.restart_animation();
        }
    }

    impl Drawable for Animation {
        fn draw_albedo(
            &self,
            program: &LumenpyxProgram,
            transform: &Transform,
            albedo_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        ) {
            self.lumen_animation
                .draw_albedo(program, transform, albedo_framebuffer);
        }

        fn draw_height(
            &self,
            program: &LumenpyxProgram,
            transform: &Transform,
            height_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        ) {
            self.lumen_animation
                .draw_height(program, transform, height_framebuffer);
        }

        fn draw_normal(
            &self,
            program: &LumenpyxProgram,
            transform: &Transform,
            normal_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        ) {
            self.lumen_animation
                .draw_normal(program, transform, normal_framebuffer);
        }

        fn draw_roughness(
            &self,
            program: &LumenpyxProgram,
            transform: &Transform,
            roughness_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        ) {
            self.lumen_animation
                .draw_roughness(program, transform, roughness_framebuffer);
        }

        fn set_transform(&mut self, transform: Transform) {
            self.lumen_animation.set_transform(transform);
        }

        fn try_load_shaders(&self, program: &mut LumenpyxProgram) {
            self.lumen_animation.try_load_shaders(program);
        }

        fn get_transform(&self) -> Transform {
            self.lumen_animation.get_transform()
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
        fn draw_albedo(
            &self,
            program: &LumenpyxProgram,
            transform: &Transform,
            albedo_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        ) {
            self.lumen_animation_state_machine
                .draw_albedo(program, transform, albedo_framebuffer);
        }

        fn draw_height(
            &self,
            program: &LumenpyxProgram,
            transform: &Transform,
            height_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        ) {
            self.lumen_animation_state_machine
                .draw_height(program, transform, height_framebuffer);
        }

        fn draw_normal(
            &self,
            program: &LumenpyxProgram,
            transform: &Transform,
            normal_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        ) {
            self.lumen_animation_state_machine
                .draw_normal(program, transform, normal_framebuffer);
        }

        fn draw_roughness(
            &self,
            program: &LumenpyxProgram,
            transform: &Transform,
            roughness_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        ) {
            self.lumen_animation_state_machine.draw_roughness(
                program,
                transform,
                roughness_framebuffer,
            );
        }

        fn set_transform(&mut self, transform: Transform) {
            self.lumen_animation_state_machine.set_transform(transform);
        }

        fn try_load_shaders(&self, program: &mut LumenpyxProgram) {
            self.lumen_animation_state_machine.try_load_shaders(program);
        }

        fn get_transform(&self) -> Transform {
            self.lumen_animation_state_machine.get_transform()
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
        fn draw_albedo(
            &self,
            program: &LumenpyxProgram,
            transform: &Transform,
            albedo_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        ) {
            self.lumen_circle
                .draw_albedo(program, transform, albedo_framebuffer);
        }

        fn draw_height(
            &self,
            program: &LumenpyxProgram,
            transform: &Transform,
            height_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        ) {
            self.lumen_circle
                .draw_height(program, transform, height_framebuffer);
        }

        fn draw_normal(
            &self,
            program: &LumenpyxProgram,
            transform: &Transform,
            normal_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        ) {
            self.lumen_circle
                .draw_normal(program, transform, normal_framebuffer);
        }

        fn draw_roughness(
            &self,
            program: &LumenpyxProgram,
            transform: &Transform,
            roughness_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        ) {
            self.lumen_circle
                .draw_roughness(program, transform, roughness_framebuffer);
        }

        fn set_transform(&mut self, transform: Transform) {
            self.lumen_circle.set_transform(transform);
        }

        fn try_load_shaders(&self, program: &mut LumenpyxProgram) {
            self.lumen_circle.try_load_shaders(program);
        }

        fn get_transform(&self) -> Transform {
            self.lumen_circle.get_transform()
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
        fn draw_albedo(
            &self,
            program: &LumenpyxProgram,
            transform: &Transform,
            albedo_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        ) {
            self.lumen_rectangle
                .draw_albedo(program, transform, albedo_framebuffer);
        }

        fn draw_height(
            &self,
            program: &LumenpyxProgram,
            transform: &Transform,
            height_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        ) {
            self.lumen_rectangle
                .draw_height(program, transform, height_framebuffer);
        }

        fn draw_normal(
            &self,
            program: &LumenpyxProgram,
            transform: &Transform,
            normal_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        ) {
            self.lumen_rectangle
                .draw_normal(program, transform, normal_framebuffer);
        }

        fn draw_roughness(
            &self,
            program: &LumenpyxProgram,
            transform: &Transform,
            roughness_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        ) {
            self.lumen_rectangle
                .draw_roughness(program, transform, roughness_framebuffer);
        }

        fn set_transform(&mut self, transform: Transform) {
            self.lumen_rectangle.set_transform(transform);
        }

        fn try_load_shaders(&self, program: &mut LumenpyxProgram) {
            self.lumen_rectangle.try_load_shaders(program);
        }

        fn get_transform(&self) -> Transform {
            self.lumen_rectangle.get_transform()
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

    #[derive(Clone, Copy)]
    pub struct Sprite {
        lumen_sprite: lumenpyx::primitives::Sprite,
    }

    impl Sprite {
        pub fn new(
            albedo: TextureInput,
            height: TextureInput,
            roughness: TextureInput,
            normal: NormalInput,
            program: &mut LumenpyxProgram,
        ) -> (
            Self,
            TextureHandle,
            TextureHandle,
            TextureHandle,
            TextureHandle,
        ) {
            let (sprite, albedo, height, roughness, normal) = lumenpyx::primitives::Sprite::new(
                albedo,
                height,
                roughness,
                normal,
                program,
                Transform::default(),
            );

            (
                Self {
                    lumen_sprite: sprite,
                },
                albedo,
                height,
                roughness,
                normal,
            )
        }
    }

    impl Drawable for Sprite {
        fn draw_albedo(
            &self,
            program: &LumenpyxProgram,
            transform: &Transform,
            albedo_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        ) {
            self.lumen_sprite
                .draw_albedo(program, transform, albedo_framebuffer);
        }

        fn draw_height(
            &self,
            program: &LumenpyxProgram,
            transform: &Transform,
            height_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        ) {
            self.lumen_sprite
                .draw_height(program, transform, height_framebuffer);
        }

        fn draw_normal(
            &self,
            program: &LumenpyxProgram,
            transform: &Transform,
            normal_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        ) {
            self.lumen_sprite
                .draw_normal(program, transform, normal_framebuffer);
        }

        fn draw_roughness(
            &self,
            program: &LumenpyxProgram,
            transform: &Transform,
            roughness_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        ) {
            self.lumen_sprite
                .draw_roughness(program, transform, roughness_framebuffer);
        }

        fn set_transform(&mut self, transform: Transform) {
            self.lumen_sprite.set_transform(transform);
        }

        fn try_load_shaders(&self, program: &mut LumenpyxProgram) {
            self.lumen_sprite.try_load_shaders(program);
        }

        fn get_transform(&self) -> Transform {
            self.lumen_sprite.get_transform()
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
        fn draw_albedo(
            &self,
            program: &LumenpyxProgram,
            transform: &Transform,
            albedo_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        ) {
            self.lumen_cylinder
                .draw_albedo(program, transform, albedo_framebuffer);
        }

        fn draw_height(
            &self,
            program: &LumenpyxProgram,
            transform: &Transform,
            height_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        ) {
            self.lumen_cylinder
                .draw_height(program, transform, height_framebuffer);
        }

        fn draw_normal(
            &self,
            program: &LumenpyxProgram,
            transform: &Transform,
            normal_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        ) {
            self.lumen_cylinder
                .draw_normal(program, transform, normal_framebuffer);
        }

        fn draw_roughness(
            &self,
            program: &LumenpyxProgram,
            transform: &Transform,
            roughness_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        ) {
            self.lumen_cylinder
                .draw_roughness(program, transform, roughness_framebuffer);
        }

        fn set_transform(&mut self, transform: Transform) {
            self.lumen_cylinder.set_transform(transform);
        }

        fn try_load_shaders(&self, program: &mut LumenpyxProgram) {
            self.lumen_cylinder.try_load_shaders(program);
        }

        fn get_transform(&self) -> Transform {
            self.lumen_cylinder.get_transform()
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
        fn draw_albedo(
            &self,
            program: &LumenpyxProgram,
            transform: &Transform,
            albedo_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        ) {
            self.lumen_sphere
                .draw_albedo(program, transform, albedo_framebuffer);
        }

        fn draw_height(
            &self,
            program: &LumenpyxProgram,
            transform: &Transform,
            height_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        ) {
            self.lumen_sphere
                .draw_height(program, transform, height_framebuffer);
        }

        fn draw_normal(
            &self,
            program: &LumenpyxProgram,
            transform: &Transform,
            normal_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        ) {
            self.lumen_sphere
                .draw_normal(program, transform, normal_framebuffer);
        }

        fn draw_roughness(
            &self,
            program: &LumenpyxProgram,
            transform: &Transform,
            roughness_framebuffer: &mut glium::framebuffer::SimpleFrameBuffer,
        ) {
            self.lumen_sphere
                .draw_roughness(program, transform, roughness_framebuffer);
        }

        fn set_transform(&mut self, transform: Transform) {
            self.lumen_sphere.set_transform(transform);
        }

        fn try_load_shaders(&self, program: &mut LumenpyxProgram) {
            self.lumen_sphere.try_load_shaders(program);
        }

        fn get_transform(&self) -> Transform {
            self.lumen_sphere.get_transform()
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

        fn get_transform(&self) -> Transform {
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

        fn get_transform(&self) -> Transform {
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

        fn get_transform(&self) -> Transform {
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
