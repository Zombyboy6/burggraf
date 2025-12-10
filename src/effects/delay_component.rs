use std::{marker::PhantomData, time::Duration};

use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    prelude::*,
};

pub struct DelayComponentPlugin;

impl Plugin for DelayComponentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, delay_component_system);
    }
}

#[derive(Debug, Component, Deref, DerefMut)]
pub struct DelayObserver(Timer);

#[derive(Debug, EntityEvent)]
pub struct DelayEvent(Entity);

/// This triggers `DelayEvent` on that entity after the specified duration.
impl DelayObserver {
    pub fn new(duration: Duration) -> Self {
        Self(Timer::new(duration, TimerMode::Once))
    }
}

#[derive(Component, Clone, Debug)]
#[component(on_add = Self::on_add)]
pub struct DelayInsert<T>
where
    T: Component + Clone,
{
    timer: Timer,
    component: T,
}

impl<T> DelayInsert<T>
where
    T: Component + Clone,
{
    pub fn new(delay: Duration, component: T) -> Self {
        Self {
            timer: Timer::new(delay, TimerMode::Once),
            component,
        }
    }

    fn on_add(mut world: DeferredWorld, hook_context: HookContext) {
        let delay_insert = world
            .get::<DelayInsert<T>>(hook_context.entity)
            .unwrap()
            .to_owned();

        world
            .commands()
            .entity(hook_context.entity)
            .insert((DelayObserver(delay_insert.timer.clone()),))
            .observe(move |delay: On<DelayEvent>, mut commands: Commands| {
                commands
                    .entity(delay.event_target())
                    .insert(delay_insert.component.clone())
                    .remove::<DelayInsert<T>>();
            });
    }
}

#[derive(Component, Clone, Debug)]
#[component(on_add = Self::on_add)]
pub struct DelayRemove<T>
where
    T: Component,
{
    timer: Timer,
    component: PhantomData<T>,
}

impl<T> DelayRemove<T>
where
    T: Component,
{
    pub fn new(delay: Duration) -> Self {
        Self {
            timer: Timer::new(delay, TimerMode::Once),
            component: PhantomData,
        }
    }

    fn on_add(mut world: DeferredWorld, hook_context: HookContext) {
        let delay_insert_timer = world
            .get::<DelayRemove<T>>(hook_context.entity)
            .unwrap()
            .timer
            .to_owned();

        world
            .commands()
            .entity(hook_context.entity)
            .insert((DelayObserver(delay_insert_timer.clone()),))
            .observe(move |delay: On<DelayEvent>, mut commands: Commands| {
                commands
                    .entity(delay.event_target())
                    .remove::<T>()
                    .remove::<DelayRemove<T>>();
            });
    }
}

fn delay_component_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut DelayObserver)>,
    time: Res<Time>,
) {
    for (entity, mut delay) in query.iter_mut() {
        delay.tick(time.delta());
        if delay.just_finished() {
            commands
                .entity(entity)
                .trigger(DelayEvent)
                .remove::<DelayObserver>();
        }
    }
}
