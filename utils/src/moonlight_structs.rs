pub mod moonlight_structs {
    use bincode::{serialize, deserialize};
    use rand::*;
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
    pub struct Player {
        pub player_id: u32,
        pub player_name: String,
        pub pos_x: i32,
        pub pos_y: i32
    }

    pub trait PlayerTrait {
        fn new(player_name: String) -> Player;
    }

    impl PlayerTrait for Player {
        fn new(player_name: String) -> Player {
            let mut rng = rand::thread_rng();
            let player_id = rng.gen_range(1..9999);
            let pos_x = 200;
            let pos_y = 150;
            Player {
                player_id,
                player_name,
                pos_x,
                pos_y
            }
        }
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    pub struct Message {
        pub message_id: u32,
        pub message_type: u8,
        pub player: Player
    }

    pub trait Messaging {
        fn serialize_moon(&self) -> Vec<u8>;
        fn deserialize_moon(bytes: Vec<u8>) -> Message;
    }

    impl Messaging for Message {
        fn serialize_moon(&self) -> Vec<u8> {
            let vec: Vec<u8> = serialize(&self).unwrap();
            return vec;
        }
        fn deserialize_moon(bytes: Vec<u8>) -> Message {
            let decoded_msg: Message = deserialize(&bytes).unwrap();
            print!("Deserialized message {} from player {}. Coords: {} {}.", 
                decoded_msg.message_id,
                decoded_msg.player.player_id,
                decoded_msg.player.pos_x,
                decoded_msg.player.pos_y);
            return decoded_msg;
        }
    }

    #[cfg(test)]
    mod tests {
        use bincode::{serialize, deserialize};
        use super::*;

        #[test]
        fn encode_decode() {
            let msg = create_message();
            let encoded: Vec<u8> = serialize(&msg).unwrap();
            let decoded_msg: Message = deserialize(&encoded).unwrap();

            assert_eq!(msg.message_id, decoded_msg.message_id);
        }
        #[test]
        fn self_encode_decode() {
            let msg = create_message();
            let serialized_vec: Vec<u8> = msg.serialize_moon();
            let deserialized_msg: Message = Message::deserialize_moon(serialized_vec.clone());

            println!("Deserialized message {} from player {}. Buffer size {}.", 
                deserialized_msg.message_id,
                deserialized_msg.player.player_id,
                serialized_vec.len());
            assert_eq!(msg.message_id, deserialized_msg.message_id);
            assert_eq!(msg.player.player_id, deserialized_msg.player.player_id);
        }

        fn create_player() -> Player {
            let player = Player {
                player_id: 1,
                player_name: String::from("Beto"),
                pos_x: 25,
                pos_y: 50,
            };
            return player;
        }

        fn create_message() -> Message {
            let player = create_player();
            let msg = Message {
                message_id: 501,
                message_type: 1,
                player
            };
            return msg;
        }
    }
}