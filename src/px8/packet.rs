use std::result::Result;

use std::io::Read;

use std::io;
use std::io::{Error, ErrorKind};
use std::default;
use std::fmt;

use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};

pub trait Serializable: Sized {
    fn read_from<R: io::Read>(buf: &mut R) -> Result<Self, Error>;
    fn write_to<W: io::Write>(&self, buf: &mut W) -> Result<(), Error>;
}


impl Serializable for () {
    fn read_from<R: io::Read>(_: &mut R) -> Result<(), Error> {
        Result::Ok(())
    }
    fn write_to<W: io::Write>(&self, _: &mut W) -> Result<(), Error> {
        Result::Ok(())
    }
}

impl Serializable for u8 {
    fn read_from<R: io::Read>(buf: &mut R) -> Result<u8, Error> {
        Result::Ok(try!(buf.read_u8()))
    }
    fn write_to<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        try!(buf.write_u8(*self));
        Result::Ok(())
    }
}

impl Serializable for i8 {
    fn read_from<R: io::Read>(buf: &mut R) -> Result<i8, Error> {
        Result::Ok(try!(buf.read_i8()))
    }
    fn write_to<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        try!(buf.write_i8(*self));
        Result::Ok(())
    }
}

impl Serializable for i32 {
    fn read_from<R: io::Read>(buf: &mut R) -> Result<i32, Error> {
        Result::Ok(try!(buf.read_i32::<BigEndian>()))
    }
    fn write_to<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        try!(buf.write_i32::<BigEndian>(*self));
        Result::Ok(())
    }
}

impl Serializable for u32 {
    fn read_from<R: io::Read>(buf: &mut R) -> Result<u32, Error> {
        Result::Ok(try!(buf.read_u32::<BigEndian>()))
    }
    fn write_to<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        try!(buf.write_u32::<BigEndian>(*self));
        Result::Ok(())
    }
}

impl Serializable for Vec<u8> {
    fn read_from<R: io::Read>(buf: &mut R) -> Result<Vec<u8>, Error> {
        let mut v = Vec::new();
        try!(buf.read_to_end(&mut v));
        Ok(v)
    }

    fn write_to<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        buf.write_all(&self[..]).map_err(|v| v.into())
    }
}

impl Serializable for bool {
    fn read_from<R: io::Read>(buf: &mut R) -> Result<bool, Error> {
        Result::Ok(try!(buf.read_u8()) != 0)
    }
    fn write_to<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        try!(buf.write_u8(if *self {
            1
        } else {
            0
        }));
        Result::Ok(())
    }
}

impl Serializable for String {
    fn read_from<R: io::Read>(buf: &mut R) -> Result<String, Error> {
        let len = VarInt::read_from(buf)?.0;
        debug_assert!(len >= 0, "Negative string length: {}", len);
        debug_assert!(len <= 65536, "String length too big: {}", len);
        let mut ret = String::new();
        try!(buf.take(len as u64).read_to_string(&mut ret));
        Result::Ok(ret)
    }
    fn write_to<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        let bytes = self.as_bytes();
        try!(VarInt(bytes.len() as i32).write_to(buf));
        try!(buf.write_all(bytes));
        Result::Ok(())
    }
}

pub trait PacketType {
    fn packet_id(&self) -> i32;
    fn write<W: io::Write>(self, buf: &mut W) -> Result<(), Error>;
}

#[macro_export]
macro_rules! create_ids {
    ($t:ty, ) => ();
    ($t:ty, prev($prev:ident), $name:ident) => (
        #[allow(non_upper_case_globals)]
        pub const $name: $t = $prev + 1;
    );
    ($t:ty, prev($prev:ident), $name:ident, $($n:ident),+) => (
        #[allow(non_upper_case_globals)]
        pub const $name: $t = $prev + 1;
        create_ids!($t, prev($name), $($n),+);
    );
    ($t:ty, $name:ident, $($n:ident),+) => (
        #[allow(non_upper_case_globals)]
        pub const $name: $t = 0;
        create_ids!($t, prev($name), $($n),+);
    );
    ($t:ty, $name:ident) => (
        #[allow(non_upper_case_globals)]
        pub const $name: $t = 0;
    );
}

