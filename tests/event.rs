extern crate bluetooth_hci as hci;

use hci::event::*;

#[derive(Debug)]
struct VendorEvent;
#[derive(Debug)]
struct VendorError;

impl hci::event::VendorEvent for VendorEvent {
    type Error = VendorError;

    fn new(_buffer: &[u8]) -> Result<Self, hci::event::Error<Self::Error>> {
        Err(hci::event::Error::Vendor(VendorError))
    }
}

type TestEvent = Event<VendorEvent>;

#[test]
fn connection_complete() {
    let buffer = [
        0x03, 11, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x00, 0x00,
    ];
    match TestEvent::new(Packet(&buffer)) {
        Ok(Event::ConnectionComplete(event)) => {
            assert_eq!(event.status, hci::Status::Success);
            assert_eq!(event.conn_handle, hci::ConnectionHandle(0x0201));
            assert_eq!(
                event.bdaddr,
                hci::BdAddr([0x03, 0x04, 0x05, 0x06, 0x07, 0x08])
            );
            assert_eq!(event.link_type, LinkType::Sco);
            assert_eq!(event.encryption_enabled, false);
        }
        other => panic!("Did not get connection complete event: {:?}", other),
    }
}

#[test]
fn connection_complete_failed_bad_status() {
    let buffer = [
        0x03, 11, 0x80, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x00, 0x00,
    ];
    match TestEvent::new(Packet(&buffer)) {
        Err(Error::BadStatus(0x80)) => (),
        other => panic!("Did not get bad status: {:?}", other),
    }
}

#[test]
fn connection_complete_failed_bad_link_type() {
    let buffer = [
        0x03, 11, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x02, 0x00,
    ];
    match TestEvent::new(Packet(&buffer)) {
        Err(Error::BadLinkType(0x02)) => (),
        other => panic!("Did not get bad connection link type: {:?}", other),
    }
}

#[test]
fn connection_complete_failed_encryption_enabled() {
    let buffer = [
        0x03, 11, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x01, 0x02,
    ];
    match TestEvent::new(Packet(&buffer)) {
        Err(Error::BadEncryptionEnabledValue(0x02)) => (),
        other => panic!("Did not get bad connection link type: {:?}", other),
    }
}

#[test]
fn disconnection_complete() {
    let buffer = [0x05, 4, 0, 0x01, 0x02, 0];
    match TestEvent::new(Packet(&buffer)) {
        Ok(Event::DisconnectionComplete(event)) => {
            assert_eq!(event.status, hci::Status::Success);
            assert_eq!(event.conn_handle, hci::ConnectionHandle(0x0201));
            assert_eq!(event.reason, hci::Status::Success);
        }
        other => panic!("Did not get disconnection complete event: {:?}", other),
    }
}

#[test]
fn disconnection_complete_failed_bad_status() {
    let buffer = [0x05, 4, 0x80, 0x01, 0x02, 0];
    match TestEvent::new(Packet(&buffer)) {
        Err(Error::BadStatus(0x80)) => (),
        other => panic!("Did not get bad status: {:?}", other),
    }
}

#[test]
fn disconnection_complete_failed_bad_reason() {
    let buffer = [0x05, 4, 0, 0x01, 0x02, 0x80];
    match TestEvent::new(Packet(&buffer)) {
        Err(Error::BadReason(0x80)) => (),
        other => panic!("Did not get bad reason: {:?}", other),
    }
}

#[test]
fn encryption_change() {
    let buffer = [0x08, 4, 0x00, 0x01, 0x02, 0x00];
    match TestEvent::new(Packet(&buffer)) {
        Ok(Event::EncryptionChange(event)) => {
            assert_eq!(event.status, hci::Status::Success);
            assert_eq!(event.conn_handle, hci::ConnectionHandle(0x0201));
            assert_eq!(event.encryption, Encryption::Off);
        }
        other => panic!("Did not get encryption change event: {:?}", other),
    }
}

