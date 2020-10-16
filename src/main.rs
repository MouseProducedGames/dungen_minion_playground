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
            if tile_type.is_none() {
                continue;
            }

            let tile_type = tile_type.unwrap();
            let ch = match tile_type {
                TileType::Void => ' ',
                TileType::Floor => '.',
                TileType::Wall => '#',
                TileType::Portal => '+',
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
            if tile_type.is_none() {
                continue;
            }

            let tile_type = tile_type.unwrap();
            let ch = match tile_type {
                TileType::Void => ' ',
                TileType::Floor => '.',
                TileType::Wall => '#',
                TileType::Portal => '+',
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
        .gen_with(FillTilesGenerator::new(Size::new(40, 30), TileType::Void))
        .gen_with(SubMapGenerator::new(
            &[SubMapGeneratorSet::new(
                &num_sub_maps,
                &Area::new(Position::new(0, 0), Size::new(40, 30)),
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

    let maps = MAPS.read();
    let map = maps[map_id].read();

    assert!(*map.size() == Size::new(40, 30));
    assert!(map.sub_map_count() == num_sub_maps);
    assert!(map.sub_map_count() >= 4 && map.sub_map_count() <= 9);
    let mut sub_map_count = 0;
    for sub_map in map.sub_maps() {
        let target_map = maps[sub_map.value()].read();
        assert!(target_map.size().width() >= 6 && target_map.size().width() <= 12);
        assert!(target_map.size().height() >= 6 && target_map.size().height() <= 12);
        assert!(target_map.tile_type_at_local(Position::new(0, 0)) == Some(TileType::Wall));
        assert!(target_map.tile_type_at_local(Position::new(1, 1)) == Some(TileType::Floor));
        sub_map_count += 1;
    }
    assert!(sub_map_count == num_sub_maps);
    assert!(sub_map_count >= 4 && sub_map_count <= 9);

    println!("Map");
    draw_map(map_id);
}
