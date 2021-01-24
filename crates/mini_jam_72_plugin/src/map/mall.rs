use crate::map::{Coordinate, MapData, Npc};
use std::collections::HashMap;

pub fn get_mall_map() -> MapData {
    let mut path_map = HashMap::default();
    path_map.insert('#', "structure/woodenFloor.png".to_owned());
    path_map.insert('w', "structure/wallblue.png".to_owned());
    path_map.insert('1', "objects/bed_1.png".to_owned());
    path_map.insert('2', "objects/bed_2.png".to_owned());
    path_map.insert('3', "objects/bed_3.png".to_owned());
    path_map.insert('4', "objects/bed_4.png".to_owned());

    return MapData {
        layers: vec![
            "\
            #########\n\
            ####12###\n\
            ####43###\n\
            #########\n\
            #########\n\
            #########\n\
            #########\n\
            #########\n\
            #########\n\
            #########\n\
            #########"
                .to_owned(),
            "\
            wwwwwwwww\n\
            w.......w\n\
            w.......w\n\
            w.......w\n\
            w.......w\n\
            w.......w\n\
            w.......w\n\
            w.......w\n\
            w.......w\n\
            w.......w\n\
            wwwwwwwww"
                .to_owned(),
        ],
        path_map,
        colliding_layers: vec![1],
        npcs: vec![Npc {
            conversation_id: Some(1),
            position: Coordinate {
                x: 7. * 32.,
                y: 7. * 32.,
            },
        },Npc {
            conversation_id: None,
            position: Coordinate {
                x: 2. * 32.,
                y: 6. * 32.,
            },
        },Npc {
            conversation_id: Some(2),
            position: Coordinate {
                x: 3. * 32.,
                y: 2. * 32.,
            },
        }],
    };
}