#[macro_export]
macro_rules! create_packets {
        ($(
            $(#[$attr:meta])*
            packet $name:ident {
                $($(#[$fattr:meta])*field $field:ident: $field_type:ty = $(when ($cond:expr))*, )+
            }
        )*)
        => {

        #[derive(Debug)]
        pub enum Packet {
                $(
                    $name($name),
                )*
        }

        pub mod internal_ids {
            create_ids!(i32, $($name),*);
        }

        $(
            #[derive(Default, Debug)]
            $(#[$attr])* pub struct $name {
                $($(#[$fattr])* pub $field: $field_type),+,
            }

            impl PacketType for $name {

                fn packet_id(&self) -> i32 { internal_ids::$name }

                fn write<W: io::Write>(self, buf: &mut W) -> Result<(), Error> {
                    $(
                        if true $(&& ($cond(&self)))* {
                            try!(self.$field.write_to(buf));
                        }
                    )+

                    Result::Ok(())
                }
            }
        )*

        pub fn packet_by_id<R: io::Read>(id: i32, mut buf: &mut R) -> Result<Option<Packet>, Error> {
            match id {
            $(
                self::internal_ids::$name => {
                    use self::$name;
                    let mut packet : $name = $name::default();
                    $(
                        if true $(&& ($cond(&packet)))* {
                            packet.$field = try!(Serializable::read_from(&mut buf));
                        }
                    )+
                    Result::Ok(Option::Some(Packet::$name(packet)))
                },
            )*
                _ => Result::Ok(Option::None)
            }
        }

    }
}

create_packets!(
    packet LoadMusic {
        field filename: String =,
    }
    packet PlayMusic {
        field filename: String =,
    }
    packet StopMusic {
        field filename: String =,
    }
    packet PauseMusic {
        field filename: String =,
    }
    packet ResumeMusic {
        field filename: String =,
    }
    packet RewindMusic {
        field filename: String =,
    }
    packet LoadSound {
        field filename: String =,
    }
    packet PlaySound {
        field filename: String =,
    }
);


pub trait Lengthable : Serializable + Copy + Default {
    fn into(self) -> usize;
    fn from(usize) -> Self;
}

#[derive(Clone, Copy)]
pub struct VarInt(pub i32);

impl Lengthable for VarInt {
    fn into(self) -> usize {
        self.0 as usize
    }

    fn from(u: usize) -> VarInt {
        VarInt(u as i32)
    }
}

impl Serializable for VarInt {
    /// Decodes a `VarInt` from the Reader
    fn read_from<R: io::Read>(buf: &mut R) -> Result<VarInt, Error> {
        const PART : u32 = 0x7F;
        let mut size = 0;
        let mut val = 0u32;
        loop {
            let b = try!(buf.read_u8()) as u32;
            val |= (b & PART) << (size * 7);
            size += 1;
            if size > 5 {
                return Result::Err(Error::new(ErrorKind::Other, "VarInt too big"));
            }
            if (b & 0x80) == 0 {
                break
            }
        }

        Result::Ok(VarInt(val as i32))
    }

    /// Encodes a `VarInt` into the Writer
    fn write_to<W: io::Write>(&self, buf: &mut W) -> Result<(), Error> {
        const PART : u32 = 0x7F;
        let mut val = self.0 as u32;
        loop {
            if (val & !PART) == 0 {
                try!(buf.write_u8(val as u8));
                return Result::Ok(());
            }
            try!(buf.write_u8(((val & PART) | 0x80) as u8));
            val >>= 7;
        }
    }
}


impl default::Default for VarInt {
    fn default() -> VarInt {
        VarInt(0)
    }
}

impl fmt::Debug for VarInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn write_packet<T: PacketType>(packet: T) -> Result<(Vec<u8>), Error> {
    let mut buf = Vec::new();

    // println!("{:?}", packet.packet_id());

    try!(VarInt(packet.packet_id()).write_to(&mut buf));
    try!(packet.write(&mut buf));

    // println!("{:?}", buf);

    Result::Ok((buf))
}

pub fn read_packet(buf: Vec<u8>) -> Result<Packet, Error> {
    // let len = try!(VarInt::read_from(buf)).0 as usize;
    // let mut ibuf = vec![0; len];

    // try!(buf.read_exact(&mut ibuf));
    let mut buf_cur = io::Cursor::new(buf);

    let id = try!(VarInt::read_from(&mut buf_cur)).0;
    let packet = try!(packet_by_id(id, &mut buf_cur));

    //   println!("READ_PACKET {:?}", packet);

    match packet {
        Some(val) => {
            Result::Ok(val)
        }
        None => Result::Err(Error::new(ErrorKind::Other, "missing packet")),
    }
}
/*
pub fn read_packet() -> Result<Packet, Error> {

}
*/
