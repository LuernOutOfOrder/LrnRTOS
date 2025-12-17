# LrnRTOS - Platform Abstraction Layer

## Description

All machine use devices, but not all machine define devices the same ways. On more modern and complexe, or bigger machine, there the FDT [1], So the kernel use the FDT to devices informations.
But on smaller machines, or in embedded, there's no FDT. So it's the user who's using the kernel who has the responsability to define the devices needed for the bare minimum working of the kernel,
or the devices he want to use. Because we want the kernel to be able to know if there's a FDT or not, and the drivers to be able to init themselves without knowing if there's a FDT or not, 
we define an abstraction layer. This abstraction layer will check if there's a FDT or not and handle a static: `PLATFORM_INFO`, a static used to either using the FDT or a static devices definitions.
Depending on the static `PLATFORM_INFO`, the abstraction layer will either use the FDT to retrieve information about a specified device, or use the static array `DEVICES`, defined in `src/devices_info.rs`, where all static devices is defined.
The platform layer sits between early boot and driver initialization, and acts as a unified device-discovery backend.
This design avoids driver-side branching and enforces a single discovery contract.

## Booting process flow

The platform layer must be initialized before initializing drivers, because if not, the FDT would not be parsed, and the platform layer would not know if there's a FDT or not, so the drivers coulnd't be initialized.
Before initializing drivers, the function `platform_init()` is called. Pass the FDT address to it, the function will lookup to see if there's a FDT present or not. If there's it will update the
static `PLATFORM_INFO` and set the mode flag to `true`. If there's no FDT, the static will not be change, as it is initialized on `0`.
After the boot process, the `PLATFORM_INFO` is fixed, it would not be changed, no drivers can be init before that. 

### Platform info flags

The `PLATFORM_INFO` static is a u8, it is used as a set of different flags, describing the platform behavior or changing it.
Here's a list of all the different flag used:

- mode (bit 0): define the platform device info mode, 0 = static, 1 = fdt.


## Getting devices information

When drivers initialized themselves, they used a compatible string, it's the only way to initialize a driver.
The platform layer guarantees that the driver can safely get the needed information about the device needed, without the need for the driver to know where the device info come from.
The driver can trust the platform layer to be correctly initialized to be able to initialize all drivers correctly.

The platform layer will get devices informations, and create a generic structure for this device, a single structure is used to store, and to get information from a driver.
When a driver get device informations from platform, it get a generic device structure, with a ptr to a specialized structure for this driver. Like a timer driver, will get a generic structure device, with a ptr to a structure implementing the trait `DeviceInfo`.
This specialized structure will be, for a timer driver, a generic timer structure, implementing common timer device properties, so that all timer driver could get the needed device information from this structure.
This ensure that all drivers, use the same generic structure, and use the specialized structure from the ptr in the generic structure depending on the driver nature, to initialized themselves.

## Properties

### Unified discovery model

All device information is exposed through the same function, regardless of whether it originates from a FDT or from the static array `DEVICES`.

### Priority rule

If a FDT is present at boot, it will always be use as the primary source of truth.
If not FDT is present, it will always be the static definition `DEVICES` in `src/devices_info.rs` as the primary source of truth.
Drivers will never know where the device information come from.

### Deterministic behavior

Getting devices information is deterministic: driver always give a "compatible" string when getting device information, and the string must always be the same as in the FDT or the same as defined in the `src/devices_info.rs`. 

### Error model

If a device is not found or the description is malformed, the platform layer returns a well-defined error.
The kernel guarantees that no driver is probed with incomplete or inconsistent device information(this doesn't cover a wrong description of a device, for exemple if a device is described with the wrong MMIO region, the platform layer couldn't return a correct error, and will continue execution).
So if the kernel cannot initialize properly a driver, the kernel will panic.

## Invariants

Once the platform layer is initialized, the kernel assumes that the hardware description (memory layout, interrupt controllers, timers) is accurate and will not change for the lifetime of the system.

If the platform layer use static hardware description; the kernel assume that the descriptions are correct and will leads to errors or UB if the descriptions are incorrect.

## References

[1] FDT documentation: `Documentation/kernen/devicetree.md`
