#![warn(clippy::all, clippy::pedantic)]
#![cfg_attr(feature = "strict", deny(warnings))]

// External includes.
#[allow(clippy::wildcard_imports)]
use dungen_minion::geometry::*;
#[allow(clippy::wildcard_imports)]
use dungen_minion::*;

// Standard includes.
use std::collections::HashSet;

// Internal includes.

#[allow(clippy::borrowed_box)]
#[allow(dead_code)]
fn draw_map(map_id: MapId, drawn: &mut HashSet<MapId>) {
    if drawn.contains(&map_id) {
        return;
    }
    drawn.insert(map_id);

    println!("Map");
    let maps = MAPS.read();
    let map = maps[map_id].read();
    println!("{}", map.area());
    for y in map.top()..=map.area().bottom() {
        for x in map.left()..=map.area().right() {
            let tile_type = map.tile_type_at(Position::new(x, y));

            let ch = match tile_type {
                Some(TileType::Void) | None => ' ',
                Some(TileType::Floor) => '.',
                Some(TileType::Wall) => '#',
                Some(TileType::Portal) => '+',
            };

            print!("{}", ch);
        }
        println!();
    }

    /* for portal in map.portals() {
        draw_placed_map(portal.target(), drawn);
    } */
}

#[allow(clippy::borrowed_box)]
#[allow(dead_code)]
fn draw_placed_map(map_id: MapId, drawn: &mut HashSet<MapId>) {
    if drawn.contains(&map_id) {
        return;
    }
    drawn.insert(map_id);

    println!("Map");
    let maps = MAPS.read();
    let map = maps[map_id].read();
    println!("{}", map.area());
    for y in map.top()..=map.area().bottom() {
        for x in map.left()..=map.area().right() {
            let tile_type = map.tile_type_at(Position::new(x, y));

            let ch = match tile_type {
                Some(TileType::Void) | None => ' ',
                Some(TileType::Floor) => '.',
                Some(TileType::Wall) => '#',
                Some(TileType::Portal) => '+',
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
    let values: Box<[(Inclusion, Box<dyn PlacedShape>); 2]> = Box::new([
        (
            Inclusion::Include,
            Box::new(Oval::new(Position::new(0, 0), Size::new(22, 22))),
        ),
        (
            Inclusion::Exclude,
            Box::new(InvertPlacedShape::new(Oval::new(
                Position::new(6, 6),
                Size::new(10, 10),
            ))),
        ),
    ]);

    let its_a_big_donut = PlacedShapeSlice::new(values);

    let map_id = DunGen::new(SparseMap::new())
        .gen_with(&EmptyRoomGenerator::new(its_a_big_donut.clone()))
        .gen_with(&WalledRoomGenerator::new(its_a_big_donut))
        .build();

    /* let values: Box<[(Inclusion, Box<dyn PlacedShape>); 2]> = Box::new([
        (
            Inclusion::Include,
            Box::new(Area::new(Position::new(2, 0), Size::new(5, 9))),
        ),
        (
            Inclusion::Include,
            Box::new(Area::new(Position::new(0, 2), Size::new(9, 5))),
        ),
    ]);

    let its_a_big_plus = PlacedShapeSlice::new(values);

    let map_id = DunGen::new(SparseMap::new())
        .gen_with(&EmptyRoomGenerator::new(its_a_big_plus.clone()))
        .gen_with(&WalledRoomGenerator::new(its_a_big_plus))
        .build(); */

    /* let map_id = DunGen::new(SparseMap::new())
    .gen_with(&EmptyRoomGenerator::new(Oval::new(
        Position::new(0, 0),
        Size::new(22, 22),
    )))
    .gen_with(&WalledRoomGenerator::new(Oval::new(
        Position::new(0, 0),
        Size::new(22, 22),
    )))
    .build(); */

    /* let map_id = DunGen::new(SparseMap::new())
    .gen_with(&EmptyRoomGenerator::new(Size::new(10, 10)))
    .gen_with(&WalledRoomGenerator::new(Size::zero()))
    .build(); */

    /* {
        let maps = MAPS.read();
        let mut map = maps[map_id].write();
        for y in 4..=5 {
            for x in 4..=5 {
                map.tile_type_at_local_set(Position::new(x, y), TileType::Void);
            }
        }
    } */

    /* let map_id = DunGen::new(map_id)
    .gen_with(&WalledRoomGenerator::new(Size::zero()))
    .build(); */

    /* let num_portals = CountRange::new(2, 5).provide_count();
    let map_id = DunGen::new(SparseMap::new())
        .gen_with(&SequentialGenerator::new(&[
            &EmptyRoomGenerator::new(Size::new(12, 8)),
            &EdgePortalsGenerator::new(num_portals, Box::new(SparseMap::new)),
            // &EdgePortalsGenerator::new(1, Box::new(SparseMap::new)),
        ]))
        .gen_with(&TraversePortalsGenerator::new(EmptyRoomGenerator::new(
            Size::new(8, 6),
        )))
        .gen_with(&TraversePortalsGenerator::new(IfMapThenGenerator::new(
            |map_id| {
                let maps = MAPS.read();
                let map = maps[map_id].read();
                map.portal_count() == 0
            },
            EdgePortalsGenerator::new(CountRange::new(2, 5), Box::new(SparseMap::new)),
            // EdgePortalsGenerator::new(1, Box::new(SparseMap::new)),
        )))
        .gen_with(&TraversePortalsGenerator::new(IfMapThenGenerator::new(
            |map_id| {
                let maps = MAPS.read();
                let map = maps[map_id].read();
                *map.size() == Size::zero()
            },
            EmptyRoomGenerator::new(SizeRange::new(Size::new(6, 6), Size::new(12, 12))),
        )))
        .gen_with(&TraverseThisAndPortalsGenerator::new(
            SequentialGenerator::new(&[
                &WalledRoomGenerator::new(Size::zero()),
                &ReciprocatePortalsGenerator::new(),
            ]),
        ))
        .gen_with(&MergePortalMapsAsSubMapsGenerator::new(2, |_| true))
        .gen_with(&FillTilesGenerator::new(
            Oval::new(Position::new(7, 7), Size::new(10, 10)),
            TileType::Void,
        ))
        .gen_with(&WalledRoomGenerator::new(InvertPlacedShape::new(
            Oval::new(Position::new(7, 7), Size::new(10, 10)),
        )))
        .build(); */

    draw_map(map_id, &mut HashSet::new());
}
