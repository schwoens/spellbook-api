use serde::Serialize;

pub trait IntoResource<T> {
    fn into_resource(self) -> T;
}

pub trait IntoCollection<T> {
    fn into_collection(self) -> Vec<T>;
}

#[derive(Serialize)]
pub struct SpellResource {
    pub name: String,
    pub level: String,
    pub casting_time: String,
    pub magic_school: String,
    pub concentration: bool,
    pub range: String,
    pub duration: String,
}
