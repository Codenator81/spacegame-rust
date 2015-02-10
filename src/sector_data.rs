#[derive(Clone, PartialEq, Eq, Hash, RustcEncodable, RustcDecodable)]
pub struct SectorId(pub u32);

#[derive(Clone, RustcEncodable, RustcDecodable)]
pub struct SectorData {
    pub id: SectorId,
    pub map_x: f64,
    pub map_y: f64,
}