// External includes.
use dungen_minion::geometry::*;
use dungen_minion::*;

// Standard includes.

// Internal includes.

#[allow(clippy::borrowed_box)]
fn draw_map(map_id: MapId) {
    let maps = MAPS.read();
    let map = maps[map_id].read();
    for y in map.area().position().y()..=map.area().bottom() {
        for x in map.area().position().x()..=map.area().right() {
            let tile_type = map.tile_type_at_local(Position::new(x, y));

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
        println!("Map");
        draw_placed_map(portal.target());
    }
}

#[allow(clippy::borrowed_box)]
fn draw_placed_map(map_id: MapId) {
    let maps = MAPS.read();
    let map = maps[map_id].read();
    for y in map.area().position().y()..=map.area().bottom() {
        for x in map.area().position().x()..=map.area().right() {
            let tile_type = map.tile_type_at_local(Position::new(x, y));

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
        println!("Map");
        draw_placed_map(portal.target());
    }
}

fn main() {
    // We could provide CountRange directly to EdgePortalsGenerator, but that would not let us
    // test that we have the right number of portals.
    // This CountRange will generate a number in the range [4, 9].
    let num_sub_maps = CountRange::new(4, 9).provide_count();
    let map_id = DunGen::new(SparseMap::new())
        .gen_with(FillTilesGenerator::new(Size::new(60, 30), TileType::Void))
        .gen_with(SubMapGenerator::new(
            &[SubMapGeneratorSet::new(
                &num_sub_maps,
                &Area::new(Position::new(0, 0), Size::new(48, 18)),
                Some(Box::new(SparseMap::new)),
                Some(&[&EmptyRoomGenerator::new(SizeRange::new(
                    Size::new(6, 6),
                    Size::new(12, 12),
                ))]),
            )],
            Some(Box::new(SparseMap::new)),
            Some(&[&WalledRoomGenerator::new(Size::zero())]),
        ))
        .build();

    println!("Map");
    draw_map(map_id);
}
