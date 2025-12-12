# LrnRTOS - Platform Abstraction Layer

## Description

All machine use devices, but not all machine define devices the same ways. On more modern and complexe, or bigger machine, there the FDT [1], So the kernel use the FDT to devices informations.
But on smaller machines, or in embedded, there's no FDT. So it's the user who's using the kernel who has the responsability to define the devices needed for the bare minimum working of the kernel,
or the devices he want to use. Because we want the kernel to be able to know if there's a FDT or not, and the drivers to be able to init themselves without knowing if there's a FDT or not, 
we define an abstraction layer. This abstraction layer will check if there's a FDT or not and handle a static: `PLATFORM_INFO`, a static boolean used to either using the FDT or a static devices definitions.
Depending on the static `PLATFORM_INFO`, the abstraction layer will either use the FDT to retrieve information about a specified device, or use the static array `DEVICES`, defined in `src/devices_info.rs`, where all static devices is defined.

## Properties

## References

[1] FDT documentation: `Documentation/kernen/devicetree.md`
