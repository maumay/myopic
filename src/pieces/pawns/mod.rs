use crate::base::{Side, Side::Black, Side::White};
use crate::base::bitboard::{BitBoard, simple::*};
use crate::base::square::constants::SQUARES;
use crate::base::square::Square;

pub mod black;
pub mod white;

///  Control sets for all squares for the white pawn.
const WHITE_CONTROL: [BitBoard; 64] = [
    BitBoard(512),
    BitBoard(1280),
    BitBoard(2560),
    BitBoard(5120),
    BitBoard(10240),
    BitBoard(20480),
    BitBoard(40960),
    BitBoard(16384),
    BitBoard(131072),
    BitBoard(327680),
    BitBoard(655360),
    BitBoard(1310720),
    BitBoard(2621440),
    BitBoard(5242880),
    BitBoard(10485760),
    BitBoard(4194304),
    BitBoard(33554432),
    BitBoard(83886080),
    BitBoard(167772160),
    BitBoard(335544320),
    BitBoard(671088640),
    BitBoard(1342177280),
    BitBoard(2684354560),
    BitBoard(1073741824),
    BitBoard(8589934592),
    BitBoard(21474836480),
    BitBoard(42949672960),
    BitBoard(85899345920),
    BitBoard(171798691840),
    BitBoard(343597383680),
    BitBoard(687194767360),
    BitBoard(274877906944),
    BitBoard(2199023255552),
    BitBoard(5497558138880),
    BitBoard(10995116277760),
    BitBoard(21990232555520),
    BitBoard(43980465111040),
    BitBoard(87960930222080),
    BitBoard(175921860444160),
    BitBoard(70368744177664),
    BitBoard(562949953421312),
    BitBoard(1407374883553280),
    BitBoard(2814749767106560),
    BitBoard(5629499534213120),
    BitBoard(11258999068426240),
    BitBoard(22517998136852480),
    BitBoard(45035996273704960),
    BitBoard(18014398509481984),
    BitBoard(144115188075855872),
    BitBoard(360287970189639680),
    BitBoard(720575940379279360),
    BitBoard(1441151880758558720),
    BitBoard(2882303761517117440),
    BitBoard(5764607523034234880),
    BitBoard(11529215046068469760),
    BitBoard(4611686018427387904),
    BitBoard(0),
    BitBoard(0),
    BitBoard(0),
    BitBoard(0),
    BitBoard(0),
    BitBoard(0),
    BitBoard(0),
    BitBoard(0),
];

///  Control sets for all squares for the black pawn.
const BLACK_CONTROL: [BitBoard; 64] = [
    BitBoard(0),
    BitBoard(0),
    BitBoard(0),
    BitBoard(0),
    BitBoard(0),
    BitBoard(0),
    BitBoard(0),
    BitBoard(0),
    BitBoard(2),
    BitBoard(5),
    BitBoard(10),
    BitBoard(20),
    BitBoard(40),
    BitBoard(80),
    BitBoard(160),
    BitBoard(64),
    BitBoard(512),
    BitBoard(1280),
    BitBoard(2560),
    BitBoard(5120),
    BitBoard(10240),
    BitBoard(20480),
    BitBoard(40960),
    BitBoard(16384),
    BitBoard(131072),
    BitBoard(327680),
    BitBoard(655360),
    BitBoard(1310720),
    BitBoard(2621440),
    BitBoard(5242880),
    BitBoard(10485760),
    BitBoard(4194304),
    BitBoard(33554432),
    BitBoard(83886080),
    BitBoard(167772160),
    BitBoard(335544320),
    BitBoard(671088640),
    BitBoard(1342177280),
    BitBoard(2684354560),
    BitBoard(1073741824),
    BitBoard(8589934592),
    BitBoard(21474836480),
    BitBoard(42949672960),
    BitBoard(85899345920),
    BitBoard(171798691840),
    BitBoard(343597383680),
    BitBoard(687194767360),
    BitBoard(274877906944),
    BitBoard(2199023255552),
    BitBoard(5497558138880),
    BitBoard(10995116277760),
    BitBoard(21990232555520),
    BitBoard(43980465111040),
    BitBoard(87960930222080),
    BitBoard(175921860444160),
    BitBoard(70368744177664),
    BitBoard(562949953421312),
    BitBoard(1407374883553280),
    BitBoard(2814749767106560),
    BitBoard(5629499534213120),
    BitBoard(11258999068426240),
    BitBoard(22517998136852480),
    BitBoard(45035996273704960),
    BitBoard(18014398509481984),
];

//fn compute_all_empty_board_control(side: Side) -> Vec<BitBoard> {
//    SQUARES
//        .iter()
//        .map(|&sq| compute_empty_board_control(side, sq))
//        .collect()
//}
//
//fn compute_empty_board_control(side: Side, loc: Square) -> BitBoard {
//    let (x, left, right) = (loc.lift(), FILES[7], FILES[0]);
//    match side {
//        White => ((x - left) << 9u8) | ((x - right) << 7u8),
//        Black => ((x - left) >> 7u8) | ((x - right) >> 9u8),
//    }
//}
