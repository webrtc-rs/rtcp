use super::*;

#[test]
fn test_rapid_resynchronization_request_unmarshal() {
    let tests = vec![
        (
            "valid",
            Bytes::from_static(&[
                0x85, 0xcd, 0x0, 0x2, // RapidResynchronizationRequest
                0x90, 0x2f, 0x9e, 0x2e, // sender=0x902f9e2e
                0x90, 0x2f, 0x9e, 0x2e, // media=0x902f9e2e
            ]),
            RapidResynchronizationRequest {
                sender_ssrc: 0x902f9e2e,
                media_ssrc: 0x902f9e2e,
            },
            None,
        ),
        (
            "short report",
            Bytes::from_static(&[
                0x85, 0xcd, 0x0, 0x2, // ssrc=0x902f9e2e
                0x90, 0x2f, 0x9e, 0x2e,
                // report ends early
            ]),
            RapidResynchronizationRequest::default(),
            Some(Error::PacketTooShort),
        ),
        (
            "wrong type",
            Bytes::from_static(&[
                0x81, 0xc8, 0x0, 0x7, // v=2, p=0, count=1, SR, len=7
                0x90, 0x2f, 0x9e, 0x2e, // ssrc=0x902f9e2e
                0xbc, 0x5e, 0x9a, 0x40, // ssrc=0xbc5e9a40
                0x0, 0x0, 0x0, 0x0, // fracLost=0, totalLost=0
                0x0, 0x0, 0x46, 0xe1, // lastSeq=0x46e1
                0x0, 0x0, 0x1, 0x11, // jitter=273
                0x9, 0xf3, 0x64, 0x32, // lsr=0x9f36432
                0x0, 0x2, 0x4a, 0x79, // delay=150137
            ]),
            RapidResynchronizationRequest::default(),
            Some(Error::WrongType),
        ),
        (
            "nil",
            Bytes::from_static(&[]),
            RapidResynchronizationRequest::default(),
            Some(Error::PacketTooShort),
        ),
    ];

    for (name, data, want, want_error) in tests {
        let got = RapidResynchronizationRequest::unmarshal(&data);

        assert_eq!(
            got.is_err(),
            want_error.is_some(),
            "Unmarshal {} rr: err = {:?}, want {:?}",
            name,
            got,
            want_error
        );

        if let Some(err) = want_error {
            let got_err = got.err().unwrap();
            assert_eq!(
                got_err, err,
                "Unmarshal {} rr: err = {:?}, want {:?}",
                name, got_err, err,
            );
        } else {
            let actual = got.unwrap();
            assert_eq!(
                actual, want,
                "Unmarshal {} rr: got {:?}, want {:?}",
                name, actual, want
            );
        }
    }
}

#[test]
fn test_rapid_resynchronization_request_roundtrip() {
    let tests = vec![(
        "valid",
        RapidResynchronizationRequest {
            sender_ssrc: 0x902f9e2e,
            media_ssrc: 0x902f9e2e,
        },
        None,
    )];

    for (name, want, want_error) in tests {
        let got = want.marshal();

        assert_eq!(
            got.is_ok(),
            want_error.is_none(),
            "Marshal {}: err = {:?}, want {:?}",
            name,
            got,
            want_error
        );

        if let Some(err) = want_error {
            let got_err = got.err().unwrap();
            assert_eq!(
                got_err, err,
                "Unmarshal {} rr: err = {:?}, want {:?}",
                name, got_err, err,
            );
        } else {
            let data = got.ok().unwrap();
            let actual = RapidResynchronizationRequest::unmarshal(&data)
                .expect(format!("Unmarshal {}", name).as_str());

            assert_eq!(
                actual, want,
                "{} round trip: got {:?}, want {:?}",
                name, actual, want
            )
        }
    }
}
