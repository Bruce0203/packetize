pub trait Packet<T> {
    fn get_id(state: &T) -> Option<u32>;
    fn is_changing_state() -> Option<T>;
}
