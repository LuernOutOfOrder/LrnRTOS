# LrnRTOS - Platform Abstraction Layer

## Description

All machine use devices, but not all machine define devices the same ways. On more modern and complexe, or bigger machine, there the FDT [1], So the kernel use the FDT to devices informations.
But on smaller machines, or in embedded, there's no FDT. So it's the user who's using the kernel who has the responsability to define the devices needed for the bare minimum working of the kernel,
or the devices he want to use. Because we want the kernel to be able to know if there's a FDT or not, and the drivers to be able to init themselves without knowing if there's a FDT or not, 
we define an abstraction layer. This abstraction layer will check if there's a FDT or not and handle a static: `PLATFORM_INFO`, a static boolean used to either using the FDT or a static devices definitions.
Depending on the static `PLATFORM_INFO`, the abstraction layer will either use the FDT to retrieve information about a specified device, or use the static array `DEVICES`, defined in `src/devices_info.rs`, where all static devices is defined.

## Booting process flow

Before initializing drivers, the function `platform_init()` is called. Pass the FDT address to it, the function will lookup to see if there's a FDT present or not. If there's it will update the
static `PLATFORM_INFO` and set it to `true`. If there's no FDT, the static will not be change, as it is initialized on `false`. 

## Getting devices information

When drivers initialize themselves, they call a function name: `devices_get_info()`, see more informations about 

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
The kernel guarantees that no driver is probed with incomplete or inconsistent device information.
So if the kernel cannot initialize properly a driver, the kernel will panic.



## References

[1] FDT documentation: `Documentation/kernen/devicetree.md`