#[test]
fn encryption_change_failed_bad_status() {
    let buffer = [0x08, 4, 0x80, 0x01, 0x02, 0x00];
    match TestEvent::new(Packet(&buffer)) {
        Err(Error::BadStatus(0x80)) => (),
        other => panic!("Did not get bad status: {:?}", other),
    }
}

#[test]
fn encryption_change_failed_bad_encryption() {
    let buffer = [0x08, 4, 0x00, 0x01, 0x02, 0x03];
    match TestEvent::new(Packet(&buffer)) {
        Err(Error::BadEncryptionType(0x03)) => (),
        other => panic!("Did not get bad encryption type: {:?}", other),
    }
}

#[test]
fn read_remote_version_complete() {
    let buffer = [0x0C, 8, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
    match TestEvent::new(Packet(&buffer)) {
        Ok(Event::ReadRemoteVersionInformationComplete(event)) => {
            assert_eq!(event.status, hci::Status::Success);
            assert_eq!(event.conn_handle, hci::ConnectionHandle(0x0201));
            assert_eq!(event.version, 0x03);
            assert_eq!(event.mfgr_name, 0x0504);
            assert_eq!(event.subversion, 0x0706);
        }
        other => panic!("Did not get read remote version info event: {:?}", other),
    }
}

#[test]
fn read_remote_version_complete_failed_bad_status() {
    let buffer = [0x0C, 8, 0x80, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
    match TestEvent::new(Packet(&buffer)) {
        Err(Error::BadStatus(0x80)) => (),
        other => panic!("Did not get bad status: {:?}", other),
    }
}

// The Command Complete event has its own set of tests in command_complete.rs

#[test]
fn command_status() {
    let buffer = [0x0F, 4, 0, 8, 0x01, 0x02];
    match TestEvent::new(Packet(&buffer)) {
        Ok(Event::CommandStatus(event)) => {
            assert_eq!(event.num_hci_command_packets, 8);
            assert_eq!(event.status, hci::Status::Success);
            assert_eq!(event.opcode, hci::Opcode(0x0201));
        }
        other => panic!("Did not get command status: {:?}", other),
    }
}

#[test]
fn hardware_error() {
    let buffer = [0x10, 1, 0x12];
    match TestEvent::new(Packet(&buffer)) {
        Ok(Event::HardwareError(event)) => {
            assert_eq!(event.code, 0x12);
        }
        other => panic!("Did not get hardware error: {:?}", other),
    }
}

#[test]
fn number_of_completed_packets() {
    let buffer = [0x13, 9, 2, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
    match TestEvent::new(Packet(&buffer)) {
        Ok(Event::NumberOfCompletedPackets(event)) => {
            let expected_conn_handles =
                [hci::ConnectionHandle(0x0201), hci::ConnectionHandle(0x0605)];
            let expected_num_packets = [0x0403, 0x0807];
            for (actual, (conn_handle, num_packets)) in event.iter().zip(
                expected_conn_handles
                    .iter()
                    .zip(expected_num_packets.iter()),
            ) {
                assert_eq!(actual.conn_handle, *conn_handle);
                assert_eq!(actual.num_completed_packets, *num_packets);
            }
        }
        other => panic!("Did not get number of completed packets: {:?}", other),
    }
}

#[test]
fn data_buffer_overflow() {
    let buffer = [0x1A, 1, 0x00];
    match TestEvent::new(Packet(&buffer)) {
        Ok(Event::DataBufferOverflow(event)) => {
            assert_eq!(event.link_type, LinkType::Sco);
        }
        other => panic!("Did not get data buffer overflow: {:?}", other),
    }
}

#[test]
fn data_buffer_overflow_failed_bad_link_type() {
    let buffer = [0x1A, 1, 0x02];
    match TestEvent::new(Packet(&buffer)) {
        Err(Error::BadLinkType(link_type)) => assert_eq!(link_type, 0x02),
        other => panic!("Did not get bad link type: {:?}", other),
    }
}
