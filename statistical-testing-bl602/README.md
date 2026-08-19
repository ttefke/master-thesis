Statistical testing app for the BL602's random number generator.
This application is flashed onto the BL602 and continously generates new random numbers.
Then it transmits them via UART (and a UART-USB transceiver) to a PC which confirms the receipt.
A random number is sent repeatedly until acknowledged by the PC, CRC checksums are used to detect bit flips.