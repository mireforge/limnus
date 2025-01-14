use limnus_app::prelude::{App, Plugin};
use limnus_default_stages::{
    First, FixedFirst, FixedPostUpdate, FixedPreUpdate, FixedUpdate, PostUpdate, PreUpdate,
    RenderFirst, RenderPostUpdate, RenderPreUpdate, RenderUpdate, Update,
};

pub struct DefaultStagePlugin;

impl Plugin for DefaultStagePlugin {
    fn build(&self, app: &mut App) {
        app.add_stage(First);
        app.add_stage(PreUpdate);
        app.add_stage(Update);
        app.add_stage(PostUpdate);

        app.add_stage(FixedFirst);
        app.add_stage(FixedPreUpdate);
        app.add_stage(FixedUpdate);
        app.add_stage(FixedPostUpdate);

        app.add_stage(RenderFirst);
        app.add_stage(RenderPreUpdate);
        app.add_stage(RenderUpdate);
        app.add_stage(RenderPostUpdate);
    }
}
