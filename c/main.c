#include "io.h"
extern void mmio_write(long reg, unsigned int val);
void main()
{
    // // select gpio21 as output
    // mmio_write(0x7E000008 + 0x200000,1<<3);
    // // set pull up 
    // mmio_write(0x7E0000E8 + 0x200000,1<<10);
    // while (1){
    //     for(int i = 0; i< 50000; i++) {
    //         asm("nop");
    //     }
    //     // set gpio 21
    //     mmio_write(0x7E00001C + 0x200000,1<<21);
    //     for(int i = 0; i< 50000; i++) {
    //         asm("nop");
    //     }
    //     // clear gpio 21
    //     mmio_write(0x7E000028 + 0x200000,1<<21);        
    // }

    uart_init();
    uart_writeText("Hello world!\n");
    while (1);
}