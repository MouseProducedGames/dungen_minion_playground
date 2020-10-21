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
    let map_id = DunGen::new(SparseMap::new())
        .gen_with(EmptyRoomGenerator::new(Size::new(10, 10)))
        .gen_with(WalledRoomGenerator::new(Size::zero()))
        .build();

    draw_map(map_id, &mut HashSet::new());

    {
        let maps = MAPS.read();
        let mut map = maps[map_id].write();
        for y in 4..=5 {
            for x in 4..=5 {
                map.tile_type_at_local_set(Position::new(x, y), TileType::Void);
            }
        }
    }

    draw_map(map_id, &mut HashSet::new());

    let map_id = DunGen::new(map_id)
        .gen_with(WalledRoomGenerator::new(Size::zero()))
        .build();

    /*
    let num_portals = CountRange::new(2, 5).provide_count();
    let map_id = DunGen::new(SparseMap::new())
        .gen_with(SequentialGenerator::new(&[
            &EmptyRoomGenerator::new(Size::new(12, 8)),
            &EdgePortalsGenerator::new(num_portals, Box::new(SparseMap::new)),
            // &EdgePortalsGenerator::new(1, Box::new(SparseMap::new)),
        ]))
        .gen_with(TraversePortalsGenerator::new(EmptyRoomGenerator::new(
            Size::new(8, 6),
        )))
        .gen_with(TraversePortalsGenerator::new(IfMapThenGenerator::new(
            |map_id| {
                let maps = MAPS.read();
                let map = maps[map_id].read();
                map.portal_count() == 0
            },
            EdgePortalsGenerator::new(CountRange::new(2, 5), Box::new(SparseMap::new)),
            // EdgePortalsGenerator::new(1, Box::new(SparseMap::new)),
        )))
        .gen_with(TraversePortalsGenerator::new(IfMapThenGenerator::new(
            |map_id| {
                let maps = MAPS.read();
                let map = maps[map_id].read();
                *map.size() == Size::zero()
            },
            EmptyRoomGenerator::new(SizeRange::new(Size::new(6, 6), Size::new(12, 12))),
        )))
        .gen_with(TraverseThisAndPortalsGenerator::new(
            SequentialGenerator::new(&[
                &WalledRoomGenerator::new(Size::zero()),
                &ReciprocatePortalsGenerator::new(),
            ]),
        ))
        .gen_with(MergePortalMapsAsSubMapsGenerator::new(2, |_| true))
        .build();
    */

    draw_map(map_id, &mut HashSet::new());
}
