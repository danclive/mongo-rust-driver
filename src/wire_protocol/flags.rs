//! Operation flags.
bitflags! {
    /// Represents the bit vector of options for an OP_REPLY message.
    pub struct OpReplyFlags: i32 {
        const CURSOR_NOT_FOUND  = 0b0000_0001;
        const QUERY_FAILURE     = 0b0000_0010;
        const AWAIT_CAPABLE     = 0b0000_1000;
    }
}

bitflags! {
    /// Represents the bit vector of flags for an OP_QUERY message.
    pub struct OpQueryFlags: i32 {
        const TAILABLE_CURSOR   = 0b0000_0010;
        const SLAVE_OK          = 0b0000_0100;
        const OPLOG_RELAY       = 0b0000_1000;
        const NO_CURSOR_TIMEOUT = 0b0001_0000;
        const AWAIT_DATA        = 0b0010_0000;
        const EXHAUST           = 0b0100_0000;
        const PARTIAL           = 0b1000_0000;
    }
}
