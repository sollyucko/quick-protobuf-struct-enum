extern crate quick_protobuf;

use quick_protobuf::{Reader, MessageRead, Writer, MessageWrite};

macro_rules! write_read_primitive {
    ($name:ident, $read:ident, $write:ident) => (write_read_primitive!($name, $read, $write, 145););
    ($name:ident, $read:ident, $write:ident, $def:expr) => (
#[test]
fn $name(){
    let v = $def;
    let mut buf = Vec::new();
    {
        let mut w = Writer::new(&mut buf);
        w.$write(v).unwrap();
    }
    let len = buf.len();
    let mut buf = &*buf;
    let mut r = Reader::from_reader(&mut buf, len);
    assert_eq!(v, r.$read().unwrap());
}
    );
}

write_read_primitive!(wr_int32, read_int32, write_int32);
write_read_primitive!(wr_int64, read_int64, write_int64);
write_read_primitive!(wr_uint32, read_uint32, write_uint32);
write_read_primitive!(wr_uint64, read_uint64, write_uint64);
write_read_primitive!(wr_sint32, read_sint32, write_sint32);
write_read_primitive!(wr_sint64, read_sint64, write_sint64);
write_read_primitive!(wr_bool, read_bool, write_bool, true);
write_read_primitive!(wr_fixed32, read_fixed32, write_fixed32);
write_read_primitive!(wr_fixed64, read_fixed64, write_fixed64);
write_read_primitive!(wr_sfixed32, read_sfixed32, write_sfixed32);
write_read_primitive!(wr_sfixed64, read_sfixed64, write_sfixed64);
write_read_primitive!(wr_float, read_float, write_float, 5.8);
write_read_primitive!(wr_double, read_double, write_double, 5.8);

#[test]
fn wr_bytes(){
    let v = b"test_write_read";
    let mut buf = Vec::new();
    {
        let mut w = Writer::new(&mut buf);
        w.write_bytes(v).unwrap();
    }
    let len = buf.len();
    let mut buf = &*buf;
    let mut r = Reader::from_reader(&mut buf, len);
    assert_eq!(v, &*r.read_bytes().unwrap());
}

#[test]
fn wr_string(){
    let v = "test_write_read";
    let mut buf = Vec::new();
    {
        let mut w = Writer::new(&mut buf);
        w.write_string(v).unwrap();
    }
    let len = buf.len();
    let mut buf = &*buf;
    let mut r = Reader::from_reader(&mut buf, len);
    assert_eq!(v, &*r.read_string().unwrap());
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum TestEnum {
    A = 0,
    B = 1,
    C = 2,
}

impl From<i32> for TestEnum {
    fn from(v: i32) -> TestEnum {
        match v {
            0 => TestEnum::A,
            1 => TestEnum::B,
            2 => TestEnum::C,
            _ => unreachable!(),
        }
    }
}

#[test]
fn wr_enum(){

    let v = TestEnum::C;
    let mut buf = Vec::new();
    {
        let mut w = Writer::new(&mut buf);
        w.write_enum(v as i32).unwrap();
    }
    let len = buf.len();
    let mut buf = &*buf;
    let mut r = Reader::from_reader(&mut buf, len);
    assert_eq!(v, r.read_enum().unwrap());
}

