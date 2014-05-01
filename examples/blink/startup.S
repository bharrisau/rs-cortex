/**************************************************************************//**
 * @file     startup_MKL25Z.S
 * @brief    CMSIS Cortex-M0Plus Core Device Startup File for
 *           Device MKL25Z
 * @version  V3.10
 * @date     23. November 2012
 *
 * @note     GCC for ARM Embedded Processors
 *
 ******************************************************************************/
/* Copyright (c) 2012 ARM LIMITED
	 Copyright (c) 2013 Ben Harris

	 All rights reserved.
	 Redistribution and use in source and binary forms, with or without
	 modification, are permitted provided that the following conditions are met:
	 - Redistributions of source code must retain the above copyright
		 notice, this list of conditions and the following disclaimer.
	 - Redistributions in binary form must reproduce the above copyright
		 notice, this list of conditions and the following disclaimer in the
		 documentation and/or other materials provided with the distribution.
	 - Neither the name of ARM nor the names of its contributors may be used
		 to endorse or promote products derived from this software without
		 specific prior written permission.
	 *
	 THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
	 AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
	 IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
	 ARE DISCLAIMED. IN NO EVENT SHALL COPYRIGHT HOLDERS AND CONTRIBUTORS BE
	 LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
	 CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
	 SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
	 INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
	 CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
	 ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
	 POSSIBILITY OF SUCH DAMAGE.
	 ---------------------------------------------------------------------------*/


		.syntax unified
		.arch armv6-m

		.section .stack
		.align 3
#ifdef __STACK_SIZE
		.equ    Stack_Size, __STACK_SIZE
#else
		.equ    Stack_Size, 0x00000400
#endif
		.globl    __StackTop
		.globl    __StackLimit
__StackLimit:
		.space    Stack_Size
		.size __StackLimit, . - __StackLimit
__StackTop:
		.size __StackTop, . - __StackTop

		.section .heap
		.align 3
#ifdef __HEAP_SIZE
		.equ    Heap_Size, __HEAP_SIZE
#else
		.equ    Heap_Size, 0x00000C00
#endif
		.globl    __HeapBase
		.globl    __HeapLimit
__HeapBase:
		.space    Heap_Size
		.size __HeapBase, . - __HeapBase
__HeapLimit:
		.size __HeapLimit, . - __HeapLimit

		.section .isr_vector
		.align 2
		.globl __isr_vector
__isr_vector:
		.long    __StackTop            /* Top of Stack */
		.long    Reset_Handler         /* Reset Handler */
		.long    NMI_Handler           /* NMI Handler */
		.long    HardFault_Handler     /* Hard Fault Handler */
		.long    0                     /* Reserved */
		.long    0                     /* Reserved */
		.long    0                     /* Reserved */
		.long    0                     /* Reserved */
		.long    0                     /* Reserved */
		.long    0                     /* Reserved */
		.long    0                     /* Reserved */
		.long    SVC_Handler           /* SVCall Handler */
		.long    0                     /* Reserved */
		.long    0                     /* Reserved */
		.long    PendSV_Handler        /* PendSV Handler */
		.long    SysTick_Handler       /* SysTick Handler */

		/* External interrupts */
		.long    DMA0_IRQHandler       /*  DMA channel 0 transfer complete interrupt */
		.long    DMA1_IRQHandler       /*  DMA channel 1 transfer complete interrupt */
		.long    DMA2_IRQHandler       /*  DMA channel 2 transfer complete interrupt */
		.long    DMA3_IRQHandler       /*  DMA channel 3 transfer complete interrupt */
		.long    0                     /* Reserved */
		.long    FTFA_IRQHandler       /*  FTFA interrupt                           */
		.long    LVD_LVW_IRQHandler 	 /*  Low Voltage Detect, Low Voltage Warning  */
		.long    LLW_IRQHandler 			 /*  Low Leakage Wakeup                       */
		.long    I2C0_IRQHandler 			 /*  I2C0 interrupt                           */
		.long    I2C1_IRQHandler 			 /*  I2C1 interrupt                           */
		.long    SPI0_IRQHandler 			 /*  SPI0 interrupt                           */
		.long    SPI1_IRQHandler 			 /*  SPI1 interrupt                           */
		.long    UART0_IRQHandler 		 /*  UART0 status/error interrupt             */
		.long    UART1_IRQHandler 		 /*  UART1 status/error interrupt             */
		.long    UART2_IRQHandler 		 /*  UART2 status/error interrupt             */
		.long    ADC0_IRQHandler 			 /*  ADC0 interrupt                           */
		.long    CMP0_IRQHandler 			 /*  CMP0 interrupt                           */
		.long    TPM0_IRQHandler 			 /*  TPM0 fault, overflow and channels interrupt */
		.long    TPM1_IRQHandler 			 /*  TPM1 fault, overflow and channels interrupt */
		.long    TPM2_IRQHandler 			 /*  TPM2 fault, overflow and channels interrupt */
		.long    RTC_IRQHandler 			 /*  RTC interrupt                            */
		.long    RTC_Seconds_IRQHandler /*  RTC seconds interrupt                    */
		.long    PIT_IRQHandler 			 /*  PIT timer interrupt                      */
		.long    0                     /* Reserved */
		.long    USB0_IRQHandler 			 /*  USB0 interrupt                           */
		.long    DAC0_IRQHandler 			 /*  DAC interrupt                            */
		.long    TSI0_IRQHandler 			 /*  TSI0 interrupt                           */
		.long    MCG_IRQHandler 			 /*  MCG interrupt                            */
		.long    LPTimer_IRQHandler 	 /*  LPTimer interrupt                        */
		.long    0                     /* Reserved */
		.long    PORTA_IRQHandler 		 /*  Port A interrupt                         */
		.long    PORTD_IRQn    				 /* Port D interrupt                          */

		.size    __isr_vector, . - __isr_vector

		.section .kinetis_flash_config
		.align 2
		.globl __kinetis_flash_config
