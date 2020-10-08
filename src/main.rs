// External includes.

// Standard includes.

// Internal includes.
use generic_dungen::*;
use traits::*;

fn main() {
    let dungen = DunGen::new(RoomHashMap::default())
        .gen_with(EmptyRoomDunGen::new(Size::new(80, 40)))
        .gen::<WalledRoomDunGen>()
        .build();

    for y in 0..dungen.size().height() {
        for x in 0..dungen.size().width() {
            let tile_type = dungen.tile_type_at_local(LocalPosition::new(x, y));
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
}
