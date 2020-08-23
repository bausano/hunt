pub mod predator;
pub mod prey;

pub use predator::Predator;
pub use prey::Prey;

use bevy::prelude::*;

use crate::prelude::*;

pub fn move_prey(time: Res<Time>, mut prey_query: Query<(Entity, &mut Prey, &Translation)>) {
    struct PreyData<'a> {
        entity: Entity,
        rf: Mut<'a, Prey>,
        pos: Vec3,
    }

    let prey_iter = &mut prey_query.iter();
    let mut prey = Vec::with_capacity(conf::prey::COUNT);
    for (entity, p, translation) in prey_iter {
        prey.push(PreyData {
            entity,
            rf: p,
            pos: **translation,
        });
        //    let current_translation: Vec3 = **translation;
        //    let move_up = Vec3::new(5.0, 0.0, 0.0);
        //    *translation = (current_translation + move_up).into();
        //    for _ in &mut prey_query.iter() {
        //        //
        //    }
    }

    #[derive(Default)]
    struct PreyUpdate {
        // Could be usize, but f32 saves us a convertion.
        flockmates: f32,
        heading_total: Vec3,
        center_total: Vec3,
    }

    for prey_index in 0..prey.len() {
        let iterated_prey = &prey[prey_index];
        let mut update = PreyUpdate::default();
        for other_index in 0..prey.len() {
            if prey_index == other_index {
                continue;
            }
            let other_prey = &prey[other_index];
            let offset = iterated_prey.pos - other_prey.pos;
            let sq_distance = offset.length_squared();

            // TODO: Const
            if sq_distance < 50.0 {
                update.flockmates += 1.0;
                update.heading_total += other_prey.rf.vel;
                update.center_total += other_prey.pos;

                // TODO: Separation heading.
            }
        }

        if update.flockmates != 0.0 {
            let mut iterated_prey = &mut prey[prey_index];
            let offset_to_flock_center =
                (update.center_total / update.flockmates) - iterated_prey.pos;
            let cohesion_force = iterated_prey.rf.steer_towards(offset_to_flock_center);
        }
    }
}
