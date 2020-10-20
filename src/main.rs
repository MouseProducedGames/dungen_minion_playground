// External includes.
use dungen_minion::geometry::*;
use dungen_minion::*;

// Standard includes.
use std::collections::HashSet;

// Internal includes.

#[allow(clippy::borrowed_box)]
fn draw_map(map_id: MapId, drawn: &mut HashSet<MapId>) {
    if drawn.contains(&map_id) {
        return;
    }
    drawn.insert(map_id);

    println!("Map");
    let maps = MAPS.read();
    let map = maps[map_id].read();
    println!("{}", map.area());
    for y in map.area().position().y()..=map.area().bottom() {
        for x in map.area().position().x()..=map.area().right() {
            let tile_type = map.tile_type_at(Position::new(x, y));

            let ch = match tile_type {
                Some(TileType::Void) => ' ',
                Some(TileType::Floor) => '.',
                Some(TileType::Wall) => '#',
                Some(TileType::Portal) => '+',
                None => ' ',
            };

            print!("{}", ch);
        }
        println!();
    }

    for portal in map.portals() {
        draw_placed_map(portal.target(), drawn);
    }
}

#[allow(clippy::borrowed_box)]
fn draw_placed_map(map_id: MapId, drawn: &mut HashSet<MapId>) {
    if drawn.contains(&map_id) {
        return;
    }
    drawn.insert(map_id);

    println!("Map");
    let maps = MAPS.read();
    let map = maps[map_id].read();
    println!("{}", map.area());
    for y in map.area().position().y()..=map.area().bottom() {
        for x in map.area().position().x()..=map.area().right() {
            let tile_type = map.tile_type_at(Position::new(x, y));

            let ch = match tile_type {
                Some(TileType::Void) => ' ',
                Some(TileType::Floor) => '.',
                Some(TileType::Wall) => '#',
                Some(TileType::Portal) => '+',
                None => ' ',
            };

            print!("{}", ch);
        }
        println!();
    }

    for portal in map.portals() {
        draw_placed_map(portal.target(), drawn);
    }
}

fn main() {
    let num_portals = CountRange::new(2, 5).provide_count();
    let map_id = DunGen::new(SparseMap::new())
        .gen_with(SequentialGenerator::new(&[
            &EmptyRoomGenerator::new(Size::new(12, 8)),
            &WalledRoomGenerator::new(Size::zero()),
            &EdgePortalsGenerator::new(num_portals, Box::new(SparseMap::new)),
        ]))
        .gen_with(TraversePortalsGenerator::new(SequentialGenerator::new(&[
            &EmptyRoomGenerator::new(Size::new(8, 6)),
            &WalledRoomGenerator::new(Size::zero()),
        ])))
        .gen_with(TraverseThisAndPortalsGenerator::new(
            ReciprocatePortalsGenerator::new(),
        ))
        .gen_with(MergePortalMapsAsSubMapsGenerator::new(|_| true))
        .build();

    draw_map(map_id, &mut HashSet::new());
}
