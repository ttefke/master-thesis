The code I developed as part of my master's thesis.
To build the executables, clone the code first:

```
git clone --recurse-submodules git@github.com:ttefke/master-thesis.git
```

Then, run the `build.sh` script. Please note this step requires [docker](https://www.docker.com/).
You will find the executables in the `out` folder.
To flash files on the BL602, you can use [blflash](https://github.com/spacemeowx2/blflash).

The script builds the following executables:
- blinky: Blinks the BL602 LED (demo app)
- button: Tests the BL602 internal GPIO pin button (demo app)
- random: Generates a random number using the BL602's RNG and prints it (demo app)
- scdrand: Smard Card Daemon Tools, adapted to extract random data from the Nitrokey 2 Pro
- statistical-testing-bl602: Continously generates random numbers using the BL602's RNG and sends it using UART
- statistical-testing-desktop: Can either listen to a random number stream generated using the `statistical-testing-bl602` tool and store received numbers (`capture`) or perform the NIST's battery of tests on a previously recorded stream of RNG data (`evaluate`).