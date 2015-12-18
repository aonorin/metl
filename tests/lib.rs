extern crate mtl;
extern crate cocoa;

use mtl::{Device, DeviceError};
use cocoa::base::nil;
use cocoa::foundation::NSString;

#[test]
fn get_device() {
    let device = Device::system_default_device();
    assert!(device.is_ok());
}

#[test]
fn get_device_name() {
    let device = Device::system_default_device().unwrap();
    assert!(!device.get_name().into_owned().is_empty());
}

#[test]
fn set_get_command_queue_label() {
    let mut device = Device::system_default_device().unwrap();
    let mut command_queue = device.new_command_queue().unwrap();

    let dummy_command_queue_name = "foo";
    command_queue.set_label(dummy_command_queue_name);

    assert_eq!(command_queue.get_label().into_owned(),
               dummy_command_queue_name.to_string());
}

#[test]
fn insert_debug_capture_boundary_on_command_queue() {
    let mut device = Device::system_default_device().unwrap();
    let mut command_queue = device.new_command_queue().unwrap();
    command_queue.insert_debug_capture_boundary();

    // if nothing panics/fails here, everything is okay
}

#[test]
fn device_from_to_raw() {
    let device = Device::system_default_device().unwrap();
    let raw = unsafe { device.into_raw() };
    let device = Device::from_raw(raw);
    assert!(device.is_ok());
}

#[test]
fn device_from_wrong_type() {
    let some_string = unsafe { NSString::alloc(nil).init_str("Hello") };
    let device_result = Device::from_raw(some_string);
    match device_result {
        Result::Err(DeviceError::ConstructedFromWrongPointerType) => {
            // pass
        }
        _ => {
            assert!(false)
        }
    }
}

#[test]
fn device_from_nullptr() {
    let device_result = Device::from_raw(nil);
    match device_result {
        Result::Err(DeviceError::ConstructedFromNil) => {
            // pass
        }
        _ => {
            assert!(false)
        }
    }
}
