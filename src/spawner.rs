use hecs::World;

pub struct SpawnEvent {
    pub start: f32,
    pub is_spawned: bool,
    pub action: Option<Box<dyn FnOnce(&mut World)>>,
}

#[derive(Default)]
pub struct Spawner {
    pub timer: f32,
    pub lists: Vec<SpawnEvent>,
}

impl Spawner {
    pub fn spawn(&mut self, start: f32, action: impl FnOnce(&mut World) + 'static) {
        self.lists.push(SpawnEvent {
            start,
            is_spawned: false,
            action: Some(Box::new(action)),
        });
    }

    pub fn update(&mut self, world: &mut World, time: f32) {
        self.timer += time;
        self.lists
            .iter_mut()
            .filter(|event| !event.is_spawned && event.start < self.timer)
            .for_each(|event| match event.action.take() {
                Some(action) => (action)(world),
                None => {}
            });

        self.lists.retain(|event| !event.is_spawned);
    }
}
