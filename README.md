# Bluetooth HCI Async

forked from [bluetooth_hci](https://github.com/danielgallagher0/bluetooth-hci)

[![Build Status](https://github.com/OueslatiGhaith/bluetooth-hci/actions/workflows/ci.yml/badge.svg)](https://github.com/OueslatiGhaith/bluetooth-hci/actions/workflows/ci.yml/badge.svg)

This crate defines a pure Rust implementation of the Bluetooth
Host-Controller Interface for the STM32WB family of microcontrollers. It defines commands
and events from the specification, and vendor-specific commands and events.

This crate requires the `#![feature(async_fn_in_trait)]` feature

## Version

This crate can support versions 4.1, 4.2, and 5.0 of the Bluetooth
specification. By default, it supports version 4.1. To enable another
version, add the following to your `Cargo.toml`:

    [dependencies.bluetooth-hci]
    features = "version-4-2"

or

    [dependencies.bluetooth-hci]
    features = "version-5-0"

## Implementation

This crate defines a trait (`Controller`) that should be implemented
for a specific BLE chip. Any implementor can then be used as a
`host::uart::UartHci` to read and write to the chip.

    impl stm32wb_hci::Controller for MyController {
        type Error = BusError;
        type Header = stm32wb_hci::host::uart::CommandHeader;
        async fn controller_write(&mut self, header: &[u8], payload: &[u8]) -> Result<(), Self::Error> {
            // implementation...
        }
        async fn controller_read_into(&mut self, buffer: &mut [u8]) -> Result<(), Self::Error> {
            // implementation...
        }
    }

The entire Bluetooth HCI is implemented in terms of these functions
that handle the low-level I/O. To read events, you can use the
`host::uart::UartHci` trait, which defines a `read` function. The easiest
way to specify the vendor-specific event type is via type inference:

    fn process_event(e: hci::event::Event<MyVendorEvent>) {
        // do stuff with e
    }
    // elsewhere...
    process_event(controller.read()?)

## Supported Commands and Events

This crate contains only partial support for commands and events right
now. The only commands and events (as of September 2018) are those
used by the [BlueNRG](https://github.com/danielgallagher0/bluenrg)
chip. Support for HCI ACL Data Packets and HCI Synchronous Data
Packets still needs to be determined.

See the [Bluetooth
Specification](https://www.bluetooth.org/DocMan/handlers/DownloadDoc.ashx?doc_id=421043)
for more (many, many more) details on what this crate should
eventually support. Volume 2, Part E, section 7 is the most relevant
portion for this crate.
