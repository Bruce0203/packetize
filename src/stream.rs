pub trait Packet<T> {
    fn get_id(state: &T) -> Option<u32>;
    // TODO rename to is_state_changing
    fn is_changing_state() -> Option<T>;
}
