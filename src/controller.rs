use crate::stateful::{StatefulNestedRouter, StatefulRoutes};

pub struct Controller<T: StatefulNestedRouter<T>>(T);

pub fn get_controller_routes<T: StatefulNestedRouter<T>>(
    controllers: Vec<Controller<T>>,
) -> StatefulRoutes<T> {
    return controllers.into_iter().map(|_| T::get()).collect();
}
