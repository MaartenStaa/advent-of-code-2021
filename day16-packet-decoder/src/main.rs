fn main() {
    let packet = parse(include_str!("input.txt"));

    println!("Version sum: {}", version_sum(&packet));
    println!("Evaluation: {}", evaluate(&packet));
}

#[derive(Clone, Debug, PartialEq)]
struct Packet {
    version: usize,
    packet_type_id: usize,
    contents: PacketContents,
}

#[derive(Clone, Debug, PartialEq)]
enum PacketContents {
    Literal(usize),
    Operator(Vec<Packet>),
}

fn parse(input: &str) -> Packet {
    parse_binary_packet(&parse_hexadecimal(input)).0
}

fn parse_hexadecimal(input: &str) -> Vec<bool> {
    input
        .chars()
        .filter_map(|c| match c {
            '0' => Some([0, 0, 0, 0]),
            '1' => Some([0, 0, 0, 1]),
            '2' => Some([0, 0, 1, 0]),
            '3' => Some([0, 0, 1, 1]),
            '4' => Some([0, 1, 0, 0]),
            '5' => Some([0, 1, 0, 1]),
            '6' => Some([0, 1, 1, 0]),
            '7' => Some([0, 1, 1, 1]),
            '8' => Some([1, 0, 0, 0]),
            '9' => Some([1, 0, 0, 1]),
            'A' => Some([1, 0, 1, 0]),
            'B' => Some([1, 0, 1, 1]),
            'C' => Some([1, 1, 0, 0]),
            'D' => Some([1, 1, 0, 1]),
            'E' => Some([1, 1, 1, 0]),
            'F' => Some([1, 1, 1, 1]),
            _ => None,
        })
        .flatten()
        .map(|i| i == 1)
        .collect()
}

fn parse_binary_packet(input: &[bool]) -> (Packet, &[bool]) {
    let version = from_bits(&input[0..3]);
    let packet_type_id = from_bits(&input[3..6]);

    let (contents, rest) = if packet_type_id == 4 {
        parse_literal_packet(&input[6..])
    } else {
        parse_operator_packet(&input[6..])
    };

    (
        Packet {
            version,
            packet_type_id,
            contents,
        },
        rest,
    )
}

fn parse_literal_packet(input: &[bool]) -> (PacketContents, &[bool]) {
    let mut iter = input.iter().copied();
    let mut bits = vec![];
    loop {
        let more = iter.next().expect("There should be more bits left");
        for _ in 0..4 {
            bits.push(iter.next().expect("There should be more bits left"));
        }

        if !more {
            break;
        }
    }

    let rest = &input[bits.len() + bits.len() / 4..];
    (PacketContents::Literal(from_bits(&bits)), rest)
}

fn parse_operator_packet(input: &[bool]) -> (PacketContents, &[bool]) {
    let mut result = vec![];
    let mut rest: &[bool];
    if input[0] {
        let packets = from_bits(&input[1..12]);
        rest = &input[12..];
        for _ in 0..packets {
            let (sub, r) = parse_binary_packet(rest);
            result.push(sub);
            rest = r;
        }
    } else {
        let length = from_bits(&input[1..16]);
        rest = &input[16..16 + length];
        while !rest.is_empty() {
            let (sub, r) = parse_binary_packet(rest);
            result.push(sub);
            rest = r;
        }

        rest = &input[16 + length..];
    }

    (PacketContents::Operator(result), rest)
}

fn from_bits(input: &[bool]) -> usize {
    input
        .iter()
        .fold(0, |acc, b| (acc << 1) + if *b { 1 } else { 0 })
}

fn version_sum(packet: &Packet) -> usize {
    packet.version
        + match &packet.contents {
            PacketContents::Literal(_) => 0,
            PacketContents::Operator(contents) => contents.iter().map(version_sum).sum(),
        }
}