__kinetis_flash_config:
    .long 0xffffffff							/* Backdoor comparison key 0,1,2,3 */
    .long 0xffffffff							/* Backdoor comparison key 4,5,6,7 */
    .byte 0xff										/* FPROT3 */
    .byte 0xff										/* FPROT2 */
    .byte 0xff										/* FPROT1 */
    .byte 0xff										/* FPROT0 */
    .byte 0xfe										/* FSEC   */
    .byte 0xff										/* FOPT   */
    .byte 0xff										/* Reserved */
    .byte 0xff										/* Reserved */

		.section	.text.Reset_Handler
		.thumb
		.thumb_func
		.align 2
		.globl   Reset_Handler
		.type    Reset_Handler, %function
Reset_Handler:
/*     Loop to copy data from read only memory to RAM. The ranges
 *      of copy from/to are specified by following symbols evaluated in
 *      linker script.
 *      __etext: End of code section, i.e., begin of data sections to copy from.
 *      __data_start__/__data_end__: RAM address range that data should be
 *      copied to. Both must be aligned to 4 bytes boundary.  */

		ldr    r1, =__etext
		ldr    r2, =__data_start__
		ldr    r3, =__data_end__

#if 0
/* Here are two copies of loop implemenations. First one favors code size
 * and the second one favors performance. Default uses the first one.
 * Change to "#if 0" to use the second one */
.flash_to_ram_loop:
		cmp     r2, r3
		ittt    lt
		ldrlt   r0, [r1], #4
		strlt   r0, [r2], #4
		blt    .flash_to_ram_loop
#else
		subs    r3, r2
		ble    .flash_to_ram_loop_end
.flash_to_ram_loop:
		subs    r3, #4
		ldr    r0, [r1, r3]
		str    r0, [r2, r3]
		bgt    .flash_to_ram_loop
.flash_to_ram_loop_end:
#endif


		ldr    r0, =SystemInit
		blx    r0
		ldr    r0, =_start
		blx    r0
    bkpt
		.pool
		.size Reset_Handler, . - Reset_Handler

/* Exception Handlers */

		.text
		.weak   NMI_Handler
		.type   NMI_Handler, %function
NMI_Handler:
		B       .
		.size   NMI_Handler, . - NMI_Handler

		.weak   HardFault_Handler
		.type   HardFault_Handler, %function
HardFault_Handler:
		B       .
		.size   HardFault_Handler, . - HardFault_Handler

		.weak   SVC_Handler
		.type   SVC_Handler, %function
SVC_Handler:
		B       .
		.size   SVC_Handler, . - SVC_Handler

		.weak   PendSV_Handler
		.type   PendSV_Handler, %function
PendSV_Handler:
		B       .
		.size   PendSV_Handler, . - PendSV_Handler

		.weak   SysTick_Handler
		.type   SysTick_Handler, %function
SysTick_Handler:
		B       .
		.size   SysTick_Handler, . - SysTick_Handler


/*    Macro to define default handlers. Default handler
 *    will be weak symbol and just dead loops. They can be
 *    overwritten by other handlers */
		.macro    def_irq_handler    handler_name
		.align 1
		.thumb_func
		.weak    \handler_name
		.type    \handler_name, %function
\handler_name :
		b    .
		.size    \handler_name, . - \handler_name


		.endm


/* IRQ Handlers */
		def_irq_handler    DMA0_IRQHandler
		def_irq_handler    DMA1_IRQHandler
		def_irq_handler    DMA2_IRQHandler
		def_irq_handler    DMA3_IRQHandler
		def_irq_handler    FTFA_IRQHandler
		def_irq_handler    LVD_LVW_IRQHandler
		def_irq_handler    LLW_IRQHandler
		def_irq_handler    I2C0_IRQHandler
		def_irq_handler    I2C1_IRQHandler
		def_irq_handler    SPI0_IRQHandler
		def_irq_handler    SPI1_IRQHandler
		def_irq_handler    UART0_IRQHandler
		def_irq_handler    UART1_IRQHandler
		def_irq_handler    UART2_IRQHandler
		def_irq_handler    ADC0_IRQHandler
		def_irq_handler    CMP0_IRQHandler
		def_irq_handler    TPM0_IRQHandler
		def_irq_handler    TPM1_IRQHandler
		def_irq_handler    TPM2_IRQHandler
		def_irq_handler    RTC_IRQHandler
		def_irq_handler    RTC_Seconds_IRQHandler
		def_irq_handler    PIT_IRQHandler
		def_irq_handler    USB0_IRQHandler
		def_irq_handler    DAC0_IRQHandler
		def_irq_handler    TSI0_IRQHandler
		def_irq_handler    MCG_IRQHandler
		def_irq_handler    LPTimer_IRQHandler
		def_irq_handler    PORTA_IRQHandler
		def_irq_handler    PORTD_IRQn

		.end
