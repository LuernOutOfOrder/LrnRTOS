# Devicetree

## Description

See complete Devicetree specification: 'https://www.devicetree.org/specifications'

## Early boot

On the hardware, the Devicetree file is compressed into the fdt(flattened devicetree), we access it by getting a pointer to it from the kernel entry point, exemple:

```rust
// On RISC-V we can get the pointer by passing a params in entry point function
unsafe extern "C" fn _start(hartid: usize, dtb: usize)
// On early boot, the CPU pass the hartid and the dtb pointer in register a0 and a1
```

After that we just pass the ptr to the parsing function.

## Parsing

For the parsing, we parse all the fdt and store all nodes and properties. We use a stack for the parsing, using it when entering a node to keep track of the hierarchy, and static arrays to store nodes and properties to be able to access them outside the parsing.

### Allocation

#### Pool

To store all nodes and properties we use static array with a max size specified in the config file.

```rust
// Static array NODE_POOL, used to store all parsed node
static mut NODE_POOL: [FdtNode; FDT_MAX_STACK];
// Static array PROPERTIES_POOL, used to store all parsed property
static mut PROPERTIES_POOL: [Property; FDT_MAX_PROPS];
```

The pools use structure to define nodes and properties. Because we don't want to store directly the value from nodes or properties, we store ptr to these property in the fdt.

#### Node

Here's the structure for the node:

```rust
pub struct FdtNode {
    pub nameoff: u32,
    pub first_prop_off: u32,
    pub prop_count: u16,
    pub parent_node_index: Option<usize>,
}
```

Properties:

- nameoff: offset to the node name in structure block.
- first_prop_off: offset to the first node's prop in the PROPERTIES_POOL, save only the first property because all node's properties are following each other in the structure block. So we only need the first property and a counter of properties to recover them all.
- prop_count: counter to keep track of all node's property found. Increment each time a new property is found.
- parent_node_index: the index of the parent node in the device tree, index in NODE_POOL. Important for keeping the hierarchy of the device tree. Use an Option<usize> because of the root node that is orphan.

##### Why this structure

In the fdt, a node is define in the structure block, it start with the node name(a string with a \0 to end the string), followed by all properties in the node. The structure keep only the important property of a node:

- the nameoff(ptr to the name in the structure block).
- first property offset in the NODE_POOL, don't need the other because all node's properties followed each other, so we just need the number of properties in the node.
- properties count to know how many property in the node.
- the parent node index in the NODE_POOL, using an Option for the root node.

#### Property

Here's the structure for a node property:

```rust
pub struct Property {
    pub nameoff: usize,
    pub off_value: usize,
    pub value_len: u32,
}
```

Properties:

- nameoff: offset to the property name in the string block.
- off_value: offset to the property value in the structure block.
- value_len: size of the value in the structure block. Used for parsing and getting the correct value size.