fn evaluate(packet: &Packet) -> usize {
    match &packet.contents {
        PacketContents::Literal(n) => *n,
        PacketContents::Operator(contents) => match &packet.packet_type_id {
            0 => contents.iter().map(evaluate).sum(),
            1 => contents.iter().map(evaluate).product(),
            2 => contents.iter().map(evaluate).min().unwrap_or_default(),
            3 => contents.iter().map(evaluate).max().unwrap_or_default(),
            5 => {
                if evaluate(&contents[0]) > evaluate(&contents[1]) {
                    1
                } else {
                    0
                }
            }
            6 => {
                if evaluate(&contents[0]) < evaluate(&contents[1]) {
                    1
                } else {
                    0
                }
            }
            7 => {
                if evaluate(&contents[0]) == evaluate(&contents[1]) {
                    1
                } else {
                    0
                }
            }
            _ => panic!("Invalid operator type: {}", packet.packet_type_id),
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        evaluate, from_bits, parse, parse_hexadecimal, version_sum, Packet, PacketContents,
    };

    fn print_bits(bits: &[bool]) -> String {
        bits.iter().map(|b| if *b { '1' } else { '0' }).collect()
    }

    #[test]
    fn test_parse_hexadecimal() {
        assert_eq!(
            "110100101111111000101000",
            &print_bits(&parse_hexadecimal("D2FE28"))
        )
    }

    #[test]
    fn test_from_bits() {
        assert_eq!(1, from_bits(&[true]));
        assert_eq!(1, from_bits(&[false, false, false, false, false, true]));
        assert_eq!(6, from_bits(&[true, true, false]));
        assert_eq!(4, from_bits(&[true, false, false]));
        assert_eq!(
            27,
            from_bits(&[
                false, false, false, false, false, false, false, false, false, false, true, true,
                false, true, true,
            ])
        );
    }

    #[test]
    fn test_parse_packet() {
        assert_eq!(
            Packet {
                version: 6,
                packet_type_id: 4,
                contents: PacketContents::Literal(2021)
            },
            parse("D2FE28")
        );
        assert_eq!(
            Packet {
                version: 1,
                packet_type_id: 6,
                contents: PacketContents::Operator(vec![
                    Packet {
                        version: 6,
                        packet_type_id: 4,
                        contents: PacketContents::Literal(10)
                    },
                    Packet {
                        version: 2,
                        packet_type_id: 4,
                        contents: PacketContents::Literal(20)
                    }
                ])
            },
            parse("38006F45291200")
        );
        assert_eq!(
            Packet {
                version: 7,
                packet_type_id: 3,
                contents: PacketContents::Operator(vec![
                    Packet {
                        version: 2,
                        packet_type_id: 4,
                        contents: PacketContents::Literal(1)
                    },
                    Packet {
                        version: 4,
                        packet_type_id: 4,
                        contents: PacketContents::Literal(2)
                    },
                    Packet {
                        version: 1,
                        packet_type_id: 4,
                        contents: PacketContents::Literal(3)
                    },
                ])
            },
            parse("EE00D40C823060")
        );
    }

    #[test]
    fn test_version_sum() {
        assert_eq!(16, version_sum(&parse("8A004A801A8002F478")));
        assert_eq!(12, version_sum(&parse("620080001611562C8802118E34")));
        assert_eq!(23, version_sum(&parse("C0015000016115A2E0802F182340")));
        assert_eq!(31, version_sum(&parse("A0016C880162017C3686B18A3D4780")));
    }

    #[test]
    fn test_evaluate() {
        assert_eq!(3, evaluate(&parse("C200B40A82")));
        assert_eq!(54, evaluate(&parse("04005AC33890")));
        assert_eq!(7, evaluate(&parse("880086C3E88112")));
        assert_eq!(9, evaluate(&parse("CE00C43D881120")));
        assert_eq!(1, evaluate(&parse("D8005AC2A8F0")));
        assert_eq!(0, evaluate(&parse("F600BC2D8F")));
        assert_eq!(0, evaluate(&parse("9C005AC2F8F0")));
        assert_eq!(1, evaluate(&parse("9C0141080250320F1802104A08")));
    }
}
