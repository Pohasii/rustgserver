// automatically generated by the FlatBuffers compiler, do not modify
extern crate flatbuffers;
use std::mem;
use std::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
pub enum MessageOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Message<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Message<'a> {
  type Inner = Message<'a>;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table { buf, loc } }
  }
}

impl<'a> Message<'a> {
  pub const VT_PLAYERS: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Message { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args MessageArgs<'args>
  ) -> flatbuffers::WIPOffset<Message<'bldr>> {
    let mut builder = MessageBuilder::new(_fbb);
    if let Some(x) = args.players { builder.add_players(x); }
    builder.finish()
  }


  #[inline]
  pub fn players(&self) -> Option<&'a [Game_Object]> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, Game_Object>>>(Message::VT_PLAYERS, None).map(|v| v.safe_slice())
  }
}

impl flatbuffers::Verifiable for Message<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, Game_Object>>>("players", Self::VT_PLAYERS, false)?
     .finish();
    Ok(())
  }
}
pub struct MessageArgs<'a> {
    pub players: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, Game_Object>>>,
}
impl<'a> Default for MessageArgs<'a> {
  #[inline]
  fn default() -> Self {
    MessageArgs {
      players: None,
    }
  }
}
pub struct MessageBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> MessageBuilder<'a, 'b> {
  #[inline]
  pub fn add_players(&mut self, players: flatbuffers::WIPOffset<flatbuffers::Vector<'b , Game_Object>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Message::VT_PLAYERS, players);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> MessageBuilder<'a, 'b> {
    let start = _fbb.start_table();
    MessageBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Message<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for Message<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("Message");
      ds.field("players", &self.players());
      ds.finish()
  }
}
#[inline]
#[deprecated(since="2.0.0", note="Deprecated in favor of `root_as...` methods.")]
pub fn get_root_as_message<'a>(buf: &'a [u8]) -> Message<'a> {
  unsafe { flatbuffers::root_unchecked::<Message<'a>>(buf) }
}

#[inline]
#[deprecated(since="2.0.0", note="Deprecated in favor of `root_as...` methods.")]
pub fn get_size_prefixed_root_as_message<'a>(buf: &'a [u8]) -> Message<'a> {
  unsafe { flatbuffers::size_prefixed_root_unchecked::<Message<'a>>(buf) }
}

#[inline]
/// Verifies that a buffer of bytes contains a `Message`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_message_unchecked`.
pub fn root_as_message(buf: &[u8]) -> Result<Message, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root::<Message>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `Message` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_message_unchecked`.
pub fn size_prefixed_root_as_message(buf: &[u8]) -> Result<Message, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root::<Message>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `Message` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_message_unchecked`.
pub fn root_as_message_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<Message<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root_with_opts::<Message<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `Message` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_message_unchecked`.
pub fn size_prefixed_root_as_message_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<Message<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root_with_opts::<Message<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a Message and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `Message`.
pub unsafe fn root_as_message_unchecked(buf: &[u8]) -> Message {
  flatbuffers::root_unchecked::<Message>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed Message and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `Message`.
pub unsafe fn size_prefixed_root_as_message_unchecked(buf: &[u8]) -> Message {
  flatbuffers::size_prefixed_root_unchecked::<Message>(buf)
}
#[inline]
pub fn finish_message_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<Message<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_message_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<Message<'a>>) {
  fbb.finish_size_prefixed(root, None);
}
