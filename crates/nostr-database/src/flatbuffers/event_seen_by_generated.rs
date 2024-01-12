// automatically generated by the FlatBuffers compiler, do not modify

// @generated

use core::cmp::Ordering;
use core::mem;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[allow(unused_imports, dead_code)]
pub mod event_seen_by_fbs {

    use core::cmp::Ordering;
    use core::mem;

    extern crate flatbuffers;
    use self::flatbuffers::{EndianScalar, Follow};

    pub enum EventSeenByOffset {}
    #[derive(Copy, Clone, PartialEq)]

    pub struct EventSeenBy<'a> {
        pub _tab: flatbuffers::Table<'a>,
    }

    impl<'a> flatbuffers::Follow<'a> for EventSeenBy<'a> {
        type Inner = EventSeenBy<'a>;

        #[inline]
        unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
            Self {
                _tab: flatbuffers::Table::new(buf, loc),
            }
        }
    }

    impl<'a> EventSeenBy<'a> {
        pub const VT_RELAY_URLS: flatbuffers::VOffsetT = 4;

        #[inline]
        pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
            EventSeenBy { _tab: table }
        }

        #[allow(unused_mut)]
        pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
            _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
            args: &'args EventSeenByArgs<'args>,
        ) -> flatbuffers::WIPOffset<EventSeenBy<'bldr>> {
            let mut builder = EventSeenByBuilder::new(_fbb);
            if let Some(x) = args.relay_urls {
                builder.add_relay_urls(x);
            }
            builder.finish()
        }

        #[inline]
        pub fn relay_urls(
            &self,
        ) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<&'a str>>> {
            // Safety:
            // Created from valid Table for this object
            // which contains a valid value in this slot
            unsafe {
                self._tab.get::<flatbuffers::ForwardsUOffset<
                    flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<&'a str>>,
                >>(EventSeenBy::VT_RELAY_URLS, None)
            }
        }
    }

    impl flatbuffers::Verifiable for EventSeenBy<'_> {
        #[inline]
        fn run_verifier(
            v: &mut flatbuffers::Verifier,
            pos: usize,
        ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
            use self::flatbuffers::Verifiable;
            v.visit_table(pos)?
                .visit_field::<flatbuffers::ForwardsUOffset<
                    flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<&'_ str>>,
                >>("relay_urls", Self::VT_RELAY_URLS, false)?
                .finish();
            Ok(())
        }
    }
    pub struct EventSeenByArgs<'a> {
        pub relay_urls: Option<
            flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<&'a str>>>,
        >,
    }
    impl<'a> Default for EventSeenByArgs<'a> {
        #[inline]
        fn default() -> Self {
            EventSeenByArgs { relay_urls: None }
        }
    }

    pub struct EventSeenByBuilder<'a: 'b, 'b> {
        fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
        start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
    }
    impl<'a: 'b, 'b> EventSeenByBuilder<'a, 'b> {
        #[inline]
        pub fn add_relay_urls(
            &mut self,
            relay_urls: flatbuffers::WIPOffset<
                flatbuffers::Vector<'b, flatbuffers::ForwardsUOffset<&'b str>>,
            >,
        ) {
            self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
                EventSeenBy::VT_RELAY_URLS,
                relay_urls,
            );
        }

        #[inline]
        pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> EventSeenByBuilder<'a, 'b> {
            let start = _fbb.start_table();
            EventSeenByBuilder {
                fbb_: _fbb,
                start_: start,
            }
        }

        #[inline]
        pub fn finish(self) -> flatbuffers::WIPOffset<EventSeenBy<'a>> {
            let o = self.fbb_.end_table(self.start_);
            flatbuffers::WIPOffset::new(o.value())
        }
    }

    impl core::fmt::Debug for EventSeenBy<'_> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            let mut ds = f.debug_struct("EventSeenBy");
            ds.field("relay_urls", &self.relay_urls());
            ds.finish()
        }
    }
    #[inline]
    /// Verifies that a buffer of bytes contains a `EventSeenBy`
    /// and returns it.
    /// Note that verification is still experimental and may not
    /// catch every error, or be maximally performant. For the
    /// previous, unchecked, behavior use
    /// `root_as_event_seen_by_unchecked`.
    pub fn root_as_event_seen_by(
        buf: &[u8],
    ) -> Result<EventSeenBy, flatbuffers::InvalidFlatbuffer> {
        flatbuffers::root::<EventSeenBy>(buf)
    }
    #[inline]
    /// Verifies that a buffer of bytes contains a size prefixed
    /// `EventSeenBy` and returns it.
    /// Note that verification is still experimental and may not
    /// catch every error, or be maximally performant. For the
    /// previous, unchecked, behavior use
    /// `size_prefixed_root_as_event_seen_by_unchecked`.
    pub fn size_prefixed_root_as_event_seen_by(
        buf: &[u8],
    ) -> Result<EventSeenBy, flatbuffers::InvalidFlatbuffer> {
        flatbuffers::size_prefixed_root::<EventSeenBy>(buf)
    }
    #[inline]
    /// Verifies, with the given options, that a buffer of bytes
    /// contains a `EventSeenBy` and returns it.
    /// Note that verification is still experimental and may not
    /// catch every error, or be maximally performant. For the
    /// previous, unchecked, behavior use
    /// `root_as_event_seen_by_unchecked`.
    pub fn root_as_event_seen_by_with_opts<'b, 'o>(
        opts: &'o flatbuffers::VerifierOptions,
        buf: &'b [u8],
    ) -> Result<EventSeenBy<'b>, flatbuffers::InvalidFlatbuffer> {
        flatbuffers::root_with_opts::<EventSeenBy<'b>>(opts, buf)
    }
    #[inline]
    /// Verifies, with the given verifier options, that a buffer of
    /// bytes contains a size prefixed `EventSeenBy` and returns
    /// it. Note that verification is still experimental and may not
    /// catch every error, or be maximally performant. For the
    /// previous, unchecked, behavior use
    /// `root_as_event_seen_by_unchecked`.
    pub fn size_prefixed_root_as_event_seen_by_with_opts<'b, 'o>(
        opts: &'o flatbuffers::VerifierOptions,
        buf: &'b [u8],
    ) -> Result<EventSeenBy<'b>, flatbuffers::InvalidFlatbuffer> {
        flatbuffers::size_prefixed_root_with_opts::<EventSeenBy<'b>>(opts, buf)
    }
    #[inline]
    /// Assumes, without verification, that a buffer of bytes contains a EventSeenBy and returns it.
    /// # Safety
    /// Callers must trust the given bytes do indeed contain a valid `EventSeenBy`.
    pub unsafe fn root_as_event_seen_by_unchecked(buf: &[u8]) -> EventSeenBy {
        flatbuffers::root_unchecked::<EventSeenBy>(buf)
    }
    #[inline]
    /// Assumes, without verification, that a buffer of bytes contains a size prefixed EventSeenBy and returns it.
    /// # Safety
    /// Callers must trust the given bytes do indeed contain a valid size prefixed `EventSeenBy`.
    pub unsafe fn size_prefixed_root_as_event_seen_by_unchecked(buf: &[u8]) -> EventSeenBy {
        flatbuffers::size_prefixed_root_unchecked::<EventSeenBy>(buf)
    }
    #[inline]
    pub fn finish_event_seen_by_buffer<'a, 'b>(
        fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
        root: flatbuffers::WIPOffset<EventSeenBy<'a>>,
    ) {
        fbb.finish(root, None);
    }

    #[inline]
    pub fn finish_size_prefixed_event_seen_by_buffer<'a, 'b>(
        fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
        root: flatbuffers::WIPOffset<EventSeenBy<'a>>,
    ) {
        fbb.finish_size_prefixed(root, None);
    }
} // pub mod EventSeenByFbs
