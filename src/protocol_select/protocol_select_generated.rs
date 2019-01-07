// automatically generated by the FlatBuffers compiler, do not modify


pub mod p2p {
  #![allow(dead_code)]
  #![allow(unused_imports)]

  use std::mem;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::EndianScalar;
pub mod protocol_select {
  #![allow(dead_code)]
  #![allow(unused_imports)]

  use std::mem;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::EndianScalar;

pub enum ProtocolMessageOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct ProtocolMessage<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for ProtocolMessage<'a> {
    type Inner = ProtocolMessage<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> ProtocolMessage<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        ProtocolMessage {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args ProtocolMessageArgs<'args>) -> flatbuffers::WIPOffset<ProtocolMessage<'bldr>> {
      let mut builder = ProtocolMessageBuilder::new(_fbb);
      if let Some(x) = args.support_versions { builder.add_support_versions(x); }
      if let Some(x) = args.name { builder.add_name(x); }
      builder.finish()
    }

    pub const VT_NAME: flatbuffers::VOffsetT = 4;
    pub const VT_SUPPORT_VERSIONS: flatbuffers::VOffsetT = 6;

  #[inline]
  pub fn name(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(ProtocolMessage::VT_NAME, None)
  }
  #[inline]
  pub fn support_versions(&self) -> Option<flatbuffers::Vector<flatbuffers::ForwardsUOffset<&'a str>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<flatbuffers::ForwardsUOffset<&'a str>>>>(ProtocolMessage::VT_SUPPORT_VERSIONS, None)
  }
}

pub struct ProtocolMessageArgs<'a> {
    pub name: Option<flatbuffers::WIPOffset<&'a  str>>,
    pub support_versions: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , flatbuffers::ForwardsUOffset<&'a  str>>>>,
}
impl<'a> Default for ProtocolMessageArgs<'a> {
    #[inline]
    fn default() -> Self {
        ProtocolMessageArgs {
            name: None,
            support_versions: None,
        }
    }
}
pub struct ProtocolMessageBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> ProtocolMessageBuilder<'a, 'b> {
  #[inline]
  pub fn add_name(&mut self, name: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(ProtocolMessage::VT_NAME, name);
  }
  #[inline]
  pub fn add_support_versions(&mut self, support_versions: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<&'b  str>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(ProtocolMessage::VT_SUPPORT_VERSIONS, support_versions);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> ProtocolMessageBuilder<'a, 'b> {
    let start = _fbb.start_table();
    ProtocolMessageBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<ProtocolMessage<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

}  // pub mod ProtocolSelect
}  // pub mod P2P
