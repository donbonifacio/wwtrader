pub trait Controller {
    fn get_type(self) -> ControllerType;
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum ControllerType {
    Player,
    Enemy,
}
