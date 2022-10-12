# ewos
RPI4B物尽其用

## Boot from SD card
On the card, generate a file named `config.txt` with the following contents:
```txt
arm_64bit=1
core_freq_min=500
```
### FirmWare, download from given web address and copy to SD card
    - [fixup4.dat](https://github.com/raspberrypi/firmware/raw/master/boot/fixup4.dat)
    - [start4.elf](https://github.com/raspberrypi/firmware/raw/master/boot/start4.elf)
    - [bcm2711-rpi-4-b.dtb](https://github.com/raspberrypi/firmware/raw/master/boot/bcm2711-rpi-4-b.dtb)


### Build target<kernel8.img> and boot RPI4
1. make all
2. copy kernel8.img to SD card
3. open uart console and POWER ON
4. you may see "Hello world!" on the uart console.
