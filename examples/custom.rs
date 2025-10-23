use hardware_address::addr_ty;

addr_ty!(
  /// Represents an address.
  MyAddr[12]
);

fn main() {
  let addr = MyAddr::from_raw([
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b,
  ]);
  println!("{:?}", addr);
}
