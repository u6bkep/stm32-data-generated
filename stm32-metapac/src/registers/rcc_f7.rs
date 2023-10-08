
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Rcc",
            extends: None,
            description: Some(
                "Reset and clock control",
            ),
            items: &[
                BlockItem {
                    name: "cr",
                    description: Some(
                        "clock control register",
                    ),
                    array: None,
                    byte_offset: 0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pllcfgr",
                    description: Some(
                        "PLL configuration register",
                    ),
                    array: None,
                    byte_offset: 4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pllcfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfgr",
                    description: Some(
                        "clock configuration register",
                    ),
                    array: None,
                    byte_offset: 8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cir",
                    description: Some(
                        "clock interrupt register",
                    ),
                    array: None,
                    byte_offset: 12,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cir",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb1rstr",
                    description: Some(
                        "AHB1 peripheral reset register",
                    ),
                    array: None,
                    byte_offset: 16,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb1rstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb2rstr",
                    description: Some(
                        "AHB2 peripheral reset register",
                    ),
                    array: None,
                    byte_offset: 20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb2rstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb3rstr",
                    description: Some(
                        "AHB3 peripheral reset register",
                    ),
                    array: None,
                    byte_offset: 24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb3rstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1rstr",
                    description: Some(
                        "APB1 peripheral reset register",
                    ),
                    array: None,
                    byte_offset: 32,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1rstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2rstr",
                    description: Some(
                        "APB2 peripheral reset register",
                    ),
                    array: None,
                    byte_offset: 36,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb2rstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb1enr",
                    description: Some(
                        "AHB1 peripheral clock register",
                    ),
                    array: None,
                    byte_offset: 48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb1enr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb2enr",
                    description: Some(
                        "AHB2 peripheral clock enable register",
                    ),
                    array: None,
                    byte_offset: 52,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb2enr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb3enr",
                    description: Some(
                        "AHB3 peripheral clock enable register",
                    ),
                    array: None,
                    byte_offset: 56,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb3enr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1enr",
                    description: Some(
                        "APB1 peripheral clock enable register",
                    ),
                    array: None,
                    byte_offset: 64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1enr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2enr",
                    description: Some(
                        "APB2 peripheral clock enable register",
                    ),
                    array: None,
                    byte_offset: 68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb2enr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb1lpenr",
                    description: Some(
                        "AHB1 peripheral clock enable in low power mode register",
                    ),
                    array: None,
                    byte_offset: 80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb1lpenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb2lpenr",
                    description: Some(
                        "AHB2 peripheral clock enable in low power mode register",
                    ),
                    array: None,
                    byte_offset: 84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb2lpenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb3lpenr",
                    description: Some(
                        "AHB3 peripheral clock enable in low power mode register",
                    ),
                    array: None,
                    byte_offset: 88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb3lpenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1lpenr",
                    description: Some(
                        "APB1 peripheral clock enable in low power mode register",
                    ),
                    array: None,
                    byte_offset: 96,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1lpenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2lpenr",
                    description: Some(
                        "APB2 peripheral clock enabled in low power mode register",
                    ),
                    array: None,
                    byte_offset: 100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb2lpenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bdcr",
                    description: Some(
                        "Backup domain control register",
                    ),
                    array: None,
                    byte_offset: 112,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bdcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "csr",
                    description: Some(
                        "clock control & status register",
                    ),
                    array: None,
                    byte_offset: 116,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Csr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sscgr",
                    description: Some(
                        "spread spectrum clock generation register",
                    ),
                    array: None,
                    byte_offset: 128,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sscgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "plli2scfgr",
                    description: Some(
                        "PLLI2S configuration register",
                    ),
                    array: None,
                    byte_offset: 132,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Plli2scfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pllsaicfgr",
                    description: Some(
                        "PLL configuration register",
                    ),
                    array: None,
                    byte_offset: 136,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pllsaicfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dckcfgr1",
                    description: Some(
                        "dedicated clocks configuration register",
                    ),
                    array: None,
                    byte_offset: 140,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dckcfgr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dckcfgr2",
                    description: Some(
                        "dedicated clocks configuration register",
                    ),
                    array: None,
                    byte_offset: 144,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dckcfgr2",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Ahb2lpenr",
            extends: None,
            description: Some(
                "AHB2 peripheral clock enable in low power mode register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dcmilpen",
                    description: Some(
                        "Camera interface enable during Sleep mode",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jpeglpen",
                    description: Some(
                        "JPEG module enabled during Sleep mode",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aeslpen",
                    description: Some(
                        "AES module clock enable during Sleep mode",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cryplpen",
                    description: Some(
                        "Cryptography modules clock enable during Sleep mode",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hashlpen",
                    description: Some(
                        "Hash modules clock enable during Sleep mode",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rnglpen",
                    description: Some(
                        "Random number generator clock enable during Sleep mode",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usb_otg_fslpen",
                    description: Some(
                        "USB OTG FS clock enable during Sleep mode",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Plli2scfgr",
            extends: None,
            description: Some(
                "PLLI2S configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "plli2sn",
                    description: Some(
                        "PLLI2S multiplication factor for VCO",
                    ),
                    bit_offset: 6,
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "plli2sp",
                    description: Some(
                        "PLLI2S division factor for SPDIFRX clock",
                    ),
                    bit_offset: 16,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Pllisp",
                    ),
                },
                Field {
                    name: "plli2sq",
                    description: Some(
                        "PLLI2S division factor for SAI1 clock",
                    ),
                    bit_offset: 24,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "plli2sr",
                    description: Some(
                        "PLLI2S division factor for I2S clocks",
                    ),
                    bit_offset: 28,
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dckcfgr1",
            extends: None,
            description: Some(
                "dedicated clocks configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "plli2sdivq",
                    description: Some(
                        "PLLI2S division factor for SAI1 clock",
                    ),
                    bit_offset: 0,
                    bit_size: 5,
                    array: None,
                    enumm: Some(
                        "Pllisdivq",
                    ),
                },
                Field {
                    name: "pllsaidivq",
                    description: Some(
                        "PLLSAI division factor for SAI1 clock",
                    ),
                    bit_offset: 8,
                    bit_size: 5,
                    array: None,
                    enumm: Some(
                        "Pllsaidivq",
                    ),
                },
                Field {
                    name: "pllsaidivr",
                    description: Some(
                        "division factor for LCD_CLK",
                    ),
                    bit_offset: 16,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Pllsaidivr",
                    ),
                },
                Field {
                    name: "sai1sel",
                    description: Some(
                        "SAI1 clock source selection",
                    ),
                    bit_offset: 20,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Saisel",
                    ),
                },
                Field {
                    name: "sai2sel",
                    description: Some(
                        "SAI2 clock source selection",
                    ),
                    bit_offset: 22,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Saisel",
                    ),
                },
                Field {
                    name: "timpre",
                    description: Some(
                        "Timers clocks prescalers selection",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Timpre",
                    ),
                },
                Field {
                    name: "dfsdm1sel",
                    description: Some(
                        "DFSDM1 clock source selection",
                    ),
                    bit_offset: 25,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Dfsdmsel",
                    ),
                },
                Field {
                    name: "adfsdm1sel",
                    description: Some(
                        "DFSDM1 AUDIO clock source selection",
                    ),
                    bit_offset: 26,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Adfsdmsel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Apb1rstr",
            extends: None,
            description: Some(
                "APB1 peripheral reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2rst",
                    description: Some(
                        "TIM2 reset",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim3rst",
                    description: Some(
                        "TIM3 reset",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim4rst",
                    description: Some(
                        "TIM4 reset",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim5rst",
                    description: Some(
                        "TIM5 reset",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim6rst",
                    description: Some(
                        "TIM6 reset",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim7rst",
                    description: Some(
                        "TIM7 reset",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim12rst",
                    description: Some(
                        "TIM12 reset",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim13rst",
                    description: Some(
                        "TIM13 reset",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim14rst",
                    description: Some(
                        "TIM14 reset",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim1rst",
                    description: Some(
                        "Low power timer 1 reset",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wwdgrst",
                    description: Some(
                        "Window watchdog reset",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "can3rst",
                    description: Some(
                        "CAN 3 reset",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi2rst",
                    description: Some(
                        "SPI 2 reset",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi3rst",
                    description: Some(
                        "SPI 3 reset",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spdifrxrst",
                    description: Some(
                        "SPDIF-RX reset",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart2rst",
                    description: Some(
                        "USART 2 reset",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart3rst",
                    description: Some(
                        "USART 3 reset",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uart4rst",
                    description: Some(
                        "USART 4 reset",
                    ),
                    bit_offset: 19,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uart5rst",
                    description: Some(
                        "USART 5 reset",
                    ),
                    bit_offset: 20,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c1rst",
                    description: Some(
                        "I2C 1 reset",
                    ),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c2rst",
                    description: Some(
                        "I2C 2 reset",
                    ),
                    bit_offset: 22,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c3rst",
                    description: Some(
                        "I2C3 reset",
                    ),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c4rst",
                    description: Some(
                        "I2C 4 reset",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "can1rst",
                    description: Some(
                        "CAN1 reset",
                    ),
                    bit_offset: 25,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "can2rst",
                    description: Some(
                        "CAN2 reset",
                    ),
                    bit_offset: 26,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cecrst",
                    description: Some(
                        "HDMI-CEC reset",
                    ),
                    bit_offset: 27,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pwrrst",
                    description: Some(
                        "Power interface reset",
                    ),
                    bit_offset: 28,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dacrst",
                    description: Some(
                        "DAC reset",
                    ),
                    bit_offset: 29,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uart7rst",
                    description: Some(
                        "UART7 reset",
                    ),
                    bit_offset: 30,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uart8rst",
                    description: Some(
                        "UART8 reset",
                    ),
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pllcfgr",
            extends: None,
            description: Some(
                "PLL configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pllm",
                    description: Some(
                        "Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock",
                    ),
                    bit_offset: 0,
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "plln",
                    description: Some(
                        "Main PLL (PLL) multiplication factor for VCO",
                    ),
                    bit_offset: 6,
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllp",
                    description: Some(
                        "Main PLL (PLL) division factor for main system clock",
                    ),
                    bit_offset: 16,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Pllp",
                    ),
                },
                Field {
                    name: "pllsrc",
                    description: Some(
                        "Main PLL(PLL) and audio PLL (PLLI2S) entry clock source",
                    ),
                    bit_offset: 22,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pllsrc",
                    ),
                },
                Field {
                    name: "pllq",
                    description: Some(
                        "Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks",
                    ),
                    bit_offset: 24,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllr",
                    description: Some(
                        "PLL division factor for DSI clock",
                    ),
                    bit_offset: 28,
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ahb1enr",
            extends: None,
            description: Some(
                "AHB1 peripheral clock register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpioaen",
                    description: Some(
                        "IO port A clock enable",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpioben",
                    description: Some(
                        "IO port B clock enable",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiocen",
                    description: Some(
                        "IO port C clock enable",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpioden",
                    description: Some(
                        "IO port D clock enable",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpioeen",
                    description: Some(
                        "IO port E clock enable",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiofen",
                    description: Some(
                        "IO port F clock enable",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiogen",
                    description: Some(
                        "IO port G clock enable",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiohen",
                    description: Some(
                        "IO port H clock enable",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpioien",
                    description: Some(
                        "IO port I clock enable",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiojen",
                    description: Some(
                        "IO port J clock enable",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpioken",
                    description: Some(
                        "IO port K clock enable",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "crcen",
                    description: Some(
                        "CRC clock enable",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bkpsramen",
                    description: Some(
                        "Backup SRAM interface clock enable",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtcmramen",
                    description: Some(
                        "CCM data RAM clock enable",
                    ),
                    bit_offset: 20,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dma1en",
                    description: Some(
                        "DMA1 clock enable",
                    ),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dma2en",
                    description: Some(
                        "DMA2 clock enable",
                    ),
                    bit_offset: 22,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dma2den",
                    description: Some(
                        "DMA2D clock enable",
                    ),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ethen",
                    description: Some(
                        "Ethernet MAC clock enable",
                    ),
                    bit_offset: 25,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ethtxen",
                    description: Some(
                        "Ethernet Transmission clock enable",
                    ),
                    bit_offset: 26,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ethrxen",
                    description: Some(
                        "Ethernet Reception clock enable",
                    ),
                    bit_offset: 27,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ethptpen",
                    description: Some(
                        "Ethernet PTP clock enable",
                    ),
                    bit_offset: 28,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usb_otg_hsen",
                    description: Some(
                        "USB OTG HS clock enable",
                    ),
                    bit_offset: 29,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usb_otg_hsulpien",
                    description: Some(
                        "USB OTG HSULPI clock enable",
                    ),
                    bit_offset: 30,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sscgr",
            extends: None,
            description: Some(
                "spread spectrum clock generation register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "modper",
                    description: Some(
                        "Modulation period",
                    ),
                    bit_offset: 0,
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "incstep",
                    description: Some(
                        "Incrementation step",
                    ),
                    bit_offset: 13,
                    bit_size: 15,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spreadsel",
                    description: Some(
                        "Spread Select",
                    ),
                    bit_offset: 30,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Spreadsel",
                    ),
                },
                Field {
                    name: "sscgen",
                    description: Some(
                        "Spread spectrum modulation enable",
                    ),
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb2enr",
            extends: None,
            description: Some(
                "APB2 peripheral clock enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim1en",
                    description: Some(
                        "TIM1 clock enable",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim8en",
                    description: Some(
                        "TIM8 clock enable",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart1en",
                    description: Some(
                        "USART1 clock enable",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart6en",
                    description: Some(
                        "USART6 clock enable",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sdmmc2en",
                    description: Some(
                        "SDMMC2 clock enable",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adc1en",
                    description: Some(
                        "ADC1 clock enable",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adc2en",
                    description: Some(
                        "ADC2 clock enable",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adc3en",
                    description: Some(
                        "ADC3 clock enable",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sdmmc1en",
                    description: Some(
                        "SDMMC1 clock enable",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi1en",
                    description: Some(
                        "SPI1 clock enable",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi4en",
                    description: Some(
                        "SPI4 clock enable",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "syscfgen",
                    description: Some(
                        "System configuration controller clock enable",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim9en",
                    description: Some(
                        "TIM9 clock enable",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim10en",
                    description: Some(
                        "TIM10 clock enable",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim11en",
                    description: Some(
                        "TIM11 clock enable",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi5en",
                    description: Some(
                        "SPI5 clock enable",
                    ),
                    bit_offset: 20,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi6en",
                    description: Some(
                        "SPI6 clock enable",
                    ),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sai1en",
                    description: Some(
                        "SAI1 clock enable",
                    ),
                    bit_offset: 22,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sai2en",
                    description: Some(
                        "SAI2 clock enable",
                    ),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ltdcen",
                    description: Some(
                        "LTDC clock enable",
                    ),
                    bit_offset: 26,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dsien",
                    description: Some(
                        "DSI clock enable",
                    ),
                    bit_offset: 27,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dfsdm1en",
                    description: Some(
                        "DFSDM1 clock enable",
                    ),
                    bit_offset: 29,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mdiosen",
                    description: Some(
                        "MDIO clock enable",
                    ),
                    bit_offset: 30,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usbphycen",
                    description: Some(
                        "USB OTG HS PHY controller clock enable",
                    ),
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ahb2enr",
            extends: None,
            description: Some(
                "AHB2 peripheral clock enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dcmien",
                    description: Some(
                        "Camera interface enable",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jpegen",
                    description: Some(
                        "JPEG enable",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aesen",
                    description: Some(
                        "AES module clock enable",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "crypen",
                    description: Some(
                        "Cryptographic modules clock enable",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hashen",
                    description: Some(
                        "Hash modules clock enable",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rngen",
                    description: Some(
                        "Random number generator clock enable",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usb_otg_fsen",
                    description: Some(
                        "USB OTG FS clock enable",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ahb3enr",
            extends: None,
            description: Some(
                "AHB3 peripheral clock enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fmcen",
                    description: Some(
                        "Flexible memory controller module clock enable",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "quadspien",
                    description: Some(
                        "Quad SPI memory controller clock enable",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dckcfgr2",
            extends: None,
            description: Some(
                "dedicated clocks configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "usart1sel",
                    description: Some(
                        "USART 1 clock source selection",
                    ),
                    bit_offset: 0,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Usart1sel",
                    ),
                },
                Field {
                    name: "usart2sel",
                    description: Some(
                        "USART 2 clock source selection",
                    ),
                    bit_offset: 2,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Usart2sel",
                    ),
                },
                Field {
                    name: "usart3sel",
                    description: Some(
                        "USART 3 clock source selection",
                    ),
                    bit_offset: 4,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Usart2sel",
                    ),
                },
                Field {
                    name: "uart4sel",
                    description: Some(
                        "UART 4 clock source selection",
                    ),
                    bit_offset: 6,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Usart2sel",
                    ),
                },
                Field {
                    name: "uart5sel",
                    description: Some(
                        "UART 5 clock source selection",
                    ),
                    bit_offset: 8,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Usart2sel",
                    ),
                },
                Field {
                    name: "usart6sel",
                    description: Some(
                        "USART 6 clock source selection",
                    ),
                    bit_offset: 10,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Usart1sel",
                    ),
                },
                Field {
                    name: "uart7sel",
                    description: Some(
                        "UART 7 clock source selection",
                    ),
                    bit_offset: 12,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Usart2sel",
                    ),
                },
                Field {
                    name: "uart8sel",
                    description: Some(
                        "UART 8 clock source selection",
                    ),
                    bit_offset: 14,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Usart2sel",
                    ),
                },
                Field {
                    name: "i2c1sel",
                    description: Some(
                        "I2C1 clock source selection",
                    ),
                    bit_offset: 16,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Icsel",
                    ),
                },
                Field {
                    name: "i2c2sel",
                    description: Some(
                        "I2C2 clock source selection",
                    ),
                    bit_offset: 18,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Icsel",
                    ),
                },
                Field {
                    name: "i2c3sel",
                    description: Some(
                        "I2C3 clock source selection",
                    ),
                    bit_offset: 20,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Icsel",
                    ),
                },
                Field {
                    name: "i2c4sel",
                    description: Some(
                        "I2C4 clock source selection",
                    ),
                    bit_offset: 22,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Icsel",
                    ),
                },
                Field {
                    name: "lptim1sel",
                    description: Some(
                        "Low power timer 1 clock source selection",
                    ),
                    bit_offset: 24,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Lptimsel",
                    ),
                },
                Field {
                    name: "cecsel",
                    description: Some(
                        "HDMI-CEC clock source selection",
                    ),
                    bit_offset: 26,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Cecsel",
                    ),
                },
                Field {
                    name: "ck48msel",
                    description: Some(
                        "48MHz clock source selection",
                    ),
                    bit_offset: 27,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ckmsel",
                    ),
                },
                Field {
                    name: "sdmmc1sel",
                    description: Some(
                        "SDMMC1 clock source selection",
                    ),
                    bit_offset: 28,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sdmmcsel",
                    ),
                },
                Field {
                    name: "sdmmc2sel",
                    description: Some(
                        "SDMMC2 clock source selection",
                    ),
                    bit_offset: 29,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sdmmcsel",
                    ),
                },
                Field {
                    name: "dsisel",
                    description: Some(
                        "DSI clock source selection",
                    ),
                    bit_offset: 30,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Dsisel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Bdcr",
            extends: None,
            description: Some(
                "Backup domain control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lseon",
                    description: Some(
                        "External low-speed oscillator enable",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lserdy",
                    description: Some(
                        "External low-speed oscillator ready",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lsebyp",
                    description: Some(
                        "External low-speed oscillator bypass",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lsedrv",
                    description: Some(
                        "LSE oscillator drive capability",
                    ),
                    bit_offset: 3,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Lsedrv",
                    ),
                },
                Field {
                    name: "rtcsel",
                    description: Some(
                        "RTC clock source selection",
                    ),
                    bit_offset: 8,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Rtcsel",
                    ),
                },
                Field {
                    name: "rtcen",
                    description: Some(
                        "RTC clock enable",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bdrst",
                    description: Some(
                        "Backup domain software reset",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb1lpenr",
            extends: None,
            description: Some(
                "APB1 peripheral clock enable in low power mode register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2lpen",
                    description: Some(
                        "TIM2 clock enable during Sleep mode",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim3lpen",
                    description: Some(
                        "TIM3 clock enable during Sleep mode",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim4lpen",
                    description: Some(
                        "TIM4 clock enable during Sleep mode",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim5lpen",
                    description: Some(
                        "TIM5 clock enable during Sleep mode",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim6lpen",
                    description: Some(
                        "TIM6 clock enable during Sleep mode",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim7lpen",
                    description: Some(
                        "TIM7 clock enable during Sleep mode",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim12lpen",
                    description: Some(
                        "TIM12 clock enable during Sleep mode",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim13lpen",
                    description: Some(
                        "TIM13 clock enable during Sleep mode",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim14lpen",
                    description: Some(
                        "TIM14 clock enable during Sleep mode",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim1lpen",
                    description: Some(
                        "low power timer 1 clock enable during Sleep mode",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rtclpen",
                    description: Some(
                        "RTCAPB clock enable during Sleep mode",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wwdglpen",
                    description: Some(
                        "Window watchdog clock enable during Sleep mode",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "can3lpen",
                    description: Some(
                        "CAN 3 clock enable during Sleep mode",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi2lpen",
                    description: Some(
                        "SPI2 clock enable during Sleep mode",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi3lpen",
                    description: Some(
                        "SPI3 clock enable during Sleep mode",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spdifrxlpen",
                    description: Some(
                        "SPDIF-RX clock enable during sleep mode",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart2lpen",
                    description: Some(
                        "USART2 clock enable during Sleep mode",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart3lpen",
                    description: Some(
                        "USART3 clock enable during Sleep mode",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uart4lpen",
                    description: Some(
                        "UART4 clock enable during Sleep mode",
                    ),
                    bit_offset: 19,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uart5lpen",
                    description: Some(
                        "UART5 clock enable during Sleep mode",
                    ),
                    bit_offset: 20,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c1lpen",
                    description: Some(
                        "I2C1 clock enable during Sleep mode",
                    ),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c2lpen",
                    description: Some(
                        "I2C2 clock enable during Sleep mode",
                    ),
                    bit_offset: 22,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c3lpen",
                    description: Some(
                        "I2C3 clock enable during Sleep mode",
                    ),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c4lpen",
                    description: Some(
                        "I2C4 clock enable during Sleep mode",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "can1lpen",
                    description: Some(
                        "CAN 1 clock enable during Sleep mode",
                    ),
                    bit_offset: 25,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "can2lpen",
                    description: Some(
                        "CAN 2 clock enable during Sleep mode",
                    ),
                    bit_offset: 26,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ceclpen",
                    description: Some(
                        "HDMI-CEN clock enable during Sleep mode",
                    ),
                    bit_offset: 27,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pwrlpen",
                    description: Some(
                        "Power interface clock enable during Sleep mode",
                    ),
                    bit_offset: 28,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "daclpen",
                    description: Some(
                        "DAC interface clock enable during Sleep mode",
                    ),
                    bit_offset: 29,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uart7lpen",
                    description: Some(
                        "UART7 clock enable during Sleep mode",
                    ),
                    bit_offset: 30,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uart8lpen",
                    description: Some(
                        "UART8 clock enable during Sleep mode",
                    ),
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Csr",
            extends: None,
            description: Some(
                "clock control & status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsion",
                    description: Some(
                        "Internal low-speed oscillator enable",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lsirdy",
                    description: Some(
                        "Internal low-speed oscillator ready",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rmvf",
                    description: Some(
                        "Remove reset flag",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "borrstf",
                    description: Some(
                        "BOR reset flag",
                    ),
                    bit_offset: 25,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "padrstf",
                    description: Some(
                        "PIN reset flag",
                    ),
                    bit_offset: 26,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "porrstf",
                    description: Some(
                        "POR/PDR reset flag",
                    ),
                    bit_offset: 27,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sftrstf",
                    description: Some(
                        "Software reset flag",
                    ),
                    bit_offset: 28,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wdgrstf",
                    description: Some(
                        "Independent watchdog reset flag",
                    ),
                    bit_offset: 29,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wwdgrstf",
                    description: Some(
                        "Window watchdog reset flag",
                    ),
                    bit_offset: 30,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpwrrstf",
                    description: Some(
                        "Low-power reset flag",
                    ),
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some(
                "clock control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsion",
                    description: Some(
                        "Internal high-speed clock enable",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsirdy",
                    description: Some(
                        "Internal high-speed clock ready flag",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsitrim",
                    description: Some(
                        "Internal high-speed clock trimming",
                    ),
                    bit_offset: 3,
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsical",
                    description: Some(
                        "Internal high-speed clock calibration",
                    ),
                    bit_offset: 8,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hseon",
                    description: Some(
                        "HSE clock enable",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hserdy",
                    description: Some(
                        "HSE clock ready flag",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsebyp",
                    description: Some(
                        "HSE clock bypass",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "csson",
                    description: Some(
                        "Clock security system enable",
                    ),
                    bit_offset: 19,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllon",
                    description: Some(
                        "Main PLL (PLL) enable",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllrdy",
                    description: Some(
                        "Main PLL (PLL) clock ready flag",
                    ),
                    bit_offset: 25,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "plli2son",
                    description: Some(
                        "PLLI2S enable",
                    ),
                    bit_offset: 26,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "plli2srdy",
                    description: Some(
                        "PLLI2S clock ready flag",
                    ),
                    bit_offset: 27,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllsaion",
                    description: Some(
                        "PLLSAI enable",
                    ),
                    bit_offset: 28,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllsairdy",
                    description: Some(
                        "PLLSAI clock ready flag",
                    ),
                    bit_offset: 29,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pllsaicfgr",
            extends: None,
            description: Some(
                "PLL configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pllsain",
                    description: Some(
                        "PLLSAI division factor for VCO",
                    ),
                    bit_offset: 6,
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllsaip",
                    description: Some(
                        "PLLSAI division factor for 48MHz clock",
                    ),
                    bit_offset: 16,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Pllsaip",
                    ),
                },
                Field {
                    name: "pllsaiq",
                    description: Some(
                        "PLLSAI division factor for SAI clock",
                    ),
                    bit_offset: 24,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllsair",
                    description: Some(
                        "PLLSAI division factor for LCD clock",
                    ),
                    bit_offset: 28,
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ahb3lpenr",
            extends: None,
            description: Some(
                "AHB3 peripheral clock enable in low power mode register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fmclpen",
                    description: Some(
                        "Flexible memory controller module clock enable during Sleep mode",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "quadspilpen",
                    description: Some(
                        "Quand SPI memory controller clock enable during Sleep mode",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ahb1lpenr",
            extends: None,
            description: Some(
                "AHB1 peripheral clock enable in low power mode register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpioalpen",
                    description: Some(
                        "IO port A clock enable during sleep mode",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpioblpen",
                    description: Some(
                        "IO port B clock enable during Sleep mode",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpioclpen",
                    description: Some(
                        "IO port C clock enable during Sleep mode",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiodlpen",
                    description: Some(
                        "IO port D clock enable during Sleep mode",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpioelpen",
                    description: Some(
                        "IO port E clock enable during Sleep mode",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpioflpen",
                    description: Some(
                        "IO port F clock enable during Sleep mode",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpioglpen",
                    description: Some(
                        "IO port G clock enable during Sleep mode",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiohlpen",
                    description: Some(
                        "IO port H clock enable during Sleep mode",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpioilpen",
                    description: Some(
                        "IO port I clock enable during Sleep mode",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiojlpen",
                    description: Some(
                        "IO port J clock enable during Sleep mode",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpioklpen",
                    description: Some(
                        "IO port K clock enable during Sleep mode",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "crclpen",
                    description: Some(
                        "CRC clock enable during Sleep mode",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "axilpen",
                    description: Some(
                        "AXI to AHB bridge clock enable during Sleep mode",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "flashlpen",
                    description: Some(
                        "Flash interface clock enable during Sleep mode",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sram1lpen",
                    description: Some(
                        "SRAM 1interface clock enable during Sleep mode",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sram2lpen",
                    description: Some(
                        "SRAM 2 interface clock enable during Sleep mode",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bkpsramlpen",
                    description: Some(
                        "Backup SRAM interface clock enable during Sleep mode",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sram3lpen",
                    description: Some(
                        "SRAM 3 interface clock enable during Sleep mode",
                    ),
                    bit_offset: 19,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dtcmlpen",
                    description: Some(
                        "DTCM RAM interface clock enable during Sleep mode",
                    ),
                    bit_offset: 20,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dma1lpen",
                    description: Some(
                        "DMA1 clock enable during Sleep mode",
                    ),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dma2lpen",
                    description: Some(
                        "DMA2 clock enable during Sleep mode",
                    ),
                    bit_offset: 22,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dma2dlpen",
                    description: Some(
                        "DMA2D clock enable during Sleep mode",
                    ),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ethlpen",
                    description: Some(
                        "Ethernet MAC clock enable during Sleep mode",
                    ),
                    bit_offset: 25,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ethtxlpen",
                    description: Some(
                        "Ethernet transmission clock enable during Sleep mode",
                    ),
                    bit_offset: 26,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ethrxlpen",
                    description: Some(
                        "Ethernet reception clock enable during Sleep mode",
                    ),
                    bit_offset: 27,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ethptplpen",
                    description: Some(
                        "Ethernet PTP clock enable during Sleep mode",
                    ),
                    bit_offset: 28,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usb_otg_hslpen",
                    description: Some(
                        "USB OTG HS clock enable during Sleep mode",
                    ),
                    bit_offset: 29,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usb_otg_hsulpilpen",
                    description: Some(
                        "USB OTG HS ULPI clock enable during Sleep mode",
                    ),
                    bit_offset: 30,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb2rstr",
            extends: None,
            description: Some(
                "APB2 peripheral reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim1rst",
                    description: Some(
                        "TIM1 reset",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim8rst",
                    description: Some(
                        "TIM8 reset",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart1rst",
                    description: Some(
                        "USART1 reset",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart6rst",
                    description: Some(
                        "USART6 reset",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sdmmc2rst",
                    description: Some(
                        "SDMMC2 reset",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adcrst",
                    description: Some(
                        "ADC interface reset (common to all ADCs)",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sdmmc1rst",
                    description: Some(
                        "SDMMC1 reset",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi1rst",
                    description: Some(
                        "SPI 1 reset",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi4rst",
                    description: Some(
                        "SPI4 reset",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "syscfgrst",
                    description: Some(
                        "System configuration controller reset",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim9rst",
                    description: Some(
                        "TIM9 reset",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim10rst",
                    description: Some(
                        "TIM10 reset",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim11rst",
                    description: Some(
                        "TIM11 reset",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi5rst",
                    description: Some(
                        "SPI5 reset",
                    ),
                    bit_offset: 20,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi6rst",
                    description: Some(
                        "SPI6 reset",
                    ),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sai1rst",
                    description: Some(
                        "SAI1 reset",
                    ),
                    bit_offset: 22,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sai2rst",
                    description: Some(
                        "SAI2 reset",
                    ),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ltdcrst",
                    description: Some(
                        "LTDC reset",
                    ),
                    bit_offset: 26,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dsirst",
                    description: Some(
                        "DSI reset",
                    ),
                    bit_offset: 27,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dfsdm1rst",
                    description: Some(
                        "DFSDM 1 reset",
                    ),
                    bit_offset: 29,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mdiosrst",
                    description: Some(
                        "MDIOS reset",
                    ),
                    bit_offset: 30,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usbphycrst",
                    description: Some(
                        "USB OTG HS PHY controller reset",
                    ),
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb2lpenr",
            extends: None,
            description: Some(
                "APB2 peripheral clock enabled in low power mode register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim1lpen",
                    description: Some(
                        "TIM1 clock enable during Sleep mode",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim8lpen",
                    description: Some(
                        "TIM8 clock enable during Sleep mode",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart1lpen",
                    description: Some(
                        "USART1 clock enable during Sleep mode",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart6lpen",
                    description: Some(
                        "USART6 clock enable during Sleep mode",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sdmmc2lpen",
                    description: Some(
                        "SDMMC2 clock enable during Sleep mode",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adc1lpen",
                    description: Some(
                        "ADC1 clock enable during Sleep mode",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adc2lpen",
                    description: Some(
                        "ADC2 clock enable during Sleep mode",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adc3lpen",
                    description: Some(
                        "ADC 3 clock enable during Sleep mode",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sdmmc1lpen",
                    description: Some(
                        "SDMMC1 clock enable during Sleep mode",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi1lpen",
                    description: Some(
                        "SPI 1 clock enable during Sleep mode",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi4lpen",
                    description: Some(
                        "SPI 4 clock enable during Sleep mode",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "syscfglpen",
                    description: Some(
                        "System configuration controller clock enable during Sleep mode",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim9lpen",
                    description: Some(
                        "TIM9 clock enable during sleep mode",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim10lpen",
                    description: Some(
                        "TIM10 clock enable during Sleep mode",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim11lpen",
                    description: Some(
                        "TIM11 clock enable during Sleep mode",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi5lpen",
                    description: Some(
                        "SPI 5 clock enable during Sleep mode",
                    ),
                    bit_offset: 20,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi6lpen",
                    description: Some(
                        "SPI 6 clock enable during Sleep mode",
                    ),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sai1lpen",
                    description: Some(
                        "SAI1 clock enable during sleep mode",
                    ),
                    bit_offset: 22,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sai2lpen",
                    description: Some(
                        "SAI2 clock enable during sleep mode",
                    ),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ltdclpen",
                    description: Some(
                        "LTDC clock enable during sleep mode",
                    ),
                    bit_offset: 26,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dsilpen",
                    description: Some(
                        "DSI clock enable during Sleep mode",
                    ),
                    bit_offset: 27,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dfsdm1lpen",
                    description: Some(
                        "DFSDM1 clock enable during Sleep mode",
                    ),
                    bit_offset: 29,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mdioslpen",
                    description: Some(
                        "MDIO clock enable during Sleep mode",
                    ),
                    bit_offset: 30,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cfgr",
            extends: None,
            description: Some(
                "clock configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sw",
                    description: Some(
                        "System clock switch",
                    ),
                    bit_offset: 0,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Sw",
                    ),
                },
                Field {
                    name: "sws",
                    description: Some(
                        "System clock switch status",
                    ),
                    bit_offset: 2,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Sw",
                    ),
                },
                Field {
                    name: "hpre",
                    description: Some(
                        "AHB prescaler",
                    ),
                    bit_offset: 4,
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Hpre",
                    ),
                },
                Field {
                    name: "ppre1",
                    description: Some(
                        "APB Low speed prescaler (APB1)",
                    ),
                    bit_offset: 10,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Ppre",
                    ),
                },
                Field {
                    name: "ppre2",
                    description: Some(
                        "APB high-speed prescaler (APB2)",
                    ),
                    bit_offset: 13,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Ppre",
                    ),
                },
                Field {
                    name: "rtcpre",
                    description: Some(
                        "HSE division factor for RTC clock",
                    ),
                    bit_offset: 16,
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mco1sel",
                    description: Some(
                        "Microcontroller clock output 1",
                    ),
                    bit_offset: 21,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Mco1sel",
                    ),
                },
                Field {
                    name: "i2ssrc",
                    description: Some(
                        "I2S clock selection",
                    ),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Issrc",
                    ),
                },
                Field {
                    name: "mco1pre",
                    description: Some(
                        "MCO1 prescaler",
                    ),
                    bit_offset: 24,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Mcopre",
                    ),
                },
                Field {
                    name: "mco2pre",
                    description: Some(
                        "MCO2 prescaler",
                    ),
                    bit_offset: 27,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Mcopre",
                    ),
                },
                Field {
                    name: "mco2sel",
                    description: Some(
                        "Microcontroller clock output 2",
                    ),
                    bit_offset: 30,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Mco2sel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Ahb2rstr",
            extends: None,
            description: Some(
                "AHB2 peripheral reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dcmirst",
                    description: Some(
                        "Camera interface reset",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aesrst",
                    description: Some(
                        "AES module reset",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cryprst",
                    description: Some(
                        "Cryptographic module reset",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsahrst",
                    description: Some(
                        "Hash module reset",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rngrst",
                    description: Some(
                        "Random number generator module reset",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usb_otg_fsrst",
                    description: Some(
                        "USB OTG FS module reset",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ahb3rstr",
            extends: None,
            description: Some(
                "AHB3 peripheral reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fmcrst",
                    description: Some(
                        "Flexible memory controller module reset",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "quadspirst",
                    description: Some(
                        "Quad SPI memory controller reset",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb1enr",
            extends: None,
            description: Some(
                "APB1 peripheral clock enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2en",
                    description: Some(
                        "TIM2 clock enable",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim3en",
                    description: Some(
                        "TIM3 clock enable",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim4en",
                    description: Some(
                        "TIM4 clock enable",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim5en",
                    description: Some(
                        "TIM5 clock enable",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim6en",
                    description: Some(
                        "TIM6 clock enable",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim7en",
                    description: Some(
                        "TIM7 clock enable",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim12en",
                    description: Some(
                        "TIM12 clock enable",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim13en",
                    description: Some(
                        "TIM13 clock enable",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim14en",
                    description: Some(
                        "TIM14 clock enable",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim1en",
                    description: Some(
                        "Low power timer 1 clock enable",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rtcen",
                    description: Some(
                        "RTCAPB clock enable",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wwdgen",
                    description: Some(
                        "Window watchdog clock enable",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "can3en",
                    description: Some(
                        "CAN 3 enable",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi2en",
                    description: Some(
                        "SPI2 clock enable",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi3en",
                    description: Some(
                        "SPI3 clock enable",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spdifrxen",
                    description: Some(
                        "SPDIF-RX clock enable",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart2en",
                    description: Some(
                        "USART 2 clock enable",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart3en",
                    description: Some(
                        "USART3 clock enable",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uart4en",
                    description: Some(
                        "UART4 clock enable",
                    ),
                    bit_offset: 19,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uart5en",
                    description: Some(
                        "UART5 clock enable",
                    ),
                    bit_offset: 20,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c1en",
                    description: Some(
                        "I2C1 clock enable",
                    ),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c2en",
                    description: Some(
                        "I2C2 clock enable",
                    ),
                    bit_offset: 22,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c3en",
                    description: Some(
                        "I2C3 clock enable",
                    ),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c4en",
                    description: Some(
                        "I2C4 clock enable",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "can1en",
                    description: Some(
                        "CAN 1 clock enable",
                    ),
                    bit_offset: 25,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "can2en",
                    description: Some(
                        "CAN 2 clock enable",
                    ),
                    bit_offset: 26,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cecen",
                    description: Some(
                        "HDMI-CEN clock enable",
                    ),
                    bit_offset: 27,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pwren",
                    description: Some(
                        "Power interface clock enable",
                    ),
                    bit_offset: 28,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dacen",
                    description: Some(
                        "DAC interface clock enable",
                    ),
                    bit_offset: 29,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uart7en",
                    description: Some(
                        "UART7 clock enable",
                    ),
                    bit_offset: 30,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uart8en",
                    description: Some(
                        "UART8 clock enable",
                    ),
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cir",
            extends: None,
            description: Some(
                "clock interrupt register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsirdyf",
                    description: Some(
                        "LSI ready interrupt flag",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lserdyf",
                    description: Some(
                        "LSE ready interrupt flag",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsirdyf",
                    description: Some(
                        "HSI ready interrupt flag",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hserdyf",
                    description: Some(
                        "HSE ready interrupt flag",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllrdyf",
                    description: Some(
                        "Main PLL (PLL) ready interrupt flag",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "plli2srdyf",
                    description: Some(
                        "PLLI2S ready interrupt flag",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllsairdyf",
                    description: Some(
                        "PLLSAI ready interrupt flag",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cssf",
                    description: Some(
                        "Clock security system interrupt flag",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lsirdyie",
                    description: Some(
                        "LSI ready interrupt enable",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lserdyie",
                    description: Some(
                        "LSE ready interrupt enable",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsirdyie",
                    description: Some(
                        "HSI ready interrupt enable",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hserdyie",
                    description: Some(
                        "HSE ready interrupt enable",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllrdyie",
                    description: Some(
                        "Main PLL (PLL) ready interrupt enable",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "plli2srdyie",
                    description: Some(
                        "PLLI2S ready interrupt enable",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllsairdyie",
                    description: Some(
                        "PLLSAI Ready Interrupt Enable",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lsirdyc",
                    description: Some(
                        "LSI ready interrupt clear",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lserdyc",
                    description: Some(
                        "LSE ready interrupt clear",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsirdyc",
                    description: Some(
                        "HSI ready interrupt clear",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hserdyc",
                    description: Some(
                        "HSE ready interrupt clear",
                    ),
                    bit_offset: 19,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllrdyc",
                    description: Some(
                        "Main PLL(PLL) ready interrupt clear",
                    ),
                    bit_offset: 20,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "plli2srdyc",
                    description: Some(
                        "PLLI2S ready interrupt clear",
                    ),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllsairdyc",
                    description: Some(
                        "PLLSAI Ready Interrupt Clear",
                    ),
                    bit_offset: 22,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cssc",
                    description: Some(
                        "Clock security system interrupt clear",
                    ),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ahb1rstr",
            extends: None,
            description: Some(
                "AHB1 peripheral reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpioarst",
                    description: Some(
                        "IO port A reset",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiobrst",
                    description: Some(
                        "IO port B reset",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiocrst",
                    description: Some(
                        "IO port C reset",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiodrst",
                    description: Some(
                        "IO port D reset",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpioerst",
                    description: Some(
                        "IO port E reset",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiofrst",
                    description: Some(
                        "IO port F reset",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiogrst",
                    description: Some(
                        "IO port G reset",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiohrst",
                    description: Some(
                        "IO port H reset",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpioirst",
                    description: Some(
                        "IO port I reset",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiojrst",
                    description: Some(
                        "IO port J reset",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiokrst",
                    description: Some(
                        "IO port K reset",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "crcrst",
                    description: Some(
                        "CRC reset",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dma1rst",
                    description: Some(
                        "DMA2 reset",
                    ),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dma2rst",
                    description: Some(
                        "DMA2 reset",
                    ),
                    bit_offset: 22,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dma2drst",
                    description: Some(
                        "DMA2D reset",
                    ),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ethrst",
                    description: Some(
                        "Ethernet MAC reset",
                    ),
                    bit_offset: 25,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usb_otg_hsrst",
                    description: Some(
                        "USB OTG HS module reset",
                    ),
                    bit_offset: 29,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Ckmsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "PLL",
                    description: Some(
                        "48MHz clock from PLL is selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLLSAI",
                    description: Some(
                        "48MHz clock from PLLSAI is selected",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pllp",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "PLLP=2",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "PLLP=4",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV6",
                    description: Some(
                        "PLLP=6",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "PLLP=8",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Rtcsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "NOCLOCK",
                    description: Some(
                        "No clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE oscillator clock used as RTC clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "LSI",
                    description: Some(
                        "LSI oscillator clock used as RTC clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE oscillator clock divided by a prescaler used as RTC clock",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Pllsaip",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "PLL*P=2",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "PLL*P=4",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV6",
                    description: Some(
                        "PLL*P=6",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "PLL*P=8",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Hpre",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: Some(
                        "SYSCLK not divided",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "SYSCLK divided by 2",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "SYSCLK divided by 4",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "SYSCLK divided by 8",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "DIV16",
                    description: Some(
                        "SYSCLK divided by 16",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "DIV64",
                    description: Some(
                        "SYSCLK divided by 64",
                    ),
                    value: 12,
                },
                EnumVariant {
                    name: "DIV128",
                    description: Some(
                        "SYSCLK divided by 128",
                    ),
                    value: 13,
                },
                EnumVariant {
                    name: "DIV256",
                    description: Some(
                        "SYSCLK divided by 256",
                    ),
                    value: 14,
                },
                EnumVariant {
                    name: "DIV512",
                    description: Some(
                        "SYSCLK divided by 512",
                    ),
                    value: 15,
                },
            ],
        },
        Enum {
            name: "Dfsdmsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "APB2",
                    description: Some(
                        "APB2 clock (PCLK2) selected as DFSDM1 Kernel clock source",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYSCLK",
                    description: Some(
                        "System clock (SYSCLK) clock selected as DFSDM1 Kernel clock source",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pllsrc",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI clock selected as PLL and PLLI2S clock entry",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE oscillator clock selected as PLL and PLLI2S clock entry",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pllsaidivr",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "PLLSAIDIVR = /2",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "PLLSAIDIVR = /4",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "PLLSAIDIVR = /8",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV16",
                    description: Some(
                        "PLLSAIDIVR = /16",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Sdmmcsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CK48M",
                    description: Some(
                        "48 MHz clock is selected as SD clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYSCLK",
                    description: Some(
                        "System clock is selected as SD clock",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Icsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "APB",
                    description: Some(
                        "APB clock selected as I2C clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYSCLK",
                    description: Some(
                        "System clock selected as I2C clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI clock selected as I2C clock",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Mco1sel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI clock selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE oscillator selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE oscillator clock selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "PLL",
                    description: Some(
                        "PLL clock selected",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Dsisel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DSI_PHY",
                    description: Some(
                        "DSI-PHY used as DSI byte lane clock source (usual case)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLLR",
                    description: Some(
                        "PLLR used as DSI byte lane clock source, used in case DSI PLL and DSI-PHY are off (low power mode)",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Mco2sel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "SYSCLK",
                    description: Some(
                        "System clock (SYSCLK) selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLLI2S",
                    description: Some(
                        "PLLI2S clock selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE oscillator clock selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "PLL",
                    description: Some(
                        "PLL clock selected",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Pllisdivq",
            description: None,
            bit_size: 5,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: Some(
                        "PLLI2SDIVQ = /1",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "PLLI2SDIVQ = /2",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV3",
                    description: Some(
                        "PLLI2SDIVQ = /3",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "PLLI2SDIVQ = /4",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "DIV5",
                    description: Some(
                        "PLLI2SDIVQ = /5",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "DIV6",
                    description: Some(
                        "PLLI2SDIVQ = /6",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "DIV7",
                    description: Some(
                        "PLLI2SDIVQ = /7",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "PLLI2SDIVQ = /8",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "DIV9",
                    description: Some(
                        "PLLI2SDIVQ = /9",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "DIV10",
                    description: Some(
                        "PLLI2SDIVQ = /10",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "DIV11",
                    description: Some(
                        "PLLI2SDIVQ = /11",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "DIV12",
                    description: Some(
                        "PLLI2SDIVQ = /12",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "DIV13",
                    description: Some(
                        "PLLI2SDIVQ = /13",
                    ),
                    value: 12,
                },
                EnumVariant {
                    name: "DIV14",
                    description: Some(
                        "PLLI2SDIVQ = /14",
                    ),
                    value: 13,
                },
                EnumVariant {
                    name: "DIV15",
                    description: Some(
                        "PLLI2SDIVQ = /15",
                    ),
                    value: 14,
                },
                EnumVariant {
                    name: "DIV16",
                    description: Some(
                        "PLLI2SDIVQ = /16",
                    ),
                    value: 15,
                },
                EnumVariant {
                    name: "DIV17",
                    description: Some(
                        "PLLI2SDIVQ = /17",
                    ),
                    value: 16,
                },
                EnumVariant {
                    name: "DIV18",
                    description: Some(
                        "PLLI2SDIVQ = /18",
                    ),
                    value: 17,
                },
                EnumVariant {
                    name: "DIV19",
                    description: Some(
                        "PLLI2SDIVQ = /19",
                    ),
                    value: 18,
                },
                EnumVariant {
                    name: "DIV20",
                    description: Some(
                        "PLLI2SDIVQ = /20",
                    ),
                    value: 19,
                },
                EnumVariant {
                    name: "DIV21",
                    description: Some(
                        "PLLI2SDIVQ = /21",
                    ),
                    value: 20,
                },
                EnumVariant {
                    name: "DIV22",
                    description: Some(
                        "PLLI2SDIVQ = /22",
                    ),
                    value: 21,
                },
                EnumVariant {
                    name: "DIV23",
                    description: Some(
                        "PLLI2SDIVQ = /23",
                    ),
                    value: 22,
                },
                EnumVariant {
                    name: "DIV24",
                    description: Some(
                        "PLLI2SDIVQ = /24",
                    ),
                    value: 23,
                },
                EnumVariant {
                    name: "DIV25",
                    description: Some(
                        "PLLI2SDIVQ = /25",
                    ),
                    value: 24,
                },
                EnumVariant {
                    name: "DIV26",
                    description: Some(
                        "PLLI2SDIVQ = /26",
                    ),
                    value: 25,
                },
                EnumVariant {
                    name: "DIV27",
                    description: Some(
                        "PLLI2SDIVQ = /27",
                    ),
                    value: 26,
                },
                EnumVariant {
                    name: "DIV28",
                    description: Some(
                        "PLLI2SDIVQ = /28",
                    ),
                    value: 27,
                },
                EnumVariant {
                    name: "DIV29",
                    description: Some(
                        "PLLI2SDIVQ = /29",
                    ),
                    value: 28,
                },
                EnumVariant {
                    name: "DIV30",
                    description: Some(
                        "PLLI2SDIVQ = /30",
                    ),
                    value: 29,
                },
                EnumVariant {
                    name: "DIV31",
                    description: Some(
                        "PLLI2SDIVQ = /31",
                    ),
                    value: 30,
                },
                EnumVariant {
                    name: "DIV32",
                    description: Some(
                        "PLLI2SDIVQ = /32",
                    ),
                    value: 31,
                },
            ],
        },
        Enum {
            name: "Pllsaidivq",
            description: None,
            bit_size: 5,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: Some(
                        "PLLSAIDIVQ = /1",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "PLLSAIDIVQ = /2",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV3",
                    description: Some(
                        "PLLSAIDIVQ = /3",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "PLLSAIDIVQ = /4",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "DIV5",
                    description: Some(
                        "PLLSAIDIVQ = /5",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "DIV6",
                    description: Some(
                        "PLLSAIDIVQ = /6",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "DIV7",
                    description: Some(
                        "PLLSAIDIVQ = /7",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "PLLSAIDIVQ = /8",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "DIV9",
                    description: Some(
                        "PLLSAIDIVQ = /9",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "DIV10",
                    description: Some(
                        "PLLSAIDIVQ = /10",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "DIV11",
                    description: Some(
                        "PLLSAIDIVQ = /11",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "DIV12",
                    description: Some(
                        "PLLSAIDIVQ = /12",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "DIV13",
                    description: Some(
                        "PLLSAIDIVQ = /13",
                    ),
                    value: 12,
                },
                EnumVariant {
                    name: "DIV14",
                    description: Some(
                        "PLLSAIDIVQ = /14",
                    ),
                    value: 13,
                },
                EnumVariant {
                    name: "DIV15",
                    description: Some(
                        "PLLSAIDIVQ = /15",
                    ),
                    value: 14,
                },
                EnumVariant {
                    name: "DIV16",
                    description: Some(
                        "PLLSAIDIVQ = /16",
                    ),
                    value: 15,
                },
                EnumVariant {
                    name: "DIV17",
                    description: Some(
                        "PLLSAIDIVQ = /17",
                    ),
                    value: 16,
                },
                EnumVariant {
                    name: "DIV18",
                    description: Some(
                        "PLLSAIDIVQ = /18",
                    ),
                    value: 17,
                },
                EnumVariant {
                    name: "DIV19",
                    description: Some(
                        "PLLSAIDIVQ = /19",
                    ),
                    value: 18,
                },
                EnumVariant {
                    name: "DIV20",
                    description: Some(
                        "PLLSAIDIVQ = /20",
                    ),
                    value: 19,
                },
                EnumVariant {
                    name: "DIV21",
                    description: Some(
                        "PLLSAIDIVQ = /21",
                    ),
                    value: 20,
                },
                EnumVariant {
                    name: "DIV22",
                    description: Some(
                        "PLLSAIDIVQ = /22",
                    ),
                    value: 21,
                },
                EnumVariant {
                    name: "DIV23",
                    description: Some(
                        "PLLSAIDIVQ = /23",
                    ),
                    value: 22,
                },
                EnumVariant {
                    name: "DIV24",
                    description: Some(
                        "PLLSAIDIVQ = /24",
                    ),
                    value: 23,
                },
                EnumVariant {
                    name: "DIV25",
                    description: Some(
                        "PLLSAIDIVQ = /25",
                    ),
                    value: 24,
                },
                EnumVariant {
                    name: "DIV26",
                    description: Some(
                        "PLLSAIDIVQ = /26",
                    ),
                    value: 25,
                },
                EnumVariant {
                    name: "DIV27",
                    description: Some(
                        "PLLSAIDIVQ = /27",
                    ),
                    value: 26,
                },
                EnumVariant {
                    name: "DIV28",
                    description: Some(
                        "PLLSAIDIVQ = /28",
                    ),
                    value: 27,
                },
                EnumVariant {
                    name: "DIV29",
                    description: Some(
                        "PLLSAIDIVQ = /29",
                    ),
                    value: 28,
                },
                EnumVariant {
                    name: "DIV30",
                    description: Some(
                        "PLLSAIDIVQ = /30",
                    ),
                    value: 29,
                },
                EnumVariant {
                    name: "DIV31",
                    description: Some(
                        "PLLSAIDIVQ = /31",
                    ),
                    value: 30,
                },
                EnumVariant {
                    name: "DIV32",
                    description: Some(
                        "PLLSAIDIVQ = /32",
                    ),
                    value: 31,
                },
            ],
        },
        Enum {
            name: "Lptimsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "APB1",
                    description: Some(
                        "APB1 clock (PCLK1) selected as LPTILM1 clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LSI",
                    description: Some(
                        "LSI clock is selected as LPTILM1 clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI clock is selected as LPTILM1 clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE clock is selected as LPTILM1 clock",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Mcopre",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: Some(
                        "No division",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "Division by 2",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "DIV3",
                    description: Some(
                        "Division by 3",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "Division by 4",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "DIV5",
                    description: Some(
                        "Division by 5",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Cecsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE clock is selected as HDMI-CEC clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HSI_DIV488",
                    description: Some(
                        "HSI divided by 488 clock is selected as HDMI-CEC clock",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Saisel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PLLSAI",
                    description: Some(
                        "SAI2 clock frequency = f(PLLSAI_Q) / PLLSAIDIVQ",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLLI2S",
                    description: Some(
                        "SAI2 clock frequency = f(PLLI2S_Q) / PLLI2SDIVQ",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "AFIF",
                    description: Some(
                        "SAI2 clock frequency = Alternate function input frequency",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HSI_HSE",
                    description: Some(
                        "SAI2 clock frequency = HSI or HSE",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Spreadsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CENTER",
                    description: Some(
                        "Center spread",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DOWN",
                    description: Some(
                        "Down spread",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Adfsdmsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "SAI1",
                    description: Some(
                        "SAI1 clock selected as DFSDM1 Audio clock source",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SAI2",
                    description: Some(
                        "SAI2 clock selected as DFSDM1 Audio clock source",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Usart1sel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "APB2",
                    description: Some(
                        "APB2 clock (PCLK2) is selected as USART clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYSCLK",
                    description: Some(
                        "System clock is selected as USART clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI clock is selected as USART clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE clock is selected as USART clock",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Lsedrv",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "LOW",
                    description: Some(
                        "Low driving capability",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MEDIUMHIGH",
                    description: Some(
                        "Medium high driving capability",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "MEDIUMLOW",
                    description: Some(
                        "Medium low driving capability",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HIGH",
                    description: Some(
                        "High driving capability",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Ppre",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: Some(
                        "HCLK not divided",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "HCLK divided by 2",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "HCLK divided by 4",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "HCLK divided by 8",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "DIV16",
                    description: Some(
                        "HCLK divided by 16",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Usart2sel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "APB1",
                    description: Some(
                        "APB1 clock (PCLK1) is selected as USART clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYSCLK",
                    description: Some(
                        "System clock is selected as USART clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI clock is selected as USART clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE clock is selected as USART clock",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Sw",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "HSI",
                    description: Some(
                        "HSI oscillator used as system clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE oscillator used as system clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL",
                    description: Some(
                        "PLL used as system clock",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Pllisp",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "PLL*P=2",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "PLL*P=4",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV6",
                    description: Some(
                        "PLL*P=6",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "PLL*P=8",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Timpre",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "MUL2",
                    description: Some(
                        "If the APB prescaler is configured 1, TIMxCLK = PCLKx. Otherwise, TIMxCLK = 2xPCLKx",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MUL4",
                    description: Some(
                        "If the APB prescaler is configured 1, 2 or 4, TIMxCLK = HCLK. Otherwise, TIMxCLK = 4xPCLKx",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Issrc",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "PLLI2S",
                    description: Some(
                        "PLLI2S clock used as I2S clock source",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "CKIN",
                    description: Some(
                        "External clock mapped on the I2S_CKIN pin used as I2S clock source",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
