fn main() {
    enum Message {

        Move { x: i32, y: i32 },

    }

    let x: Message = Message::Move { x: 3, y: 4 };

    

    enum BoardGameTurn {

        Move { squares: i32 },

        Pass,

    }

    

    let y: BoardGameTurn = BoardGameTurn::Move { squares: 1 };

}
