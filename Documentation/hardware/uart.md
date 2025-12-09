# UART - Universal Asynchronous Receiver Transmitter

## Description

The UART is a serial protocol used to transfer data asynchronously between two machines. The communication can be:

- simplex: Data is send in one direction.
- half-duplex: Both side send data but one at a time.
- full-duplex: Both side can send data simultaneously.

UART datas are send as frames.

## Duration and synchronization of UART protocols

UART is asynchronous, receiver and transmitter doesn't share a commun clock. It simplify the protocol but leads to make the transmitter and receiver to use the same transfer speed. 
Today's most used baud rate is 800, 9600, 19,2K, 57,6K et 115,2K.
In addition to having the same baud rate, both transmitter and receiver need to use the same frame structure.

## Kernel usage

For now, UART is used for logs and debugging. When developping, we use UART to send data from qemu to host machine. On production, it can be used for debugging purpose.
