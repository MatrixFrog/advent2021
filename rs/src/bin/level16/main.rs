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

#[derive(Debug)]
struct Packet {
  version: u32,
  type_id: u32,
  contents: PacketContents,
}

#[derive(Debug)]
enum PacketContents {
  Literal(u32),
  Operator(Vec<Packet>),
}

fn parse_literal_groups(groups: &[u32]) -> u32 {
  let mut result: u32 = 0;
  for group in groups {
    result <<= 4;
    result += group & 0xF;
  }
  result
}

fn parse_int(input: &mut impl Iterator<Item = bool>, bits: u8) -> u32 {
  let mut result = 0;
  for _ in 0..bits {
    result <<= 1;
    result += if input.next().unwrap() { 1 } else { 0 };
  }
  result
}

fn parse_packet(input: &mut impl Iterator<Item = bool>) -> Packet {
  let mut version = parse_int(input, 3);
  let mut type_id = parse_int(input, 3);

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

      panic!();
    }
  }
}

fn main() {
  let mut input = input_to_bits().into_iter();
  parse_packet(&mut input);
}
