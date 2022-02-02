use std::iter::Peekable;

fn raw_input() -> &'static str {
  include_str!("input.txt")
}

fn parse_hex_digit(byte: u8) -> u8 {
  match byte {
    b'0' => 0,
    b'1' => 1,
    b'2' => 2,
    b'3' => 3,
    b'4' => 4,
    b'5' => 5,
    b'6' => 6,
    b'7' => 7,
    b'8' => 8,
    b'9' => 9,
    b'a' | b'A' => 0xA,
    b'b' | b'B' => 0xB,
    b'c' | b'C' => 0xC,
    b'd' | b'D' => 0xD,
    b'e' | b'E' => 0xE,
    b'f' | b'F' => 0xF,
    _ => panic!("{}", byte),
  }
}

fn input_to_bits() -> Vec<bool> {
  let mut bits = vec![];
  for hex_digit in raw_input().trim().bytes() {
    let hex_number = parse_hex_digit(hex_digit);
    bits.push(hex_number & 0x8 != 0);
    bits.push(hex_number & 0x4 != 0);
    bits.push(hex_number & 0x2 != 0);
    bits.push(hex_number & 0x1 != 0);
  }
  bits
}

struct Packet {
  version: u64,
  type_id: u64,
  contents: PacketContents,
}

impl std::fmt::Debug for Packet {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Packet")
      .field("type_id", &self.type_id)
      .field("contents", &self.contents)
      .finish()
  }
}
enum PacketContents {
  Literal(u64),
  Operator(Vec<Packet>),
}

impl std::fmt::Debug for PacketContents {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Literal(v) => f.debug_tuple("L").field(v).finish(),
      Self::Operator(packets) => f.debug_tuple("O").field(packets).finish(),
    }
  }
}

impl Packet {
  fn evaluate(&self) -> u64 {
    match &self.contents {
      PacketContents::Literal(value) => {
        assert_eq!(self.type_id, 4);
        return *value;
      }
      PacketContents::Operator(subpackets) => {
        let subpacket_values = subpackets.iter().map(|p| p.evaluate());
        match self.type_id {
          0 => subpacket_values.sum(),
          1 => subpacket_values.product(),
          2 => subpacket_values.min().unwrap(),
          3 => subpacket_values.max().unwrap(),
          5 => {
            assert_eq!(subpackets.len(), 2);
            if subpackets[0].evaluate() > subpackets[1].evaluate() {
              1
            } else {
              0
            }
          }
          6 => {
            assert_eq!(subpackets.len(), 2);
            if subpackets[0].evaluate() < subpackets[1].evaluate() {
              1
            } else {
              0
            }
          }
          7 => {
            assert_eq!(subpackets.len(), 2);
            if subpackets[0].evaluate() == subpackets[1].evaluate() {
              1
            } else {
              0
            }
          }
          _ => panic!("Invalid packet: {:?}", self),
        }
      }
    }
  }
}

fn parse_literal_groups(groups: &[u64]) -> u64 {
  let mut result: u64 = 0;
  for group in groups {
    result <<= 4;
    result += group & 0xF;
  }
  result
}

fn parse_int(input: &mut impl Iterator<Item = bool>, bits: u8) -> u64 {
  let mut result = 0;
  for _ in 0..bits {
    result <<= 1;
    result += if input.next().unwrap() { 1 } else { 0 };
  }
  result
}

fn parse_packet(input: &mut impl Iterator<Item = bool>) -> Packet {
  let version = parse_int(input, 3);
  let type_id = parse_int(input, 3);

  if type_id == 4 {
    // Literal
    let mut groups = vec![];
    loop {
      let group = parse_int(input, 5);
      groups.push(group);
      if group & 0x10 == 0 {
        break;
      }
    }
    return Packet {
      version,
      type_id,
      contents: PacketContents::Literal(parse_literal_groups(&groups)),
    };
  } else {
    let length_type_id = input.next().unwrap();
    if length_type_id {
      let num_subpackets = parse_int(input, 11);
      let mut subpackets = vec![];
      for _ in 0..num_subpackets {
        subpackets.push(parse_packet(input));
      }
      return Packet {
        version,
        type_id,
        contents: PacketContents::Operator(subpackets),
      };
    } else {
      let length_of_subpackets_in_bits = parse_int(input, 15);
      let mut bits_for_subpackets = Vec::new();
      for _ in 0..length_of_subpackets_in_bits {
        bits_for_subpackets.push(input.next().unwrap());
      }
      let subpackets = parse_packets(&mut bits_for_subpackets.into_iter().peekable());
      Packet {
        version,
        type_id,
        contents: PacketContents::Operator(subpackets),
      }
    }
  }
}

fn parse_packets(input: &mut Peekable<impl Iterator<Item = bool>>) -> Vec<Packet> {
  let mut packets = vec![];
  while input.peek().is_some() {
    packets.push(parse_packet(input));
  }
  packets
}

fn calc_version(packet: &Packet) -> u64 {
  let mut version = packet.version;
  match &packet.contents {
    PacketContents::Literal(_) => {}
    PacketContents::Operator(subpackets) => {
      for p in subpackets {
        version += calc_version(&p);
      }
    }
  }
  version
}

fn main() {
  let mut input = input_to_bits().into_iter().peekable();
  let packet = parse_packet(&mut input);
  println!("part 1: {}", calc_version(&packet));
  println!("part 2: {}", packet.evaluate());
}
