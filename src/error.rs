use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    /// Wrong marshal size.
    #[error("Wrong marshal size")]
    WrongMarshalSize,
    /// Packet lost exceeds maximum amount of packets
    /// that can possibly be lost.
    #[error("Invalid total lost count")]
    InvalidTotalLost,
    /// Packet contains an invalid header.
    #[error("Invalid header")]
    InvalidHeader,
    /// Packet contains empty compound.
    #[error("Empty compound packet")]
    EmptyCompound,
    /// Invalid first packet in compound packets. First packet
    /// should either be a SenderReport packet or ReceiverReport
    #[error("First packet in compound must be SR or RR")]
    BadFirstPacket,
    /// CNAME was not defined.
    #[error("Compound missing SourceDescription with CNAME")]
    MissingCname,
    /// Packet was defined before CNAME.
    #[error("Feedback packet seen before CNAME")]
    PacketBeforeCname,
    /// Too many reports.
    #[error("Too many reports")]
    TooManyReports,
    /// Too many chunks.
    #[error("Too many chunks")]
    TooManyChunks,
    /// Too many sources.
    #[error("too many sources")]
    TooManySources,
    /// Packet received is too short.
    #[error("Packet status chunk must be 2 bytes")]
    PacketTooShort,
    /// Buffer is too short.
    #[error("Buffer too short to be written")]
    BufferTooShort,
    /// Wrong packet type.
    #[error("Wrong packet type")]
    WrongType,
    /// SDES received is too long.
    #[error("SDES must be < 255 octets long")]
    SdesTextTooLong,
    /// SDES type is missing.
    #[error("SDES item missing type")]
    SdesMissingType,
    /// Reason is too long.
    #[error("Reason must be < 255 octets long")]
    ReasonTooLong,
    /// Invalid packet version.
    #[error("Invalid packet version")]
    BadVersion,
    /// Invalid padding value.
    #[error("Invalid padding value")]
    WrongPadding,
    /// Wrong feedback message type.
    #[error("Wrong feedback message type")]
    WrongFeedbackType,
    /// Wrong payload type.
    #[error("Wrong payload type")]
    WrongPayloadType,
    /// Header length is too small.
    #[error("Header length is too small")]
    HeaderTooSmall,
    /// Media ssrc was defined as zero.
    #[error("Media SSRC must be 0")]
    SsrcMustBeZero,
    /// Missing REMB identifier.
    #[error("Missing REMB identifier")]
    MissingRembIdentifier,
    /// SSRC number and length mismatches.
    #[error("SSRC num and length do not match")]
    SsrcNumAndLengthMismatch,
    /// Invalid size or start index.
    #[error("Invalid size or startIndex")]
    InvalidSizeOrStartIndex,
    /// Delta exceeds limit.
    #[error("Delta exceed limit")]
    DeltaExceedLimit,
    /// Packet status chunk is not 2 bytes.
    #[error("Packet status chunk must be 2 bytes")]
    PacketStatusChunkLength,

    #[allow(non_camel_case_types)]
    #[error("{0}")]
    new(String),
}

impl Error {
    pub fn equal(&self, err: &anyhow::Error) -> bool {
        err.downcast_ref::<Self>().map_or(false, |e| e == self)
    }
}
