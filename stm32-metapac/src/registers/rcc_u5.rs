
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
                        "RCC clock control register",
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
                    name: "icscr1",
                    description: Some(
                        "RCC internal clock sources calibration register 1",
                    ),
                    array: None,
                    byte_offset: 8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Icscr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "icscr2",
                    description: Some(
                        "RCC internal clock sources calibration register 2",
                    ),
                    array: None,
                    byte_offset: 12,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Icscr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "icscr3",
                    description: Some(
                        "RCC internal clock sources calibration register 3",
                    ),
                    array: None,
                    byte_offset: 16,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Icscr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "crrcr",
                    description: Some(
                        "RCC clock recovery RC register",
                    ),
                    array: None,
                    byte_offset: 20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Crrcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfgr1",
                    description: Some(
                        "RCC clock configuration register 1",
                    ),
                    array: None,
                    byte_offset: 28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfgr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfgr2",
                    description: Some(
                        "RCC clock configuration register 2",
                    ),
                    array: None,
                    byte_offset: 32,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfgr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfgr3",
                    description: Some(
                        "RCC clock configuration register 3",
                    ),
                    array: None,
                    byte_offset: 36,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfgr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pll1cfgr",
                    description: Some(
                        "RCC PLL1 configuration register",
                    ),
                    array: None,
                    byte_offset: 40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pll1cfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pll2cfgr",
                    description: Some(
                        "RCC PLL2 configuration register",
                    ),
                    array: None,
                    byte_offset: 44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pll2cfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pll3cfgr",
                    description: Some(
                        "RCC PLL3 configuration register",
                    ),
                    array: None,
                    byte_offset: 48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pll3cfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pll1divr",
                    description: Some(
                        "RCC PLL1 dividers register",
                    ),
                    array: None,
                    byte_offset: 52,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pll1divr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pll1fracr",
                    description: Some(
                        "RCC PLL1 fractional divider register",
                    ),
                    array: None,
                    byte_offset: 56,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pll1fracr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pll2divr",
                    description: Some(
                        "RCC PLL2 dividers configuration register",
                    ),
                    array: None,
                    byte_offset: 60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pll2divr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pll2fracr",
                    description: Some(
                        "RCC PLL2 fractional divider register",
                    ),
                    array: None,
                    byte_offset: 64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pll2fracr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pll3divr",
                    description: Some(
                        "RCC PLL3 dividers configuration register",
                    ),
                    array: None,
                    byte_offset: 68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pll3divr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pll3fracr",
                    description: Some(
                        "RCC PLL3 fractional divider register",
                    ),
                    array: None,
                    byte_offset: 72,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pll3fracr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cier",
                    description: Some(
                        "RCC clock interrupt enable register",
                    ),
                    array: None,
                    byte_offset: 80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cier",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cifr",
                    description: Some(
                        "RCC clock interrupt flag register",
                    ),
                    array: None,
                    byte_offset: 84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cifr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cicr",
                    description: Some(
                        "RCC clock interrupt clear register",
                    ),
                    array: None,
                    byte_offset: 88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cicr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb1rstr",
                    description: Some(
                        "RCC AHB1 peripheral reset register",
                    ),
                    array: None,
                    byte_offset: 96,
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
                    name: "ahb2rstr1",
                    description: Some(
                        "RCC AHB2 peripheral reset register 1",
                    ),
                    array: None,
                    byte_offset: 100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb2rstr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb2rstr2",
                    description: Some(
                        "RCC AHB2 peripheral reset register 2",
                    ),
                    array: None,
                    byte_offset: 104,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb2rstr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb3rstr",
                    description: Some(
                        "RCC AHB3 peripheral reset register",
                    ),
                    array: None,
                    byte_offset: 108,
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
                    name: "apb1rstr1",
                    description: Some(
                        "RCC APB1 peripheral reset register 1",
                    ),
                    array: None,
                    byte_offset: 116,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1rstr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1rstr2",
                    description: Some(
                        "RCC APB1 peripheral reset register 2",
                    ),
                    array: None,
                    byte_offset: 120,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1rstr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2rstr",
                    description: Some(
                        "RCC APB2 peripheral reset register",
                    ),
                    array: None,
                    byte_offset: 124,
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
                    name: "apb3rstr",
                    description: Some(
                        "RCC APB3 peripheral reset register",
                    ),
                    array: None,
                    byte_offset: 128,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb3rstr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb1enr",
                    description: Some(
                        "RCC AHB1 peripheral clock enable register",
                    ),
                    array: None,
                    byte_offset: 136,
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
                    name: "ahb2enr1",
                    description: Some(
                        "RCC AHB2 peripheral clock enable register 1",
                    ),
                    array: None,
                    byte_offset: 140,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb2enr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb2enr2",
                    description: Some(
                        "RCC AHB2 peripheral clock enable register 2",
                    ),
                    array: None,
                    byte_offset: 144,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb2enr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb3enr",
                    description: Some(
                        "RCC AHB3 peripheral clock enable register",
                    ),
                    array: None,
                    byte_offset: 148,
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
                    name: "apb1enr1",
                    description: Some(
                        "RCC APB1 peripheral clock enable register 1",
                    ),
                    array: None,
                    byte_offset: 156,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1enr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1enr2",
                    description: Some(
                        "RCC APB1 peripheral clock enable register 2",
                    ),
                    array: None,
                    byte_offset: 160,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1enr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2enr",
                    description: Some(
                        "RCC APB2 peripheral clock enable register",
                    ),
                    array: None,
                    byte_offset: 164,
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
                    name: "apb3enr",
                    description: Some(
                        "RCC APB3 peripheral clock enable register",
                    ),
                    array: None,
                    byte_offset: 168,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb3enr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb1smenr",
                    description: Some(
                        "RCC AHB1 peripheral clocks enable in Sleep and Stop modes register",
                    ),
                    array: None,
                    byte_offset: 176,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb1smenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb2smenr1",
                    description: Some(
                        "RCC AHB2 peripheral clocks enable in Sleep and\tStop modes register 1",
                    ),
                    array: None,
                    byte_offset: 180,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb2smenr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb2smenr2",
                    description: Some(
                        "RCC AHB2 peripheral clocks enable in Sleep and\tStop modes register 2",
                    ),
                    array: None,
                    byte_offset: 184,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb2smenr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ahb3smenr",
                    description: Some(
                        "RCC AHB3 peripheral clocks enable in Sleep and Stop modes register",
                    ),
                    array: None,
                    byte_offset: 188,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ahb3smenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1smenr1",
                    description: Some(
                        "RCC APB1 peripheral clocks enable in Sleep and Stop modes\tregister 1",
                    ),
                    array: None,
                    byte_offset: 196,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1smenr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb1smenr2",
                    description: Some(
                        "RCC APB1 peripheral clocks enable in Sleep and\tStop modes register 2",
                    ),
                    array: None,
                    byte_offset: 200,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb1smenr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb2smenr",
                    description: Some(
                        "RCC APB2 peripheral clocks enable in Sleep and Stop modes register",
                    ),
                    array: None,
                    byte_offset: 204,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb2smenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "apb3smenr",
                    description: Some(
                        "RCC APB3 peripheral clock enable in Sleep and Stop modes register",
                    ),
                    array: None,
                    byte_offset: 208,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Apb3smenr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "srdamr",
                    description: Some(
                        "RCC SmartRun domain peripheral autonomous mode register",
                    ),
                    array: None,
                    byte_offset: 216,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Srdamr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccipr1",
                    description: Some(
                        "RCC peripherals independent clock configuration register 1",
                    ),
                    array: None,
                    byte_offset: 224,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccipr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccipr2",
                    description: Some(
                        "RCC peripherals independent clock configuration register 2",
                    ),
                    array: None,
                    byte_offset: 228,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccipr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccipr3",
                    description: Some(
                        "RCC peripherals independent clock configuration register 3",
                    ),
                    array: None,
                    byte_offset: 232,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccipr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bdcr",
                    description: Some(
                        "RCC Backup domain control register",
                    ),
                    array: None,
                    byte_offset: 240,
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
                        "RCC control/status register",
                    ),
                    array: None,
                    byte_offset: 244,
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
                    name: "seccfgr",
                    description: Some(
                        "RCC secure configuration register",
                    ),
                    array: None,
                    byte_offset: 272,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Seccfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "privcfgr",
                    description: Some(
                        "RCC privilege configuration register",
                    ),
                    array: None,
                    byte_offset: 276,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Privcfgr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Ahb2smenr2",
            extends: None,
            description: Some(
                "RCC AHB2 peripheral clocks enable in Sleep and\tStop modes register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fsmcsmen",
                    description: Some(
                        "FSMC clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "octospi1smen",
                    description: Some(
                        "OCTOSPI1 clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "octospi2smen",
                    description: Some(
                        "OCTOSPI2 clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hspi1smen",
                    description: Some(
                        "HSPI1 clock enable during Sleep and Stop modes \r This bit is set and cleared by software.\r Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sram6smen",
                    description: Some(
                        "SRAM6 clock enable during Sleep and Stop modes \r This bit is set and cleared by software.\r Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.",
                    ),
                    bit_offset: 30,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sram5smen",
                    description: Some(
                        "SRAM5 clock enable during Sleep and Stop modes \r This bit is set and cleared by software.\r Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.",
                    ),
                    bit_offset: 31,
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
                "RCC APB2 peripheral reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim1rst",
                    description: Some(
                        "TIM1 reset\r Set and cleared by software.",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi1rst",
                    description: Some(
                        "SPI1 reset\r Set and cleared by software.",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim8rst",
                    description: Some(
                        "TIM8 reset\r Set and cleared by software.",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart1rst",
                    description: Some(
                        "USART1 reset\r Set and cleared by software.",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim15rst",
                    description: Some(
                        "TIM15 reset\r Set and cleared by software.",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim16rst",
                    description: Some(
                        "TIM16 reset\r Set and cleared by software.",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim17rst",
                    description: Some(
                        "TIM17 reset\r Set and cleared by software.",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sai1rst",
                    description: Some(
                        "SAI1 reset\r Set and cleared by software.",
                    ),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sai2rst",
                    description: Some(
                        "SAI2 reset\r Set and cleared by software.",
                    ),
                    bit_offset: 22,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usbrst",
                    description: Some(
                        "USB reset\r This bit is set and cleared by software.\r Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gfxtimrst",
                    description: Some(
                        "GFXTIM reset\r This bit is set and cleared by software.\r Note: .This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.",
                    ),
                    bit_offset: 25,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ltdcrst",
                    description: Some(
                        "LTDC reset\r This bit is set and cleared by software.\r Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.",
                    ),
                    bit_offset: 26,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dsirst",
                    description: Some(
                        "DSI reset\r This bit is set and cleared by software.\r Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.",
                    ),
                    bit_offset: 27,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb1rstr2",
            extends: None,
            description: Some(
                "RCC APB1 peripheral reset register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "i2c4rst",
                    description: Some(
                        "I2C4 reset\r Set and cleared by software",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim2rst",
                    description: Some(
                        "LPTIM2 reset\r Set and cleared by software.",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c5rst",
                    description: Some(
                        "I2C5 reset\r This bit is set and cleared by software\r Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c6rst",
                    description: Some(
                        "I2C6 reset\r This bit is set and cleared by software\r Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fdcan1rst",
                    description: Some(
                        "FDCAN1 reset\r Set and cleared by software.",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ucpd1rst",
                    description: Some(
                        "UCPD1 reset\r Set and cleared by software.",
                    ),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cifr",
            extends: None,
            description: Some(
                "RCC clock interrupt flag register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsirdyf",
                    description: Some(
                        "LSI ready interrupt flag\r Set by hardware when the LSI clock becomes stable and LSIRDYIE is set.\r Cleared by software setting the LSIRDYC bit.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lserdyf",
                    description: Some(
                        "LSE ready interrupt flag\r Set by hardware when the LSE clock becomes stable and LSERDYIE is set.\r Cleared by software setting the LSERDYC bit.",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "msisrdyf",
                    description: Some(
                        "MSIS ready interrupt flag\r Set by hardware when the MSIS clock becomes stable and MSISRDYIE is set.\r Cleared by software setting the MSISRDYC bit.",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsirdyf",
                    description: Some(
                        "HSI16 ready interrupt flag\r Set by hardware when the HSI16 clock becomes stable and HSIRDYIE is set in a response to setting the HSION (see RCC_CR). When HSION is not set but the HSI16 oscillator is enabled by the peripheral through a clock request, this bit is not set and no interrupt is generated.\r Cleared by software setting the HSIRDYC bit.",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hserdyf",
                    description: Some(
                        "HSE ready interrupt flag\r Set by hardware when the HSE clock becomes stable and HSERDYIE is set.\r Cleared by software setting the HSERDYC bit.",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsi48rdyf",
                    description: Some(
                        "HSI48 ready interrupt flag\r Set by hardware when the HSI48 clock becomes stable and HSI48RDYIE is set.\r Cleared by software setting the HSI48RDYC bit.",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllrdyf",
                    description: Some(
                        "PLL1 ready interrupt flag\r Set by hardware when the PLL1 locks and PLL1RDYIE is set.\r Cleared by software setting the PLL1RDYC bit.",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "cssf",
                    description: Some(
                        "Clock security system interrupt flag\r Set by hardware when a failure is detected in the HSE oscillator.\r Cleared by software setting the CSSC bit.",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "msikrdyf",
                    description: Some(
                        "MSIK ready interrupt flag\r Set by hardware when the MSIK clock becomes stable and MSIKRDYIE is set.\r Cleared by software setting the MSIKRDYC bit.",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "shsirdyf",
                    description: Some(
                        "SHSI ready interrupt flag\r Set by hardware when the SHSI clock becomes stable and SHSIRDYIE is set.\r Cleared by software setting the SHSIRDYC bit.",
                    ),
                    bit_offset: 12,
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
                "RCC AHB3 peripheral clock enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lpgpio1en",
                    description: Some(
                        "LPGPIO1 enable\r Set and cleared by software.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pwren",
                    description: Some(
                        "PWR clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adc4en",
                    description: Some(
                        "ADC4 clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dac1en",
                    description: Some(
                        "DAC1 clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpdma1en",
                    description: Some(
                        "LPDMA1 clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adf1en",
                    description: Some(
                        "ADF1 clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gtzc2en",
                    description: Some(
                        "GTZC2 clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sram4en",
                    description: Some(
                        "SRAM4 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ccipr2",
            extends: None,
            description: Some(
                "RCC peripherals independent clock configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mdf1sel",
                    description: Some(
                        "MDF1 kernel clock source selection\r These bits are used to select the MDF1 kernel clock source.\r others: reserved",
                    ),
                    bit_offset: 0,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Mdfsel",
                    ),
                },
                Field {
                    name: "sai1sel",
                    description: Some(
                        "SAI1 kernel clock source selection\r These bits are used to select the SAI1 kernel clock source.\r others: reserved\r Note: If the selected clock is the external clock and this clock is stopped, a switch to another clock is impossible.",
                    ),
                    bit_offset: 5,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Saisel",
                    ),
                },
                Field {
                    name: "sai2sel",
                    description: Some(
                        "SAI2 kernel clock source selection\r These bits are used to select the SAI2 kernel clock source.\r others: reserved\r Note: If the selected clock is the external clock and this clock is stopped, a switch to another clock is impossible.",
                    ),
                    bit_offset: 8,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Saisel",
                    ),
                },
                Field {
                    name: "saessel",
                    description: Some(
                        "SAES kernel clock source selection\r This bit is used to select the SAES kernel clock source.",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Saessel",
                    ),
                },
                Field {
                    name: "rngsel",
                    description: Some(
                        "RNGSEL kernel clock source selection\r These bits are used to select the RNG kernel clock source.",
                    ),
                    bit_offset: 12,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Rngsel",
                    ),
                },
                Field {
                    name: "sdmmcsel",
                    description: Some(
                        "SDMMC1 and SDMMC2 kernel clock source selection\r This bit is used to select the SDMMC kernel clock source. It is recommended to change this bit only after reset and before enabling the SDMMC.",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Sdmmcsel",
                    ),
                },
                Field {
                    name: "dsisel",
                    description: Some(
                        "DSI kernel clock source selection\r This bit is used to select the DSI kernel clock source.\r This bit is only available on some devices in the STM32U5 Series. \r Refer to the device datasheet for availability of its associated peripheral. \r Note: If not present, consider this bit as reserved and keep it at reset value.",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Dsisel",
                    ),
                },
                Field {
                    name: "usart6sel",
                    description: Some(
                        "USART6 kernel clock source selection\r These bits are used to select the USART6 kernel clock source.\r The USART6 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16 or LSE.\r Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.",
                    ),
                    bit_offset: 16,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Usartsel",
                    ),
                },
                Field {
                    name: "ltdcsel",
                    description: Some(
                        "LTDC kernel clock source selection\r This bit is used to select the LTDC kernel clock source.\r Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ltdcsel",
                    ),
                },
                Field {
                    name: "octospisel",
                    description: Some(
                        "OCTOSPI1 and OCTOSPI2 kernel clock source selection\r These bits are used to select the OCTOSPI1 and OCTOSPI2 kernel clock source.",
                    ),
                    bit_offset: 20,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Octospisel",
                    ),
                },
                Field {
                    name: "hspi1sel",
                    description: Some(
                        "HSPI1 kernel clock source selection\r These bits are used to select the HSPI1 kernel clock source.\r Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.",
                    ),
                    bit_offset: 22,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Hspisel",
                    ),
                },
                Field {
                    name: "i2c5sel",
                    description: Some(
                        "I2C5 kernel clock source selection\r These bits are used to select the I2C5 kernel clock source.\r The I2C5 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16�or MSIK.\r Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.",
                    ),
                    bit_offset: 24,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Icsel",
                    ),
                },
                Field {
                    name: "i2c6sel",
                    description: Some(
                        "I2C6 kernel clock source selection\r These bits are used to select the I2C6 kernel clock source.\r The I2C6 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI16�or MSIK.\r Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.",
                    ),
                    bit_offset: 26,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Icsel",
                    ),
                },
                Field {
                    name: "otghssel",
                    description: Some(
                        "OTG_HS PHY kernel clock source selection\r These bits are used to select the OTG_HS PHY kernel clock source.\r Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.",
                    ),
                    bit_offset: 30,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Otghssel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Cfgr2",
            extends: None,
            description: Some(
                "RCC clock configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hpre",
                    description: Some(
                        "AHB prescaler\r Set and cleared by software to control the division factor of the AHB clock (HCLK).\r Depending on the device voltage range, the software must set these bits correctly to ensure that the system frequency does not exceed the maximum allowed frequency (for more details, refer to ). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value is taken into account.\r 0xxx: SYSCLK not divided",
                    ),
                    bit_offset: 0,
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Hpre",
                    ),
                },
                Field {
                    name: "ppre1",
                    description: Some(
                        "APB1 prescaler\r Set and cleared by software to control the division factor of the APB1 clock (PCLK1).\r 0xx: HCLK not divided",
                    ),
                    bit_offset: 4,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Ppre",
                    ),
                },
                Field {
                    name: "ppre2",
                    description: Some(
                        "APB2 prescaler\r Set and cleared by software to control the division factor of the APB2 clock (PCLK2).\r 0xx: HCLK not divided",
                    ),
                    bit_offset: 8,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Ppre",
                    ),
                },
                Field {
                    name: "dpre",
                    description: Some(
                        "DSI PHY prescaler\r This bitfiled is set and cleared by software to control the division factor of DSI PHY bus clock (DCLK).\r 0xx: DCLK not divided\r Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value.",
                    ),
                    bit_offset: 12,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Dpre",
                    ),
                },
                Field {
                    name: "ahb1dis",
                    description: Some(
                        "AHB1 clock disable\r This bit can be set in order to further reduce power consumption, when none of the AHB1 peripherals (except those listed hereafter) are used and when their clocks are disabled in RCC_AHB1ENR. When this bit is set, all the AHB1 peripherals clocks are off, except for FLASH, BKPSRAM, ICACHE, DCACHE1 and SRAM1.",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ahb2dis1",
                    description: Some(
                        "AHB2_1 clock disable\r This bit can be set in order to further reduce power consumption, when none of the AHB2 peripherals from RCC_AHB2ENR1 (except SRAM2 and SRAM3) are used and when their clocks are disabled in RCC_AHB2ENR1. When this bit is set, all the AHB2 peripherals clocks from RCC_AHB2ENR1 are off, except for SRAM2 and SRAM3.",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ahb2dis2",
                    description: Some(
                        "AHB2_2 clock disable\r This bit can be set in order to further reduce power consumption, when none of the AHB2 peripherals from RCC_AHB2ENR2 are used and when their clocks are disabled in RCC_AHB2ENR2. When this bit is set, all the AHB2 peripherals clocks from RCC_AHB2EBNR2 are off.",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "apb1dis",
                    description: Some(
                        "APB1 clock disable\r This bit can be set in order to further reduce power consumption, when none of the APB1 peripherals (except IWDG) are used and when their clocks are disabled in RCC_APB1ENR. When this bit is set, all the APB1 peripherals clocks are off, except for IWDG.",
                    ),
                    bit_offset: 19,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "apb2dis",
                    description: Some(
                        "APB2 clock disable\r This bit can be set in order to further reduce power consumption, when none of the APB2 peripherals are used and when their clocks are disabled in RCC_APB2ENR. When this bit is set, all the APB2 peripherals clocks are off.",
                    ),
                    bit_offset: 20,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cier",
            extends: None,
            description: Some(
                "RCC clock interrupt enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsirdyie",
                    description: Some(
                        "LSI ready interrupt enable\r Set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lserdyie",
                    description: Some(
                        "LSE ready interrupt enable\r Set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization.",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "msisrdyie",
                    description: Some(
                        "MSIS ready interrupt enable\r Set and cleared by software to enable/disable interrupt caused by the MSIS oscillator stabilization.",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsirdyie",
                    description: Some(
                        "HSI16 ready interrupt enable\r Set and cleared by software to enable/disable interrupt caused by the HSI16 oscillator stabilization.",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hserdyie",
                    description: Some(
                        "HSE ready interrupt enable\r Set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization.",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsi48rdyie",
                    description: Some(
                        "HSI48 ready interrupt enable\r Set and cleared by software to enable/disable interrupt caused by the HSI48 oscillator stabilization.",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllrdyie",
                    description: Some(
                        "PLL ready interrupt enable\r Set and cleared by software to enable/disable interrupt caused by PLL1 lock.",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "msikrdyie",
                    description: Some(
                        "MSIK ready interrupt enable\r Set and cleared by software to enable/disable interrupt caused by the MSIK oscillator stabilization.",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "shsirdyie",
                    description: Some(
                        "SHSI ready interrupt enable\r Set and cleared by software to enable/disable interrupt caused by the SHSI oscillator stabilization.",
                    ),
                    bit_offset: 12,
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
                "RCC AHB3 peripheral reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lpgpio1rst",
                    description: Some(
                        "LPGPIO1 reset\r Set and cleared by software.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adc4rst",
                    description: Some(
                        "ADC4 reset\r Set and cleared by software.",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dac1rst",
                    description: Some(
                        "DAC1 reset\r Set and cleared by software.",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpdma1rst",
                    description: Some(
                        "LPDMA1 reset\r Set and cleared by software.",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adf1rst",
                    description: Some(
                        "ADF1 reset\r Set and cleared by software.",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb1smenr2",
            extends: None,
            description: Some(
                "RCC APB1 peripheral clocks enable in Sleep and\tStop modes register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "i2c4smen",
                    description: Some(
                        "I2C4 clocks enable during Sleep and Stop modes\r Set and cleared by software\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim2smen",
                    description: Some(
                        "LPTIM2 clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c5smen",
                    description: Some(
                        "I2C5 clock enable during Sleep and Stop modes\r This bit is set and cleared by software\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.\r Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c6smen",
                    description: Some(
                        "I2C6 clock enable during Sleep and Stop modes\r This bit is set and cleared by software\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.\r Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fdcan1smen",
                    description: Some(
                        "FDCAN1 clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ucpd1smen",
                    description: Some(
                        "UCPD1 clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cfgr1",
            extends: None,
            description: Some(
                "RCC clock configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sw",
                    description: Some(
                        "system clock switch\r Set and cleared by software to select system clock source (SYSCLK).\r Configured by hardware to force MSIS oscillator selection when exiting Standby or Shutdown mode. Configured by hardware to force MSIS or HSI16 oscillator selection when exiting Stop mode or in case of HSE oscillator failure, depending on STOPWUCK value.",
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
                        "system clock switch status\r Set and cleared by hardware to indicate which clock source is used as system clock.",
                    ),
                    bit_offset: 2,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Sw",
                    ),
                },
                Field {
                    name: "stopwuck",
                    description: Some(
                        "wakeup from Stop and CSS backup clock selection\r Set and cleared by software to select the system clock used when exiting Stop mode.\r The selected clock is also used as emergency clock for the clock security system on HSE. Warning: STOPWUCK must not be modified when the CSS is enabled by HSECSSON bit in RCC_CR and the system clock is HSE (SWS = 10) or a switch on HSE is requested (SW\u{a0}=\u{a0}10).",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Stopwuck",
                    ),
                },
                Field {
                    name: "stopkerwuck",
                    description: Some(
                        "wakeup from Stop kernel clock automatic enable selection\r Set and cleared by software to enable automatically another oscillator when exiting Stop mode. This oscillator can be used as independent kernel clock by peripherals.",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Stopkerwuck",
                    ),
                },
                Field {
                    name: "mcosel",
                    description: Some(
                        "microcontroller clock output\r Set and cleared by software.\r Others: reserved\r Note: This clock output may have some truncated cycles at startup or during MCO clock source switching.",
                    ),
                    bit_offset: 24,
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Mcosel",
                    ),
                },
                Field {
                    name: "mcopre",
                    description: Some(
                        "microcontroller clock output prescaler\r Set and cleared by software.\r It is highly recommended to change this prescaler before MCO output is enabled.\r Others: not allowed",
                    ),
                    bit_offset: 28,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Mcopre",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Pll1fracr",
            extends: None,
            description: Some(
                "RCC PLL1 fractional divider register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pllfracn",
                    description: Some(
                        "Fractional part of the multiplication factor for PLL1 VCO\r Set and reset by software to control the fractional part of the multiplication factor of the VCO.\r These bits can be written at any time, allowing dynamic fine-tuning of the PLL1 VCO.\r VCO output frequency = Fref1_ck x (PLL1N + (PLL1FRACN / 213)), with:\r PLL1N must be between 4 and 512.\r PLL1FRACN can be between 0 and 213- 1.\r The input frequency Fref1_ck must be between 4 and 16 MHz.\r To change the FRACN value on-the-fly even if the PLL is enabled, the application must proceed as follows:\r Set the bit PLL1FRACEN to 0.\r Write the new fractional value into PLL1FRACN.\r Set the bit PLL1FRACEN to 1.",
                    ),
                    bit_offset: 3,
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb3smenr",
            extends: None,
            description: Some(
                "RCC APB3 peripheral clock enable in Sleep and Stop modes register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "syscfgsmen",
                    description: Some(
                        "SYSCFG clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi3smen",
                    description: Some(
                        "SPI3 clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpuart1smen",
                    description: Some(
                        "LPUART1 clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c3smen",
                    description: Some(
                        "I2C3 clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim1smen",
                    description: Some(
                        "LPTIM1 clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim3smen",
                    description: Some(
                        "LPTIM3 clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim4smen",
                    description: Some(
                        "LPTIM4 clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "opampsmen",
                    description: Some(
                        "OPAMP clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "compsmen",
                    description: Some(
                        "COMP clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vrefsmen",
                    description: Some(
                        "VREFBUF clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 20,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rtcapbsmen",
                    description: Some(
                        "RTC and TAMP APB clock enable during Sleep and Stop modes\r Set and cleared by software.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
                    ),
                    bit_offset: 21,
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
                "RCC APB2 peripheral clock enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim1en",
                    description: Some(
                        "TIM1 clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi1en",
                    description: Some(
                        "SPI1 clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim8en",
                    description: Some(
                        "TIM8 clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart1en",
                    description: Some(
                        "USART1clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim15en",
                    description: Some(
                        "TIM15 clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim16en",
                    description: Some(
                        "TIM16 clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim17en",
                    description: Some(
                        "TIM17 clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sai1en",
                    description: Some(
                        "SAI1 clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sai2en",
                    description: Some(
                        "SAI2 clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 22,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usben",
                    description: Some(
                        "USB clock enable\r This bit is set and cleared by software.\r Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gfxtimen",
                    description: Some(
                        "GFXTIM clock enable\r This bit is set and cleared by software.\r Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.",
                    ),
                    bit_offset: 25,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ltdcen",
                    description: Some(
                        "LTDC clock enable\r This bit is set and cleared by software.\r Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.",
                    ),
                    bit_offset: 26,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dsien",
                    description: Some(
                        "DSI clock enable\r This bit is set and cleared by software.\r Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.",
                    ),
                    bit_offset: 27,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pll1cfgr",
            extends: None,
            description: Some(
                "RCC PLL1 configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pllsrc",
                    description: Some(
                        "PLL1 entry clock source\r Set and cleared by software to select PLL1 clock source. These bits can be written only when the PLL1 is disabled.\r In order to save power, when no PLL1 is used, the value of PLL1SRC must be 0.",
                    ),
                    bit_offset: 0,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Pllsrc",
                    ),
                },
                Field {
                    name: "pllrge",
                    description: Some(
                        "PLL1 input frequency range\r Set and reset by software to select the proper reference frequency range used for PLL1.\r This bit must be written before enabling the PLL1.\r 00-01-10: PLL1 input (ref1_ck) clock range frequency between 4 and 8 MHz",
                    ),
                    bit_offset: 2,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Pllrge",
                    ),
                },
                Field {
                    name: "pllfracen",
                    description: Some(
                        "PLL1 fractional latch enable\r Set and reset by software to latch the content of PLL1FRACN into the Î£Î modulator.\r In order to latch the PLL1FRACN value into the Î£Î modulator, PLL1FRACEN must be set\u{a0}to\u{a0}0, then set to 1: the transition 0 to 1 transfers the content of PLL1FRACN into the modulator (see for details).",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllm",
                    description: Some(
                        "Prescaler for PLL1\r Set and cleared by software to configure the prescaler of the PLL1. The VCO1 input frequency is PLL1 input clock frequency/PLL1M.\r This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).\r ...",
                    ),
                    bit_offset: 8,
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Pllm",
                    ),
                },
                Field {
                    name: "pllmboost",
                    description: Some(
                        "Prescaler for EPOD booster input clock\r Set and cleared by software to configure the prescaler of the PLL1, used for the EPOD booster. The EPOD booster input frequency is PLL1 input clock frequency/PLL1MBOOST.\r This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0) and EPOD Boost mode is disabled (see ).\r others: reserved",
                    ),
                    bit_offset: 12,
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Pllmboost",
                    ),
                },
                Field {
                    name: "pllpen",
                    description: Some(
                        "PLL1 DIVP divider output enable\r Set and reset by software to enable the pll1_p_ck output of the PLL1.\r To save power, PLL1PEN and PLL1P bits must be set to 0 when the pll1_p_ck is not used.\r This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllqen",
                    description: Some(
                        "PLL1 DIVQ divider output enable\r Set and reset by software to enable the pll1_q_ck output of the PLL1.\r To save power, PLL1QEN and PLL1Q bits must be set to 0 when the pll1_q_ck is not used.\r This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllren",
                    description: Some(
                        "PLL1 DIVR divider output enable\r Set and reset by software to enable the pll1_r_ck output of the PLL1.\r To save power, PLL1RENPLL2REN and PLL1R bits must be set to 0 when the pll1_r_ck is not used.\r This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cicr",
            extends: None,
            description: Some(
                "RCC clock interrupt clear register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lsirdyc",
                    description: Some(
                        "LSI ready interrupt clear\r Writing this bit to 1 clears the LSIRDYF flag. Writing 0 has no effect.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lserdyc",
                    description: Some(
                        "LSE ready interrupt clear\r Writing this bit to 1 clears the LSERDYF flag. Writing 0 has no effect.",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "msisrdyc",
                    description: Some(
                        "MSIS ready interrupt clear\r Writing this bit to 1 clears the MSISRDYF flag. Writing 0 has no effect.",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsirdyc",
                    description: Some(
                        "HSI16 ready interrupt clear\r Writing this bit to 1 clears the HSIRDYF flag. Writing 0 has no effect.",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hserdyc",
                    description: Some(
                        "HSE ready interrupt clear\r Writing this bit to 1 clears the HSERDYF flag. Writing 0 has no effect.",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsi48rdyc",
                    description: Some(
                        "HSI48 ready interrupt clear\r Writing this bit to 1 clears the HSI48RDYF flag. Writing 0 has no effect.",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllrdyc",
                    description: Some(
                        "PLL1 ready interrupt clear\r Writing this bit to 1 clears the PLL1RDYF flag. Writing 0 has no effect.",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "cssc",
                    description: Some(
                        "Clock security system interrupt clear\r Writing this bit to 1 clears the CSSF flag. Writing 0 has no effect.",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "msikrdyc",
                    description: Some(
                        "MSIK oscillator ready interrupt clear\r Writing this bit to 1 clears the MSIKRDYF flag. Writing 0 has no effect.",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "shsirdyc",
                    description: Some(
                        "SHSI oscillator ready interrupt clear\r Writing this bit to 1 clears the SHSIRDYF flag. Writing 0 has no effect.",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Srdamr",
            extends: None,
            description: Some(
                "RCC SmartRun domain peripheral autonomous mode register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "spi3amen",
                    description: Some(
                        "SPI3 autonomous mode enable in Stop 0,1, 2 mode\r Set and cleared by software.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpuart1amen",
                    description: Some(
                        "LPUART1 autonomous mode enable in Stop 0,1, 2 mode\r Set and cleared by software.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c3amen",
                    description: Some(
                        "I2C3 autonomous mode enable in Stop 0,1,2 mode\r Set and cleared by software.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim1amen",
                    description: Some(
                        "LPTIM1 autonomous mode enable in Stop 0,1,2 mode\r Set and cleared by software.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim3amen",
                    description: Some(
                        "LPTIM3 autonomous mode enable in Stop 0,1,2 mode\r Set and cleared by software.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim4amen",
                    description: Some(
                        "LPTIM4 autonomous mode enable in Stop 0,1,2 mode\r Set and cleared by software.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "opampamen",
                    description: Some(
                        "OPAMP autonomous mode enable in Stop 0,1,2 mode\r Set and cleared by software.",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "compamen",
                    description: Some(
                        "COMP autonomous mode enable in Stop 0,1,2 mode\r Set and cleared by software.",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vrefamen",
                    description: Some(
                        "VREFBUF autonomous mode enable in Stop 0,1,2 mode\r Set and cleared by software.",
                    ),
                    bit_offset: 20,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rtcapbamen",
                    description: Some(
                        "RTC and TAMP autonomous mode enable in Stop 0,1,2 mode\r Set and cleared by software.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
                    ),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adc4amen",
                    description: Some(
                        "ADC4 autonomous mode enable in Stop 0,1,2 mode\r Set and cleared by software.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
                    ),
                    bit_offset: 25,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpgpio1amen",
                    description: Some(
                        "LPGPIO1 autonomous mode enable in Stop 0,1,2 mode\r Set and cleared by software.",
                    ),
                    bit_offset: 26,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dac1amen",
                    description: Some(
                        "DAC1 autonomous mode enable in Stop 0,1,2 mode\r Set and cleared by software.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
                    ),
                    bit_offset: 27,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpdma1amen",
                    description: Some(
                        "LPDMA1 autonomous mode enable in Stop 0,1,2 mode\r Set and cleared by software.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
                    ),
                    bit_offset: 28,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adf1amen",
                    description: Some(
                        "ADF1 autonomous mode enable in Stop 0,1,2 mode\r Set and cleared by software.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
                    ),
                    bit_offset: 29,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sram4amen",
                    description: Some(
                        "SRAM4 autonomous mode enable in Stop 0,1,2 mode\r Set and cleared by software.",
                    ),
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb1smenr1",
            extends: None,
            description: Some(
                "RCC APB1 peripheral clocks enable in Sleep and Stop modes\tregister 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2smen",
                    description: Some(
                        "TIM2 clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim3smen",
                    description: Some(
                        "TIM3 clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim4smen",
                    description: Some(
                        "TIM4 clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim5smen",
                    description: Some(
                        "TIM5 clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim6smen",
                    description: Some(
                        "TIM6 clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim7smen",
                    description: Some(
                        "TIM7 clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wwdgsmen",
                    description: Some(
                        "Window watchdog clocks enable during Sleep and Stop modes\r Set and cleared by software. This bit is forced to 1 by hardware when the hardware WWDG option is activated.",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi2smen",
                    description: Some(
                        "SPI2 clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart2smen",
                    description: Some(
                        "USART2 clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart3smen",
                    description: Some(
                        "USART3 clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uart4smen",
                    description: Some(
                        "UART4 clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
                    ),
                    bit_offset: 19,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uart5smen",
                    description: Some(
                        "UART5 clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
                    ),
                    bit_offset: 20,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c1smen",
                    description: Some(
                        "I2C1 clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
                    ),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c2smen",
                    description: Some(
                        "I2C2 clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
                    ),
                    bit_offset: 22,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "crssmen",
                    description: Some(
                        "CRS clock enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart6smen",
                    description: Some(
                        "USART6 clock enable during Sleep and Stop modes\r This bit is set and cleared by software.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.\r Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.",
                    ),
                    bit_offset: 25,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Icscr3",
            extends: None,
            description: Some(
                "RCC internal clock sources calibration register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsical",
                    description: Some(
                        "HSI clock calibration\r These bits are initialized at startup with the factory-programmed HSI calibration trim value. When HSITRIM is written, HSICAL is updated with the sum of HSITRIM and the factory trim value.",
                    ),
                    bit_offset: 0,
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsitrim",
                    description: Some(
                        "HSI clock trimming\r These bits provide an additional user-programmable trimming value that is added to the HSICAL[11:0] bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the HSI.",
                    ),
                    bit_offset: 16,
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Privcfgr",
            extends: None,
            description: Some(
                "RCC privilege configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "spriv",
                    description: Some(
                        "RCC secure functions privilege configuration\r Set and reset by software. This bit can be written only by a secure privileged access.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Priv",
                    ),
                },
                Field {
                    name: "nspriv",
                    description: Some(
                        "RCC non-secure functions privilege configuration\r Set and reset by software. This bit can be written only by privileged access, secure or non-secure.",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Priv",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Ahb2enr2",
            extends: None,
            description: Some(
                "RCC AHB2 peripheral clock enable register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fsmcen",
                    description: Some(
                        "FSMC clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "octospi1en",
                    description: Some(
                        "OCTOSPI1 clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "octospi2en",
                    description: Some(
                        "OCTOSPI2 clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hspi1en",
                    description: Some(
                        "HSPI1 clock enable\r This bit is set and cleared by software.\r Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sram6en",
                    description: Some(
                        "SRAM6 clock enable \r This bit is set and reset by software.\r Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.",
                    ),
                    bit_offset: 30,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sram5en",
                    description: Some(
                        "SRAM5 clock enable \r This bit is set and reset by software.\r Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.",
                    ),
                    bit_offset: 31,
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
                "RCC AHB1 peripheral reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpdma1rst",
                    description: Some(
                        "GPDMA1 reset\r Set and cleared by software.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cordicrst",
                    description: Some(
                        "CORDIC reset\r Set and cleared by software.",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fmacrst",
                    description: Some(
                        "FMAC reset\r Set and cleared by software.",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mdf1rst",
                    description: Some(
                        "MDF1 reset\r Set and cleared by software.",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "crcrst",
                    description: Some(
                        "CRC reset\r Set and cleared by software.",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jpegrst",
                    description: Some(
                        "JPEG reset\r This bit is set and cleared by software.\r Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscrst",
                    description: Some(
                        "TSC reset\r Set and cleared by software.",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ramcfgrst",
                    description: Some(
                        "RAMCFG reset\r Set and cleared by software.",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dma2drst",
                    description: Some(
                        "DMA2D reset\r Set and cleared by software.",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gfxmmurst",
                    description: Some(
                        "GFXMMU reset\r This bit is set and cleared by software.\r Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.",
                    ),
                    bit_offset: 19,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpu2drst",
                    description: Some(
                        "GPU2D reset\r This bit is set and cleared by software.\r Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.",
                    ),
                    bit_offset: 20,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cfgr3",
            extends: None,
            description: Some(
                "RCC clock configuration register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ppre3",
                    description: Some(
                        "APB3 prescaler\r Set and cleared by software to control the division factor of the APB3 clock (PCLK3).\r 0xx: HCLK not divided",
                    ),
                    bit_offset: 4,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Ppre",
                    ),
                },
                Field {
                    name: "ahb3dis",
                    description: Some(
                        "AHB3 clock disable\r This bit can be set in order to further reduce power consumption, when none of the AHB3 peripherals (except SRAM4) are used and when their clocks are disabled in RCC_AHB3ENR. When this bit is set, all the AHB3 peripherals clocks are off, except for SRAM4.",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "apb3dis",
                    description: Some(
                        "APB3 clock disable\r This bit can be set in order to further reduce power consumption, when none of the APB3 peripherals from RCC_APB3ENR are used and when their clocks are disabled in RCC_APB3ENR. When this bit is set, all the APB3 peripherals clocks are off.",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bdcr",
            extends: None,
            description: Some(
                "RCC Backup domain control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lseon",
                    description: Some(
                        "LSE oscillator enable\r Set and cleared by software.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lserdy",
                    description: Some(
                        "LSE oscillator ready\r Set and cleared by hardware to indicate when the external 32\u{a0}kHz oscillator is stable. After the LSEON bit is cleared, LSERDY goes low after six external low-speed oscillator clock cycles.",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lsebyp",
                    description: Some(
                        "LSE oscillator bypass\r Set and cleared by software to bypass oscillator in debug mode. This bit can be written only when the external 32\u{a0}kHz oscillator is disabled (LSEON = 0 and LSERDY = 0).",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lsedrv",
                    description: Some(
                        "LSE oscillator drive capability\r Set by software to modulate the drive capability of the LSE oscillator. This field can be written only when the external 32 kHz oscillator is disabled (LSEON = 0 and LSERDY = 0).\r Note: The oscillator is in 'Xtal mode when it is not in bypass mode.",
                    ),
                    bit_offset: 3,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Lsedrv",
                    ),
                },
                Field {
                    name: "lsecsson",
                    description: Some(
                        "CSS on LSE enable\r Set by software to enable the CSS on LSE. LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware), and after the RTCSEL bit is selected.\r Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD\u{a0}=\u{a0}1). In that case, the software must disable the LSECSSON bit.",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lsecssd",
                    description: Some(
                        "CSS on LSE failure Detection\r Set by hardware to indicate when a failure is detected by the CCS on the external 32\u{a0}kHz oscillator (LSE).",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lsesysen",
                    description: Some(
                        "LSE system clock (LSESYS) enable\r Set by software to enable always the LSE system clock generated by RCC. This clock can be used by any peripheral when its source clock is the LSE or at system level in case of one of the LSCOSEL, MCO, MSI PLL mode or CSS on LSE is needed.\r The LSESYS clock can be generated even if LSESYSEN= 0 if the LSE clock is requested by the CSS on LSE, by a peripheral or any other source clock using LSE.",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rtcsel",
                    description: Some(
                        "RTC and TAMP clock source selection\r Set by software to select the clock source for the RTC and TAMP . Once the RTC and TAMP clock source has been selected, it cannot be changed anymore unless the Backup domain is reset, or unless a failure is detected on LSE (LSECSSD is set). The BDRST bit can be used to reset them.",
                    ),
                    bit_offset: 8,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Rtcsel",
                    ),
                },
                Field {
                    name: "lsesysrdy",
                    description: Some(
                        "LSE system clock (LSESYS) ready\r Set and cleared by hardware to indicate when the LSE system clock is stable.When the LSESYSEN bit is set, the LSESYSRDY flag is set after two LSE clock cycles.\r The LSE clock must be already enabled and stable (LSEON and LSERDY are set).\r When the LSEON bit is cleared, LSERDY goes low after six external low-speed oscillator clock cycles.",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lsegfon",
                    description: Some(
                        "LSE clock glitch filter enable\r Set and cleared by hardware to enable the LSE glitch filter. This bit can be written only when the LSE is disabled (LSEON = 0 and LSERDY = 0)",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rtcen",
                    description: Some(
                        "RTC and TAMP clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bdrst",
                    description: Some(
                        "Backup domain software reset\r Set and cleared by software.",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lscoen",
                    description: Some(
                        "Low-speed clock output (LSCO) enable\r Set and cleared by software.",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lscosel",
                    description: Some(
                        "Low-speed clock output selection\r Set and cleared by software.",
                    ),
                    bit_offset: 25,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Lscosel",
                    ),
                },
                Field {
                    name: "lsion",
                    description: Some(
                        "LSI oscillator enable\r Set and cleared by software.",
                    ),
                    bit_offset: 26,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lsirdy",
                    description: Some(
                        "LSI oscillator ready\r Set and cleared by hardware to indicate when the LSI oscillator is stable. After the LSION bit is cleared, LSIRDY goes low after three internal low-speed oscillator clock cycles. This bit is set when the LSI is used by IWDG or RTC, even if LSION = 0.",
                    ),
                    bit_offset: 27,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lsiprediv",
                    description: Some(
                        "Low-speed clock divider configuration\r Set and cleared by software to enable the LSI division. This bit can be written only when the LSI is disabled (LSION = 0 and LSIRDY = 0). If the LSI was previously enabled, it is necessary to wait for at least 60 Î¼s after clearing LSION bit (synchronization time for LSI to be really disabled), before writing LSIPREDIV. The LSIPREDIV cannot be changed if the LSI is used by the IWDG or by the RTC.",
                    ),
                    bit_offset: 28,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Lsiprediv",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Csr",
            extends: None,
            description: Some(
                "RCC control/status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "msiksrange",
                    description: Some(
                        "MSIK range after Standby mode\r Set by software to chose the MSIK frequency at startup. This range is used after exiting Standby mode until MSIRGSEL is set. After a NRST pin or a power-on reset or when exiting Shutdown mode, the range is always 4\u{a0}MHz. MSIKSRANGE can be written only when MSIRGSEL = 1.\r others: reserved\r Note: Changing the MSIKSRANGE does not change the current MSIK frequency.",
                    ),
                    bit_offset: 8,
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Msixsrange",
                    ),
                },
                Field {
                    name: "msissrange",
                    description: Some(
                        "MSIS range after Standby mode\r Set by software to chose the MSIS frequency at startup. This range is used after exiting Standby mode until MSIRGSEL is set. After a NRST pin or a power-on reset or when exiting Shutdown mode, the range is always 4\u{a0}MHz. MSISSRANGE can be written only when MSIRGSEL = 1.\r others: reserved\r Note: Changing the MSISSRANGE does not change the current MSIS frequency.",
                    ),
                    bit_offset: 12,
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Msixsrange",
                    ),
                },
                Field {
                    name: "rmvf",
                    description: Some(
                        "Remove reset flag\r Set by software to clear the reset flags.",
                    ),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "oblrstf",
                    description: Some(
                        "Option byte loader reset flag\r Set by hardware when a reset from the option byte loading occurs.\r Cleared by writing to the RMVF bit.",
                    ),
                    bit_offset: 25,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pinrstf",
                    description: Some(
                        "NRST pin reset flag\r Set by hardware when a reset from the NRST pin occurs.\r Cleared by writing to the RMVF bit.",
                    ),
                    bit_offset: 26,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "borrstf",
                    description: Some(
                        "BOR flag\r Set by hardware when a BOR occurs.\r Cleared by writing to the RMVF bit.",
                    ),
                    bit_offset: 27,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sftrstf",
                    description: Some(
                        "Software reset flag\r Set by hardware when a software reset occurs.\r Cleared by writing to the RMVF bit.",
                    ),
                    bit_offset: 28,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "iwdgrstf",
                    description: Some(
                        "Independent watchdog reset flag\r Set by hardware when an independent watchdog reset domain occurs.\r Cleared by writing to the RMVF bit.",
                    ),
                    bit_offset: 29,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wwdgrstf",
                    description: Some(
                        "Window watchdog reset flag\r Set by hardware when a window watchdog reset occurs.\r Cleared by writing to the RMVF bit.",
                    ),
                    bit_offset: 30,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpwrrstf",
                    description: Some(
                        "Low-power reset flag\r Set by hardware when a reset occurs due to Stop, Standby or Shutdown mode entry, whereas the corresponding nRST_STOP, nRST_STBY or nRST_SHDW option bit is cleared.\r Cleared by writing to the RMVF bit.",
                    ),
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ahb1enr",
            extends: None,
            description: Some(
                "RCC AHB1 peripheral clock enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpdma1en",
                    description: Some(
                        "GPDMA1 clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cordicen",
                    description: Some(
                        "CORDIC clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fmacen",
                    description: Some(
                        "FMAC clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mdf1en",
                    description: Some(
                        "MDF1 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "flashen",
                    description: Some(
                        "FLASH clock enable\r Set and cleared by software. This bit can be disabled only when the Flash memory is in power down mode.",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "crcen",
                    description: Some(
                        "CRC clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jpegen",
                    description: Some(
                        "JPEG clock enable\r This bit is set and cleared by software.\r Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscen",
                    description: Some(
                        "Touch sensing controller clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ramcfgen",
                    description: Some(
                        "RAMCFG clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dma2den",
                    description: Some(
                        "DMA2D clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gfxmmuen",
                    description: Some(
                        "GFXMMU clock enable\r This bit is set and cleared by software.\r Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.",
                    ),
                    bit_offset: 19,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpu2den",
                    description: Some(
                        "GPU2D clock enable\r This bit is set and cleared by software.\r Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.",
                    ),
                    bit_offset: 20,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dcache2en",
                    description: Some(
                        "DCACHE2 clock enable \r This bit is set and reset by software.\r Note: DCACHE2 clock must be enabled to access memories, even if the DCACHE2 is bypassed.\r Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.",
                    ),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gtzc1en",
                    description: Some(
                        "GTZC1 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bkpsramen",
                    description: Some(
                        "BKPSRAM clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 28,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dcache1en",
                    description: Some(
                        "DCACHE1 clock enable\r Set and reset by software.\r Note: DCACHE1 clock must be enabled when external memories are accessed through OCTOSPI1, OCTOSPI2 or FSMC, even if the DCACHE1 is bypassed.",
                    ),
                    bit_offset: 30,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sram1en",
                    description: Some(
                        "SRAM1 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb2smenr",
            extends: None,
            description: Some(
                "RCC APB2 peripheral clocks enable in Sleep and Stop modes register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim1smen",
                    description: Some(
                        "TIM1 clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi1smen",
                    description: Some(
                        "SPI1 clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim8smen",
                    description: Some(
                        "TIM8 clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart1smen",
                    description: Some(
                        "USART1clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim15smen",
                    description: Some(
                        "TIM15 clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim16smen",
                    description: Some(
                        "TIM16 clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim17smen",
                    description: Some(
                        "TIM17 clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sai1smen",
                    description: Some(
                        "SAI1 clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sai2smen",
                    description: Some(
                        "SAI2 clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 22,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usbsmen",
                    description: Some(
                        "USB clock enable during Sleep and Stop modes\r This bit is set and cleared by software.\r Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gfxtimsmen",
                    description: Some(
                        "GFXTIM clock enable during Sleep and Stop modes\r This bit is set and cleared by software.\r Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.",
                    ),
                    bit_offset: 25,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ltdcsmen",
                    description: Some(
                        "LTDC clock enable during Sleep and Stop modes\r This bit is set and cleared by software.\r Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.",
                    ),
                    bit_offset: 26,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dsismen",
                    description: Some(
                        "DSI clock enable during Sleep and Stop modes\r This bit is set and cleared by software.\r Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.",
                    ),
                    bit_offset: 27,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Crrcr",
            extends: None,
            description: Some(
                "RCC clock recovery RC register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsi48cal",
                    description: Some(
                        "HSI48 clock calibration\r These bits are initialized at startup with the factory-programmed HSI48 calibration trim value.",
                    ),
                    bit_offset: 0,
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ahb3smenr",
            extends: None,
            description: Some(
                "RCC AHB3 peripheral clocks enable in Sleep and Stop modes register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lpgpio1smen",
                    description: Some(
                        "LPGPIO1 enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pwrsmen",
                    description: Some(
                        "PWR clock enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adc4smen",
                    description: Some(
                        "ADC4 clock enable during Sleep and Stop modes\r Set and cleared by software.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dac1smen",
                    description: Some(
                        "DAC1 clock enable during Sleep and Stop modes\r Set and cleared by software.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpdma1smen",
                    description: Some(
                        "LPDMA1 clock enable during Sleep and Stop modes\r Set and cleared by software.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adf1smen",
                    description: Some(
                        "ADF1 clock enable during Sleep and Stop modes\r Set and cleared by software.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gtzc2smen",
                    description: Some(
                        "GTZC2 clock enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sram4smen",
                    description: Some(
                        "SRAM4 clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pll1divr",
            extends: None,
            description: Some(
                "RCC PLL1 dividers register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "plln",
                    description: Some(
                        "Multiplication factor for PLL1 VCO\r Set and reset by software to control the multiplication factor of the VCO.\r These bits can be written only when the PLL is disabled (PLL1ON = 0 and PLL1RDY = 0).\r ...\r ...\r Others: reserved\r VCO output frequency = Fref1_ck x PLL1N, when fractional value 0 has been loaded into PLL1FRACN, with:\r PLL1N between 4 and 512\r input frequency Fref1_ck between 4 and 16\u{a0}MHz",
                    ),
                    bit_offset: 0,
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllp",
                    description: Some(
                        "PLL1 DIVP division factor\r Set and reset by software to control the frequency of the pll1_p_ck clock.\r These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).\r Note that odd division factors are not allowed.\r ...",
                    ),
                    bit_offset: 9,
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllq",
                    description: Some(
                        "PLL1 DIVQ division factor\r Set and reset by software to control the frequency of the pll1_q_ck clock.\r These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).\r ...",
                    ),
                    bit_offset: 16,
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllr",
                    description: Some(
                        "PLL1 DIVR division factor\r Set and reset by software to control the frequency of the pll1_r_ck clock.\r These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).\r ...",
                    ),
                    bit_offset: 24,
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ccipr1",
            extends: None,
            description: Some(
                "RCC peripherals independent clock configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "usart1sel",
                    description: Some(
                        "USART1 kernel clock source selection\r This bits are used to select the USART1 kernel clock source.\r Note: The USART1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or LSE.",
                    ),
                    bit_offset: 0,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Usartsel",
                    ),
                },
                Field {
                    name: "usart2sel",
                    description: Some(
                        "USART2 kernel clock source selection\r This bits are used to select the USART2 kernel clock source.\r Note: The USART2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or LSE.",
                    ),
                    bit_offset: 2,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Usartsel",
                    ),
                },
                Field {
                    name: "usart3sel",
                    description: Some(
                        "USART3 kernel clock source selection\r This bits are used to select the USART3 kernel clock source.\r Note: The USART3 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or LSE.",
                    ),
                    bit_offset: 4,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Usartsel",
                    ),
                },
                Field {
                    name: "uart4sel",
                    description: Some(
                        "UART4 kernel clock source selection\r This bits are used to select the UART4 kernel clock source.\r Note: The UART4 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or LSE.",
                    ),
                    bit_offset: 6,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Uartsel",
                    ),
                },
                Field {
                    name: "uart5sel",
                    description: Some(
                        "UART5 kernel clock source selection\r These bits are used to select the UART5 kernel clock source.\r Note: The UART5 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or LSE.",
                    ),
                    bit_offset: 8,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Uartsel",
                    ),
                },
                Field {
                    name: "i2c1sel",
                    description: Some(
                        "I2C1 kernel clock source selection\r These bits are used to select the I2C1 kernel clock source.\r Note: The I2C1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or MSIK.",
                    ),
                    bit_offset: 10,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Icsel",
                    ),
                },
                Field {
                    name: "i2c2sel",
                    description: Some(
                        "I2C2 kernel clock source selection\r These bits are used to select the I2C2 kernel clock source.\r Note: The I2C2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or MSIK.",
                    ),
                    bit_offset: 12,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Icsel",
                    ),
                },
                Field {
                    name: "i2c4sel",
                    description: Some(
                        "I2C4 kernel clock source selection\r These bits are used to select the I2C4 kernel clock source.\r Note: The I2C4 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or MSIK.",
                    ),
                    bit_offset: 14,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Icsel",
                    ),
                },
                Field {
                    name: "spi2sel",
                    description: Some(
                        "SPI2 kernel clock source selection\r These bits are used to select the SPI2 kernel clock source.\r Note: The SPI2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or MSIK.",
                    ),
                    bit_offset: 16,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Spisel",
                    ),
                },
                Field {
                    name: "lptim2sel",
                    description: Some(
                        "Low-power timer 2 kernel clock source selection\r These bits are used to select the LPTIM2 kernel clock source.\r Note: The LPTIM2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is LSI, LSE or HSI16 if HSIKERON = 1.",
                    ),
                    bit_offset: 18,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Lptimsel",
                    ),
                },
                Field {
                    name: "spi1sel",
                    description: Some(
                        "SPI1 kernel clock source selection\r These bits are used to select the SPI1 kernel clock source.\r Note: The SPI1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or MSIK.",
                    ),
                    bit_offset: 20,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Spisel",
                    ),
                },
                Field {
                    name: "systicksel",
                    description: Some(
                        "SysTick clock source selection\r These bits are used to select the SysTick clock source.\r Note: When LSE or LSI is selected, the AHB frequency must be at least four times higher than the LSI or LSE frequency. In addition, a jitter up to one HCLK cycle is introduced, due to the LSE or LSI sampling with HCLK in the SysTick circuitry.",
                    ),
                    bit_offset: 22,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Systicksel",
                    ),
                },
                Field {
                    name: "fdcan1sel",
                    description: Some(
                        "FDCAN1 kernel clock source selection\r These bits are used to select the FDCAN1 kernel clock source.",
                    ),
                    bit_offset: 24,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Fdcansel",
                    ),
                },
                Field {
                    name: "iclksel",
                    description: Some(
                        "intermediate clock source selection\r These bits are used to select the clock source used by OTG_FS and SDMMC.",
                    ),
                    bit_offset: 26,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Iclksel",
                    ),
                },
                Field {
                    name: "timicsel",
                    description: Some(
                        "Clocks sources for TIM16,TIM17 and LPTIM2 internal input capture\r When the TIMICSEL2 bit is set, the TIM16, TIM17 and LPTIM2 internal input capture can be connected either to HSI/256, MSI/4 or MSI/1024. Depending on TIMICSEL[1:0] value, MSI is either MSIK or MSIS.\r When TIMICSEL2 is cleared, the HSI, MSIK and MSIS clock sources cannot be selected as TIM16, TIM17 or LPTIM2 internal input capture.\r 0xx: HSI, MSIK and MSIS dividers disabled\r Note: The clock division must be disabled (TIMICSEL configured to 0xx) before selecting or changing a clock sources division.",
                    ),
                    bit_offset: 29,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Timicsel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Ahb2smenr1",
            extends: None,
            description: Some(
                "RCC AHB2 peripheral clocks enable in Sleep and\tStop modes register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpioasmen",
                    description: Some(
                        "IO port A clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiobsmen",
                    description: Some(
                        "IO port B clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiocsmen",
                    description: Some(
                        "IO port C clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiodsmen",
                    description: Some(
                        "IO port D clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpioesmen",
                    description: Some(
                        "IO port E clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiofsmen",
                    description: Some(
                        "IO port F clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiogsmen",
                    description: Some(
                        "IO port G clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiohsmen",
                    description: Some(
                        "IO port H clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpioismen",
                    description: Some(
                        "IO port I clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiojsmen",
                    description: Some(
                        "I/O port J clock enable during Sleep and Stop modes\r This bit is set and cleared by software.\r Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adc12smen",
                    description: Some(
                        "ADC1 and ADC2 clock enable during Sleep and Stop modes\r This bit is set and cleared by software.\r Note: This bit impacts ADC1 in STM32U535/545/575/585 and ADC1/ADC2 in�STM32U59x/5Ax/5Fx/5Gx.",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dcmismen",
                    description: Some(
                        "DCMI and PSSI clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usb_otg_fssmen",
                    description: Some(
                        "OTG_FS clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usb_otg_hssmen",
                    description: Some(
                        "OTG_HS clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usb_otg_hs_physmen",
                    description: Some(
                        "OTG_HS PHY clock enable during Sleep and Stop modes\r This bit is set and cleared by software\r Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aessmen",
                    description: Some(
                        "AES clock enable during Sleep and Stop modes\r Set and cleared by software",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hashsmen",
                    description: Some(
                        "HASH clock enable during Sleep and Stop modes\r Set and cleared by software",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rngsmen",
                    description: Some(
                        "Random number generator (RNG) clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pkasmen",
                    description: Some(
                        "PKA clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 19,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "saessmen",
                    description: Some(
                        "SAES accelerator clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 20,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "octospimsmen",
                    description: Some(
                        "OCTOSPIM clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "otfdec1smen",
                    description: Some(
                        "OTFDEC1 clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "otfdec2smen",
                    description: Some(
                        "OTFDEC2 clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sdmmc1smen",
                    description: Some(
                        "SDMMC1 clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 27,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sdmmc2smen",
                    description: Some(
                        "SDMMC2 clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 28,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sram2smen",
                    description: Some(
                        "SRAM2 clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 30,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sram3smen",
                    description: Some(
                        "SRAM3 clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ahb2rstr2",
            extends: None,
            description: Some(
                "RCC AHB2 peripheral reset register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fsmcrst",
                    description: Some(
                        "Flexible memory controller reset\r Set and cleared by software.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "octospi1rst",
                    description: Some(
                        "OCTOSPI1 reset\r Set and cleared by software.",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "octospi2rst",
                    description: Some(
                        "OCTOSPI2 reset\r Set and cleared by software.",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hspi1rst",
                    description: Some(
                        "HSPI1 reset\r This bit is set and cleared by software.\r Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pll3cfgr",
            extends: None,
            description: Some(
                "RCC PLL3 configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pllsrc",
                    description: Some(
                        "PLL3 entry clock source\r Set and cleared by software to select PLL3 clock source. These bits can be written only when the PLL3 is disabled.\r In order to save power, when no PLL3 is used, the value of PLL3SRC must be 00.",
                    ),
                    bit_offset: 0,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Pllsrc",
                    ),
                },
                Field {
                    name: "pllrge",
                    description: Some(
                        "PLL3 input frequency range\r Set and reset by software to select the proper reference frequency range used for PLL3.\r This bit must be written before enabling the PLL3.\r 00-01-10: PLL3 input (ref3_ck) clock range frequency between 4 and 8 MHz",
                    ),
                    bit_offset: 2,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Pllrge",
                    ),
                },
                Field {
                    name: "pllfracen",
                    description: Some(
                        "PLL3 fractional latch enable\r Set and reset by software to latch the content of PLL3FRACN into the Î£Î modulator.\r In order to latch the PLL3FRACN value into the Î£Î modulator, PLL3FRACEN must be set\u{a0}to\u{a0}0, then set to 1: the transition 0 to 1 transfers the content of PLL3FRACN into the modulator (see for details).",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllm",
                    description: Some(
                        "Prescaler for PLL3\r Set and cleared by software to configure the prescaler of the PLL3. The VCO3 input frequency is PLL3 input clock frequency/PLL3M.\r This bit can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0).\r ...",
                    ),
                    bit_offset: 8,
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Pllm",
                    ),
                },
                Field {
                    name: "pllpen",
                    description: Some(
                        "PLL3 DIVP divider output enable\r Set and reset by software to enable the pll3_p_ck output of the PLL3.\r To save power, PLL3PEN and PLL3P bits must be set to 0 when the pll3_p_ck is not used.\r This bit can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0).",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllqen",
                    description: Some(
                        "PLL3 DIVQ divider output enable\r Set and reset by software to enable the pll3_q_ck output of the PLL3.\r To save power, PLL3QEN and PLL3Q bits must be set to 0 when the pll3_q_ck is not used.\r This bit can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0).",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllren",
                    description: Some(
                        "PLL3 DIVR divider output enable\r Set and reset by software to enable the pll3_r_ck output of the PLL3.\r To save power, PLL3REN and PLL3R bits must be set to 0 when the pll3_r_ck is not used.\r This bit can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0).",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb3enr",
            extends: None,
            description: Some(
                "RCC APB3 peripheral clock enable register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "syscfgen",
                    description: Some(
                        "SYSCFG clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi3en",
                    description: Some(
                        "SPI3 clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpuart1en",
                    description: Some(
                        "LPUART1 clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c3en",
                    description: Some(
                        "I2C3 clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim1en",
                    description: Some(
                        "LPTIM1 clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim3en",
                    description: Some(
                        "LPTIM3 clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim4en",
                    description: Some(
                        "LPTIM4 clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "opampen",
                    description: Some(
                        "OPAMP clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "compen",
                    description: Some(
                        "COMP clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vrefen",
                    description: Some(
                        "VREFBUF clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 20,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rtcapben",
                    description: Some(
                        "RTC and TAMP APB clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb3rstr",
            extends: None,
            description: Some(
                "RCC APB3 peripheral reset register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "syscfgrst",
                    description: Some(
                        "SYSCFG reset\r Set and cleared by software.",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi3rst",
                    description: Some(
                        "SPI3 reset\r Set and cleared by software.",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpuart1rst",
                    description: Some(
                        "LPUART1 reset\r Set and cleared by software.",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c3rst",
                    description: Some(
                        "I2C3 reset\r Set and cleared by software.",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim1rst",
                    description: Some(
                        "LPTIM1 reset\r Set and cleared by software.",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim3rst",
                    description: Some(
                        "LPTIM3 reset\r Set and cleared by software.",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim4rst",
                    description: Some(
                        "LPTIM4 reset\r Set and cleared by software.",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "opamprst",
                    description: Some(
                        "OPAMP reset\r Set and cleared by software.",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "comprst",
                    description: Some(
                        "COMP reset\r Set and cleared by software.",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vrefrst",
                    description: Some(
                        "VREFBUF reset\r Set and cleared by software.",
                    ),
                    bit_offset: 20,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ahb2enr1",
            extends: None,
            description: Some(
                "RCC AHB2 peripheral clock enable register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpioaen",
                    description: Some(
                        "IO port A clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpioben",
                    description: Some(
                        "IO port B clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiocen",
                    description: Some(
                        "IO port C clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpioden",
                    description: Some(
                        "IO port D clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpioeen",
                    description: Some(
                        "IO port E clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiofen",
                    description: Some(
                        "IO port F clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiogen",
                    description: Some(
                        "IO port G clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiohen",
                    description: Some(
                        "IO port H clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpioien",
                    description: Some(
                        "IO port I clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiojen",
                    description: Some(
                        "I/O port J clock enable\r This bit is set and cleared by software.\r Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adc12en",
                    description: Some(
                        "ADC1 and ADC2 clock enable\r This bit is set and cleared by software.\r Note: This bit impacts ADC1 in STM32U535/545/575/585, and ADC1/ADC2 in�STM32U59x/5Ax/5Fx/5Gx.",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dcmien",
                    description: Some(
                        "DCMI and PSSI clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usb_otg_fsen",
                    description: Some(
                        "OTG_FS clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usb_otg_hsen",
                    description: Some(
                        "OTG_HS clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usb_otg_hs_phyen",
                    description: Some(
                        "OTG_HS PHY clock enable\r This bit is set and cleared by software.\r Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aesen",
                    description: Some(
                        "AES clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hashen",
                    description: Some(
                        "HASH clock enable\r Set and cleared by software",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rngen",
                    description: Some(
                        "RNG clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pkaen",
                    description: Some(
                        "PKA clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 19,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "saesen",
                    description: Some(
                        "SAES clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 20,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "octospimen",
                    description: Some(
                        "OCTOSPIM clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "otfdec1en",
                    description: Some(
                        "OTFDEC1 clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "otfdec2en",
                    description: Some(
                        "OTFDEC2 clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sdmmc1en",
                    description: Some(
                        "SDMMC1 clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 27,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sdmmc2en",
                    description: Some(
                        "SDMMC2 clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 28,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sram2en",
                    description: Some(
                        "SRAM2 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 30,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sram3en",
                    description: Some(
                        "SRAM3 clock enable\r Set and reset by software.",
                    ),
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb1enr2",
            extends: None,
            description: Some(
                "RCC APB1 peripheral clock enable register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "i2c4en",
                    description: Some(
                        "I2C4 clock enable\r Set and cleared by software",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lptim2en",
                    description: Some(
                        "LPTIM2 clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c5en",
                    description: Some(
                        "I2C5 clock enable\r This bit is set and cleared by software.\r Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c6en",
                    description: Some(
                        "I2C6 clock enable\r This bit is set and cleared by software.\r Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fdcan1en",
                    description: Some(
                        "FDCAN1 clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ucpd1en",
                    description: Some(
                        "UCPD1 clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 23,
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
                "RCC clock control register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "msison",
                    description: Some(
                        "MSIS clock enable\r Set and cleared by software.\r Cleared by hardware to stop the MSIS oscillator when entering Stop, Standby or Shutdown mode.\r Set by hardware to force the MSIS oscillator ON when exiting Standby or Shutdown mode.\r Set by hardware to force the MSIS oscillator ON when STOPWUCK = 0 when exiting Stop modes or in case of a failure of the HSE oscillator.\r Set by hardware when used directly or indirectly as system clock.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "msikeron",
                    description: Some(
                        "MSI enable for some peripheral kernels\r Set and cleared by software to force MSI ON even in Stop modes. Keeping the MSI ON in Stop mode allows the communication speed not to be reduced by the MSI startup time. This bit has no effect on MSISON and MSIKON values (see autonomous mode for more details).\r The MSIKERON must be configured at 0 before entering Stop 3 mode.",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "msisrdy",
                    description: Some(
                        "MSIS clock ready flag\r Set by hardware to indicate that the MSIS oscillator is stable. This bit is set only when MSIS is enabled by software by setting MSISON.\r Note: Once the MSISON bit is cleared, MSISRDY goes low after six MSIS clock cycles.",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "msipllen",
                    description: Some(
                        "MSI clock PLL-mode enable\r Set and cleared by software to enable/disable the PLL part of the MSI clock source.\r MSIPLLEN must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set by hardware). A hardware protection prevents from enabling MSIPLLEN if LSE is not ready.\r This bit is cleared by hardware when LSE is disabled (LSEON = 0) or when the CSS on LSE detects a LSE failure (see RCC_CSR).",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "msikon",
                    description: Some(
                        "MSIK clock enable\r Set and cleared by software.\r Cleared by hardware to stop the MSIK when entering Stop, Standby or Shutdown mode.\r Set by hardware to force the MSIK oscillator ON when exiting Standby or Shutdown mode.\r Set by hardware to force the MSIK oscillator ON when STOPWUCK = 0 or STOPKERWUCK\u{a0}=\u{a0}0 when exiting Stop modes or in case of a failure of the HSE oscillator.",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "msikrdy",
                    description: Some(
                        "MSIK clock ready flag\r Set by hardware to indicate that the MSIK is stable. This bit is set only when MSI kernel oscillator is enabled by software by setting MSIKON.\r Note: Once the MSIKON bit is cleared, MSIKRDY goes low after six MSIK oscillator clock cycles.",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "msipllsel",
                    description: Some(
                        "MSI clock with PLL mode selection\r Set and cleared by software to select which MSI output clock uses the PLL mode. This bit can be written only when the MSI PLL mode is disabled (MSIPLLEN = 0).\r Note: If the MSI kernel clock output uses the same oscillator source than the MSI system clock output, then the PLL mode is applied to the both clocks outputs.",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Msipllsel",
                    ),
                },
                Field {
                    name: "msipllfast",
                    description: Some(
                        "MSI PLL mode fast startup\r Set and reset by software to enable/disable the fast PLL mode start-up of the MSI clock\r source. This bit is used only if PLL mode is selected (MSIPLLEN = 1).\r The fast start-up feature is not active the first time the PLL mode is selected. The fast start-up is active when the MSI in PLL mode returns from switch off.",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Msipllfast",
                    ),
                },
                Field {
                    name: "hsion",
                    description: Some(
                        "HSI16 clock enable\r Set and cleared by software.\r Cleared by hardware to stop the HSI16 oscillator when entering Stop, Standby or Shutdown mode.\r Set by hardware to force the HSI16 oscillator ON when STOPWUCK = 1 when leaving Stop modes, or in case of failure of the HSE crystal oscillator.\r This bit is set by hardware if the HSI16 is used directly or indirectly as system clock.",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsikeron",
                    description: Some(
                        "HSI16 enable for some peripheral kernels\r Set and cleared by software to force HSI16 ON even in Stop modes. Keeping the HSI16 ON in Stop mode allows the communication speed not to be reduced by the HSI16 startup time. This bit has no effect on HSION value.\r Refer to for more details.\r The HSIKERON must be configured at 0 before entering Stop 3 mode.",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsirdy",
                    description: Some(
                        "HSI16 clock ready flag\r Set by hardware to indicate that HSI16 oscillator is stable. This bit is set only when HSI16 is enabled by software by setting HSION.\r Note: Once the HSION bit is cleared, HSIRDY goes low after six HSI16 clock cycles.",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsi48on",
                    description: Some(
                        "HSI48 clock enable\r Set and cleared by software.\r Cleared by hardware to stop the HSI48 when entering in Stop, Standby or Shutdown modes.",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsi48rdy",
                    description: Some(
                        "HSI48 clock ready flag\r Set by hardware to indicate that HSI48 oscillator is stable. This bit is set only when HSI48 is enabled by software by setting HSI48ON.",
                    ),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "shsion",
                    description: Some(
                        "SHSI clock enable\r Set and cleared by software.\r Cleared by hardware to stop the SHSI when entering in Stop, Standby or Shutdown modes.",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "shsirdy",
                    description: Some(
                        "SHSI clock ready flag\r Set by hardware to indicate that the SHSI oscillator is stable. This bit is set only when SHSI is enabled by software by setting SHSION.\r Note: Once the SHSION bit is cleared, SHSIRDY goes low after six SHSI clock cycles.",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hseon",
                    description: Some(
                        "HSE clock enable\r Set and cleared by software.\r Cleared by hardware to stop the HSE oscillator when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the HSE oscillator is used directly or indirectly as the system clock.",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hserdy",
                    description: Some(
                        "HSE clock ready flag\r Set by hardware to indicate that the HSE oscillator is stable.\r Note: Once the HSEON bit is cleared, HSERDY goes low after six HSE clock cycles.",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsebyp",
                    description: Some(
                        "HSE crystal oscillator bypass\r Set and cleared by software to bypass the oscillator with an external clock. The external clock must be enabled with the HSEON bit set, to be used by the device. The HSEBYP bit can be written only if the HSE oscillator is disabled.",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "csson",
                    description: Some(
                        "Clock security system enable\r Set by software to enable the clock security system. When CSSON is set, the clock detector is enabled by hardware when the HSE oscillator is ready, and disabled by hardware if a HSE clock failure is detected. This bit is set only and is cleared by reset.",
                    ),
                    bit_offset: 19,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hseext",
                    description: Some(
                        "HSE external clock bypass mode\r Set and reset by software to select the external clock mode in bypass mode. External clock mode must be configured with HSEON bit to be used by the device. This bit can be written only if the HSE oscillator is disabled. This bit is active only if the HSE bypass mode is enabled.",
                    ),
                    bit_offset: 20,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Hseext",
                    ),
                },
                Field {
                    name: "pllon",
                    description: Some(
                        "PLL1 enable\r Set and cleared by software to enable the main PLL.\r Cleared by hardware when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the PLL1 clock is used as the system clock.",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 2,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "pllrdy",
                    description: Some(
                        "PLL1 clock ready flag\r Set by hardware to indicate that the PLL1 is locked.",
                    ),
                    bit_offset: 25,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 2,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pll2divr",
            extends: None,
            description: Some(
                "RCC PLL2 dividers configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "plln",
                    description: Some(
                        "Multiplication factor for PLL2 VCO\r Set and reset by software to control the multiplication factor of the VCO.\r These bits can be written only when the PLL is disabled (PLL2ON = 0 and PLL2RDY = 0).\r ...\r ...\r Others: reserved\r VCO output frequency = Fref2_ck x PLL2N, when fractional value 0 has been loaded into PLL2FRACN, with:\r PLL2N between 4 and 512\r input frequency Fref2_ck between 1MHz and 16MHz",
                    ),
                    bit_offset: 0,
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllp",
                    description: Some(
                        "PLL2 DIVP division factor\r Set and reset by software to control the frequency of the pll2_p_ck clock.\r These bits can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0).\r ...",
                    ),
                    bit_offset: 9,
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllq",
                    description: Some(
                        "PLL2 DIVQ division factor\r Set and reset by software to control the frequency of the pll2_q_ck clock.\r These bits can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0).\r ...",
                    ),
                    bit_offset: 16,
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllr",
                    description: Some(
                        "PLL2 DIVR division factor\r Set and reset by software to control the frequency of the pll2_r_ck clock.\r These bits can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0).\r ...",
                    ),
                    bit_offset: 24,
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Icscr1",
            extends: None,
            description: Some(
                "RCC internal clock sources calibration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "msical3",
                    description: Some(
                        "MSIRC3 clock calibration for MSI ranges 12 to 15\r These bits are initialized at startup with the factory-programmed MSIRC3 calibration trim value for ranges 12 to 15. When MSITRIM3 is written, MSICAL3 is updated with the sum of MSITRIM3[4:0] and the factory calibration trim value MSIRC2[4:0].\r There is no hardware protection to limit a potential overflow due to the addition of MSITRIM bitfield and factory program bitfield for this calibration value. Control must be managed by software at user level.",
                    ),
                    bit_offset: 0,
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "msical2",
                    description: Some(
                        "MSIRC2 clock calibration for MSI ranges 8 to 11\r These bits are initialized at startup with the factory-programmed MSIRC2 calibration trim value for ranges 8 to 11. When MSITRIM2 is written, MSICAL2 is updated with the sum of MSITRIM2[4:0] and the factory calibration trim value MSIRC2[4:0].\r There is no hardware protection to limit a potential overflow due to the addition of MSITRIM bitfield and factory program bitfield for this calibration value. Control must be managed by software at user level.",
                    ),
                    bit_offset: 5,
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "msical1",
                    description: Some(
                        "MSIRC1 clock calibration for MSI ranges 4 to 7\r These bits are initialized at startup with the factory-programmed MSIRC1 calibration trim value for ranges 4 to 7. When MSITRIM1 is written, MSICAL1 is updated with the sum of MSITRIM1[4:0] and the factory calibration trim value MSIRC1[4:0].\r There is no hardware protection to limit a potential overflow due to the addition of MSITRIM bitfield and factory program bitfield for this calibration value. Control must be managed by software at user level.",
                    ),
                    bit_offset: 10,
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "msical0",
                    description: Some(
                        "MSIRC0 clock calibration for MSI ranges 0 to 3\r These bits are initialized at startup with the factory-programmed MSIRC0 calibration trim value for ranges 0 to 3. When MSITRIM0 is written, MSICAL0 is updated with the sum of MSITRIM0[4:0] and the factory-programmed calibration trim value MSIRC0[4:0].",
                    ),
                    bit_offset: 15,
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "msibias",
                    description: Some(
                        "MSI bias mode selection\r Set by software to select the MSI bias mode. By default, the MSI bias is in continuous mode in order to maintain the output clocks accuracy. Setting this bit reduces the MSI consumption under range 4 but decrease its accuracy.",
                    ),
                    bit_offset: 22,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Msibias",
                    ),
                },
                Field {
                    name: "msirgsel",
                    description: Some(
                        "MSI clock range selection\r Set by software to select the MSIS and MSIK clocks range with MSISRANGE[3:0] and MSIKRANGE[3:0]. Write 0 has no effect.\r After exiting Standby or Shutdown mode, or after a reset, this bit is at 0 and the MSIS and MSIK ranges are provided by MSISSRANGE[3:0] and MSIKSRANGE[3:0] in RCC_CSR.",
                    ),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Msirgsel",
                    ),
                },
                Field {
                    name: "msikrange",
                    description: Some(
                        "MSIK clock ranges\r These bits are configured by software to choose the frequency range of MSIK oscillator when MSIRGSEL is set. 16 frequency ranges are available:\r Note: MSIKRANGE can be modified when MSIK is OFF (MSISON = 0) or when MSIK is ready (MSIKRDY\u{a0}=\u{a0}1). MSIKRANGE must NOT be modified when MSIK is ON and NOT ready (MSIKON = 1 and MSIKRDY = 0)\r MSIKRANGE is kept when the device wakes up from Stop mode, except when the MSIK range is above 24 MHz. In this case MSIKRANGE is changed by hardware into Range 2 (24 MHz).",
                    ),
                    bit_offset: 24,
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Msirange",
                    ),
                },
                Field {
                    name: "msisrange",
                    description: Some(
                        "MSIS clock ranges\r These bits are configured by software to choose the frequency range of MSIS oscillator when MSIRGSEL is set. 16 frequency ranges are available:\r Note: MSISRANGE can be modified when MSIS is OFF (MSISON = 0) or when MSIS is ready (MSISRDY\u{a0}=\u{a0}1). MSISRANGE must NOT be modified when MSIS is ON and NOT ready (MSISON\u{a0}=\u{a0}1 and MSISRDY\u{a0}=\u{a0}0)\r MSISRANGE is kept when the device wakes up from Stop mode, except when the MSIS range is above 24 MHz. In this case MSISRANGE is changed by hardware into Range 2 (24 MHz).",
                    ),
                    bit_offset: 28,
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Msirange",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Ccipr3",
            extends: None,
            description: Some(
                "RCC peripherals independent clock configuration register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lpuart1sel",
                    description: Some(
                        "LPUART1 kernel clock source selection\r These bits are used to select the LPUART1 kernel clock source.\r others: reserved\r Note: The LPUART1 is functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is HSI16, LSE or MSIK.",
                    ),
                    bit_offset: 0,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Lpuartsel",
                    ),
                },
                Field {
                    name: "spi3sel",
                    description: Some(
                        "SPI3 kernel clock source selection\r These bits are used to select the SPI3 kernel clock source.\r Note: The SPI3 is functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is HSI16 or MSIK.",
                    ),
                    bit_offset: 3,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Spisel",
                    ),
                },
                Field {
                    name: "i2c3sel",
                    description: Some(
                        "I2C3 kernel clock source selection\r These bits are used to select the I2C3 kernel clock source.\r Note: The I2C3 is functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is HSI16 or MSIK.",
                    ),
                    bit_offset: 6,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Icsel",
                    ),
                },
                Field {
                    name: "lptim34sel",
                    description: Some(
                        "LPTIM3 and LPTIM4 kernel clock source selection\r These bits are used to select the LPTIM3 and LPTIM4 kernel clock source.\r Note: The LPTIM3 and LPTIM4 are functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is LSI, LSE, HSI16 with HSIKERON = 1 or MSIK with MSIKERON\u{a0}=\u{a0}1.",
                    ),
                    bit_offset: 8,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Lptimsel",
                    ),
                },
                Field {
                    name: "lptim1sel",
                    description: Some(
                        "LPTIM1 kernel clock source selection\r These bits are used to select the LPTIM1 kernel clock source.\r Note: The LPTIM1 is functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is LSI, LSE, HSI16 with HSIKERON = 1 or MSIK with MSIKERON = 1.",
                    ),
                    bit_offset: 10,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Lptimsel",
                    ),
                },
                Field {
                    name: "adcdacsel",
                    description: Some(
                        "ADC1, ADC4 and DAC1 kernel clock source selection\r These bits are used to select the ADC1, ADC4 and DAC1 kernel clock source.\r others: reserved\r Note: The ADC1, ADC4 and DAC1 are functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is HSI16 or MSIK (only ADC4 and DAC1 are functional in Stop 2 mode).",
                    ),
                    bit_offset: 12,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Adcdacsel",
                    ),
                },
                Field {
                    name: "dac1sel",
                    description: Some(
                        "DAC1 sample and hold clock source selection\r This bit is used to select the DAC1 sample and hold clock source.",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Dacsel",
                    ),
                },
                Field {
                    name: "adf1sel",
                    description: Some(
                        "ADF1 kernel clock source selection\r These bits are used to select the ADF1 kernel clock source.\r others: reserved\r Note: The ADF1 is functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is AUDIOCLK or MSIK.",
                    ),
                    bit_offset: 16,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Adfsel",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Ahb1smenr",
            extends: None,
            description: Some(
                "RCC AHB1 peripheral clocks enable in Sleep and Stop modes register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpdma1smen",
                    description: Some(
                        "GPDMA1 clocks enable during Sleep and Stop modes\r Set and cleared by software.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cordicsmen",
                    description: Some(
                        "CORDIC clocks enable during Sleep and Stop modes\r Set and cleared by software during Sleep mode.",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fmacsmen",
                    description: Some(
                        "FMAC clocks enable during Sleep and Stop modes.\r Set and cleared by software.",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mdf1smen",
                    description: Some(
                        "MDF1 clocks enable during Sleep and Stop modes.\r Set and cleared by software.\r Note: This bit must be set to allow the peripheral to wake up from Stop modes.",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "flashsmen",
                    description: Some(
                        "FLASH clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "crcsmen",
                    description: Some(
                        "CRC clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jpegsmen",
                    description: Some(
                        "JPEG clocks enable during Sleep and Stop modes\r This bit is set and cleared by software.\r Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.",
                    ),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tscsmen",
                    description: Some(
                        "TSC clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ramcfgsmen",
                    description: Some(
                        "RAMCFG clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dma2dsmen",
                    description: Some(
                        "DMA2D clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gfxmmusmen",
                    description: Some(
                        "GFXMMU clock enable during Sleep and Stop modes\r This bit is set and cleared by software.\r Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.",
                    ),
                    bit_offset: 19,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpu2dsmen",
                    description: Some(
                        "GPU2D clock enable during Sleep and Stop modes\r This bit is set and cleared by software.\r Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.",
                    ),
                    bit_offset: 20,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dcache2smen",
                    description: Some(
                        "DCACHE2 clock enable during Sleep and Stop modes\r This bit is set and cleared by software.\r Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.",
                    ),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gtzc1smen",
                    description: Some(
                        "GTZC1 clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bkpsramsmen",
                    description: Some(
                        "BKPSRAM clocks enable during Sleep and Stop modes\r Set and cleared by software",
                    ),
                    bit_offset: 28,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "icachesmen",
                    description: Some(
                        "ICACHE clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 29,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dcache1smen",
                    description: Some(
                        "DCACHE1 clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 30,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sram1smen",
                    description: Some(
                        "SRAM1 clocks enable during Sleep and Stop modes\r Set and cleared by software.",
                    ),
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb1rstr1",
            extends: None,
            description: Some(
                "RCC APB1 peripheral reset register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2rst",
                    description: Some(
                        "TIM2 reset\r Set and cleared by software.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim3rst",
                    description: Some(
                        "TIM3 reset\r Set and cleared by software.",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim4rst",
                    description: Some(
                        "TIM4 reset\r Set and cleared by software.",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim5rst",
                    description: Some(
                        "TIM5 reset\r Set and cleared by software.",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim6rst",
                    description: Some(
                        "TIM6 reset\r Set and cleared by software.",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim7rst",
                    description: Some(
                        "TIM7 reset\r Set and cleared by software.",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi2rst",
                    description: Some(
                        "SPI2 reset\r Set and cleared by software.",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart2rst",
                    description: Some(
                        "USART2 reset\r Set and cleared by software.",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart3rst",
                    description: Some(
                        "USART3 reset\r Set and cleared by software.",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uart4rst",
                    description: Some(
                        "UART4 reset\r Set and cleared by software.",
                    ),
                    bit_offset: 19,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uart5rst",
                    description: Some(
                        "UART5 reset\r Set and cleared by software.",
                    ),
                    bit_offset: 20,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c1rst",
                    description: Some(
                        "I2C1 reset\r Set and cleared by software.",
                    ),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c2rst",
                    description: Some(
                        "I2C2 reset\r Set and cleared by software.",
                    ),
                    bit_offset: 22,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "crsrst",
                    description: Some(
                        "CRS reset\r Set and cleared by software.",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart6rst",
                    description: Some(
                        "USART6 reset\r This bit is set and cleared by software.\r Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.",
                    ),
                    bit_offset: 25,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pll2cfgr",
            extends: None,
            description: Some(
                "RCC PLL2 configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pllsrc",
                    description: Some(
                        "PLL2 entry clock source\r Set and cleared by software to select PLL2 clock source. These bits can be written only when the PLL2 is disabled.\r In order to save power, when no PLL2 is used, the value of PLL2SRC must be 0.",
                    ),
                    bit_offset: 0,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Pllsrc",
                    ),
                },
                Field {
                    name: "pllrge",
                    description: Some(
                        "PLL2 input frequency range\r Set and reset by software to select the proper reference frequency range used for PLL2.\r This bit must be written before enabling the PLL2.\r 00-01-10: PLL2 input (ref2_ck) clock range frequency between 4 and 8 MHz",
                    ),
                    bit_offset: 2,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Pllrge",
                    ),
                },
                Field {
                    name: "pllfracen",
                    description: Some(
                        "PLL2 fractional latch enable\r Set and reset by software to latch the content of PLL2FRACN into the Î£Î modulator.\r In order to latch the PLL2FRACN value into the Î£Î modulator, PLL2FRACEN must be set\u{a0}to\u{a0}0, then set to 1: the transition 0 to 1 transfers the content of PLL2FRACN into the modulator (see for details).",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllm",
                    description: Some(
                        "Prescaler for PLL2\r Set and cleared by software to configure the prescaler of the PLL2. The VCO2 input frequency is PLL2 input clock frequency/PLL2M.\r This bit can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0).\r ...",
                    ),
                    bit_offset: 8,
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Pllm",
                    ),
                },
                Field {
                    name: "pllpen",
                    description: Some(
                        "PLL2 DIVP divider output enable\r Set and reset by software to enable the pll2_p_ck output of the PLL2.\r To save power, PLL2PEN and PLL2P bits must be set to 0 when the pll2_p_ck is not used.\r This bit can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0).",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllqen",
                    description: Some(
                        "PLL2 DIVQ divider output enable\r Set and reset by software to enable the pll2_q_ck output of the PLL2.\r To save power, PLL2QEN and PLL2Q bits must be set to 0 when the pll2_q_ck is not used.\r This bit can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0.",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllren",
                    description: Some(
                        "PLL2 DIVR divider output enable\r Set and reset by software to enable the pll2_r_ck output of the PLL2.\r To save power, PLL2REN and PLL2R bits must be set to 0 when the pll2_r_ck is not used.\r This bit can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0).",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pll3divr",
            extends: None,
            description: Some(
                "RCC PLL3 dividers configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "plln",
                    description: Some(
                        "Multiplication factor for PLL3 VCO\r Set and reset by software to control the multiplication factor of the VCO.\r These bits can be written only when the PLL is disabled (PLL3ON = 0 and PLL3RDY = 0).\r ...\r ...\r Others: reserved\r VCO output frequency = Fref3_ck x PLL3N, when fractional value 0 has been loaded into PLL3FRACN, with:\r PLL3N between 4 and 512\r input frequency Fref3_ck between 4 and 16MHz",
                    ),
                    bit_offset: 0,
                    bit_size: 9,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllp",
                    description: Some(
                        "PLL3 DIVP division factor\r Set and reset by software to control the frequency of the pll3_p_ck clock.\r These bits can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0).\r ...",
                    ),
                    bit_offset: 9,
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllq",
                    description: Some(
                        "PLL3 DIVQ division factor\r Set and reset by software to control the frequency of the pll3_q_ck clock.\r These bits can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0).\r ...",
                    ),
                    bit_offset: 16,
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllr",
                    description: Some(
                        "PLL3 DIVR division factor\r Set and reset by software to control the frequency of the pll3_r_ck clock.\r These bits can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0).\r ...",
                    ),
                    bit_offset: 24,
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pll2fracr",
            extends: None,
            description: Some(
                "RCC PLL2 fractional divider register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pllfracn",
                    description: Some(
                        "Fractional part of the multiplication factor for PLL2 VCO\r Set and reset by software to control the fractional part of the multiplication factor of the VCO.\r These bits can be written at any time, allowing dynamic fine-tuning of the PLL2 VCO.\r VCO output frequency = Fref2_ck x (PLL2N + (PLL2FRACN / 213)), with\r PLL2N must be between 4 and 512.\r PLL2FRACN can be between 0 and 213 - 1.\r The input frequency Fref2_ck must be between 4 and 16 MHz.\r In order to change the FRACN value on-the-fly even if the PLL is enabled, the application must proceed as follows:\r Set the bit PLL2FRACEN to 0.\r Write the new fractional value into PLL2FRACN.\r Set the bit PLL2FRACEN to 1.",
                    ),
                    bit_offset: 3,
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Icscr2",
            extends: None,
            description: Some(
                "RCC internal clock sources calibration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "msitrim3",
                    description: Some(
                        "MSI clock trimming for ranges 12 to 15\r These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC3[4:0] bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI.",
                    ),
                    bit_offset: 0,
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "msitrim2",
                    description: Some(
                        "MSI clock trimming for ranges 8 to 11\r These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC2[4:0] bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI.",
                    ),
                    bit_offset: 5,
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "msitrim1",
                    description: Some(
                        "MSI clock trimming for ranges 4 to 7\r These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC1[4:0] bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI.",
                    ),
                    bit_offset: 10,
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "msitrim0",
                    description: Some(
                        "MSI clock trimming for ranges 0 to 3\r These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC0[4:0] bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI.",
                    ),
                    bit_offset: 15,
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pll3fracr",
            extends: None,
            description: Some(
                "RCC PLL3 fractional divider register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pllfracn",
                    description: Some(
                        "Fractional part of the multiplication factor for PLL3 VCO\r Set and reset by software to control the fractional part of the multiplication factor of the VCO.\r These bits can be written at any time, allowing dynamic fine-tuning of the PLL3 VCO.\r VCO output frequency = Fref3_ck x (PLL3N + (PLL3FRACN / 213)), with:\r PLL3N must be between 4 and 512.\r PLL3FRACN can be between 0 and 213 - 1.\r The input frequency Fref3_ck must be between 4 and 16 MHz.\r In order to change the FRACN value on-the-fly even if the PLL is enabled, the application must proceed as follows:\r Set the bit PLL3FRACEN to 0.\r Write the new fractional value into PLL3FRACN.\r Set the bit PLL3FRACEN to 1.",
                    ),
                    bit_offset: 3,
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ahb2rstr1",
            extends: None,
            description: Some(
                "RCC AHB2 peripheral reset register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "gpioarst",
                    description: Some(
                        "IO port A reset\r Set and cleared by software.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiobrst",
                    description: Some(
                        "IO port B reset\r Set and cleared by software.",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiocrst",
                    description: Some(
                        "IO port C reset\r Set and cleared by software.",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiodrst",
                    description: Some(
                        "IO port D reset\r Set and cleared by software.",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpioerst",
                    description: Some(
                        "IO port E reset\r Set and cleared by software.",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiofrst",
                    description: Some(
                        "IO port F reset\r Set and cleared by software.",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiogrst",
                    description: Some(
                        "IO port G reset\r Set and cleared by software.",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiohrst",
                    description: Some(
                        "IO port H reset\r Set and cleared by software.",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpioirst",
                    description: Some(
                        "IO port I reset\r Set and cleared by software.",
                    ),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpiojrst",
                    description: Some(
                        "I/O port J reset\r This bit is set and cleared by software.\r Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.",
                    ),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adc12rst",
                    description: Some(
                        "ADC1 and ADC2 reset\r This bit is set and cleared by software.\r Note: This bit impacts ADC1 in STM32U535/545/575/585, and ADC1/ADC2 in�STM32U59x/5Ax/5Fx/5Gx.",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dcmirst",
                    description: Some(
                        "DCMI and PSSI reset\r Set and cleared by software.",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usb_otg_fsrst",
                    description: Some(
                        "OTG_FS reset\r Set and cleared by software.",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usb_otg_hsrst",
                    description: Some(
                        "OTG_HS reset\r Set and cleared by software.",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aesrst",
                    description: Some(
                        "AES hardware accelerator reset\r Set and cleared by software.",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hashrst",
                    description: Some(
                        "Hash reset\r Set and cleared by software.",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rngrst",
                    description: Some(
                        "Random number generator reset\r Set and cleared by software.",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pkarst",
                    description: Some(
                        "PKA reset\r Set and cleared by software.",
                    ),
                    bit_offset: 19,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "saesrst",
                    description: Some(
                        "SAES hardware accelerator reset\r Set and cleared by software.",
                    ),
                    bit_offset: 20,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "octospimrst",
                    description: Some(
                        "OCTOSPIM reset\r Set and cleared by software.",
                    ),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "otfdec1rst",
                    description: Some(
                        "OTFDEC1 reset\r Set and cleared by software.",
                    ),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "otfdec2rst",
                    description: Some(
                        "OTFDEC2 reset\r Set and cleared by software.",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sdmmc1rst",
                    description: Some(
                        "SDMMC1 reset\r Set and cleared by software.",
                    ),
                    bit_offset: 27,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sdmmc2rst",
                    description: Some(
                        "SDMMC2 reset\r Set and cleared by software.",
                    ),
                    bit_offset: 28,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Apb1enr1",
            extends: None,
            description: Some(
                "RCC APB1 peripheral clock enable register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim2en",
                    description: Some(
                        "TIM2 clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim3en",
                    description: Some(
                        "TIM3 clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim4en",
                    description: Some(
                        "TIM4 clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim5en",
                    description: Some(
                        "TIM5 clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim6en",
                    description: Some(
                        "TIM6 clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim7en",
                    description: Some(
                        "TIM7 clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wwdgen",
                    description: Some(
                        "WWDG clock enable\r Set by software to enable the window watchdog clock. Reset by hardware system reset.\r This bit can also be set by hardware if the WWDG_SW option bit is reset.",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi2en",
                    description: Some(
                        "SPI2 clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 14,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart2en",
                    description: Some(
                        "USART2 clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart3en",
                    description: Some(
                        "USART3 clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uart4en",
                    description: Some(
                        "UART4 clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 19,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uart5en",
                    description: Some(
                        "UART5 clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 20,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c1en",
                    description: Some(
                        "I2C1 clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c2en",
                    description: Some(
                        "I2C2 clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 22,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "crsen",
                    description: Some(
                        "CRS clock enable\r Set and cleared by software.",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usart6en",
                    description: Some(
                        "USART6 clock enable\r This bit is set and cleared by software.\r Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value.",
                    ),
                    bit_offset: 25,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Seccfgr",
            extends: None,
            description: Some(
                "RCC secure configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsisec",
                    description: Some(
                        "HSI clock configuration and status bits security\r Set and reset by software.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Security",
                    ),
                },
                Field {
                    name: "hsesec",
                    description: Some(
                        "HSE clock configuration bits, status bits and HSE_CSS security\r Set and reset by software.",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Security",
                    ),
                },
                Field {
                    name: "msisec",
                    description: Some(
                        "MSI clock configuration and status bits security\r Set and reset by software.",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Security",
                    ),
                },
                Field {
                    name: "lsisec",
                    description: Some(
                        "LSI clock configuration and status bits security\r Set and reset by software.",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Security",
                    ),
                },
                Field {
                    name: "lsesec",
                    description: Some(
                        "LSE clock configuration and status bits security\r Set and reset by software.",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Security",
                    ),
                },
                Field {
                    name: "sysclksec",
                    description: Some(
                        "SYSCLK clock selection, STOPWUCK bit, clock output on MCO configuration security\r Set and reset by software.",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Security",
                    ),
                },
                Field {
                    name: "prescsec",
                    description: Some(
                        "AHBx/APBx prescaler configuration bits security\r Set and reset by software.",
                    ),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Security",
                    ),
                },
                Field {
                    name: "pllsec",
                    description: Some(
                        "PLL1 clock configuration and status bits security\r Set and reset by software.",
                    ),
                    bit_offset: 7,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 3,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Security",
                    ),
                },
                Field {
                    name: "iclksec",
                    description: Some(
                        "intermediate clock source selection security\r Set and reset by software.",
                    ),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Security",
                    ),
                },
                Field {
                    name: "hsi48sec",
                    description: Some(
                        "HSI48 clock configuration and status bits security\r Set and reset by software.",
                    ),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Security",
                    ),
                },
                Field {
                    name: "rmvfsec",
                    description: Some(
                        "Remove reset flag security\r Set and reset by software.",
                    ),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Security",
                    ),
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Msipllfast",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NORMAL",
                    description: Some(
                        "MSI PLL normal start-up",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FAST",
                    description: Some(
                        "MSI PLL fast start-up",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Msixsrange",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "RANGE_4MHZ",
                    description: Some(
                        "range 4 around 4M Hz (reset value)",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "RANGE_2MHZ",
                    description: Some(
                        "range 5 around 2 MHz",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "RANGE_1_5MHZ",
                    description: Some(
                        "range 6 around 1.5 MHz",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "RANGE_1MHZ",
                    description: Some(
                        "range 7 around 1 MHz",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "RANGE_3_072MHZ",
                    description: Some(
                        "range 8 around 3.072 MHz",
                    ),
                    value: 8,
                },
            ],
        },
        Enum {
            name: "Usartsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PCLK2",
                    description: Some(
                        "PCLK2 selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYSCLK",
                    description: Some(
                        "SYSCLK selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSI16",
                    description: Some(
                        "HSI16 selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE selected",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Rngsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "HSI48",
                    description: Some(
                        "HSI48 selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HSI48_DIV2",
                    description: Some(
                        "HSI48 / 2 selected, can be used in Range 4",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSI16",
                    description: Some(
                        "HSI16 selected",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Iclksel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "HSI48",
                    description: Some(
                        "HSI48 clock selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL2_Q",
                    description: Some(
                        "PLL2 Q (pll2_q_ck) selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL1_Q",
                    description: Some(
                        "PLL1 Q (pll1_q_ck) selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "MSIK",
                    description: Some(
                        "MSIK clock selected",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Icsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PCLK1",
                    description: Some(
                        "PCLK1 selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYSCLK",
                    description: Some(
                        "SYSCLK selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSI16",
                    description: Some(
                        "HSI16 selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "MSIK",
                    description: Some(
                        "MSIK selected",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Otghssel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL1_P",
                    description: Some(
                        "PLL1 “P” (pll1_q_ck) selected,",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSE_DIV2",
                    description: Some(
                        "HSE/2 selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "PLL1_P_DIV2",
                    description: Some(
                        "PLL1 “P” divided by 2 (pll1_p_ck/2) selected",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Systicksel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "HCLK_DIV8",
                    description: Some(
                        "HCLK/8 selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LSI",
                    description: Some(
                        "LSI selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE selected",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Mdfsel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "HCLK",
                    description: Some(
                        "HCLK selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL1_P",
                    description: Some(
                        "PLL1 P (pll1_p_ck) selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL3_Q",
                    description: Some(
                        "PLL3 Q (pll3_q_ck) selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "AUDIOCLK",
                    description: Some(
                        "input pin AUDIOCLK selected",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "MSIK",
                    description: Some(
                        "MSIK clock selected",
                    ),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "Octospisel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "SYSCLK",
                    description: Some(
                        "SYSCLK selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MSIK",
                    description: Some(
                        "MSIK selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL1_Q",
                    description: Some(
                        "PLL1 Q (pll1_q_ck) selected, can be up to 200 MHz",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "PLL2_Q",
                    description: Some(
                        "PLL2 Q (pll2_q_ck) selected, can be up to 200 MHz",
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
            name: "Uartsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PCLK1",
                    description: Some(
                        "PCLK1 selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYSCLK",
                    description: Some(
                        "SYSCLK selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSI16",
                    description: Some(
                        "HSI16 selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE selected",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Timicsel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "NONE",
                    description: Some(
                        "No sources can be selected by TIM16, TIM17 and LPTIM2 as internal input capture",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HSI256_MSIS1024_MSIS4",
                    description: Some(
                        "HSI/256, MSIS/1024 and MSIS/4 generated and can be selected by TIM16, TIM17 and LPTIM2 as internal input capture",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "HSI256_MSIS1024_MSIK4",
                    description: Some(
                        "HSI/256, MSIS/1024 and MSIK/4 generated and can be selected by TIM16, TIM17 and LPTIM2 as internal input capture",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "HSI256_MSIK1024_MSIS4",
                    description: Some(
                        "HSI/256, MSIK/1024 and MSIS/4 generated and can be selected by TIM16, TIM17 and LPTIM2 as internal input capture",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "HSI256_MSIK1024_MSIK4",
                    description: Some(
                        "HSI/256, MSIK/1024 and MSIK/4 generated and can be selected by TIM16, TIM17 and LPTIM2 as internal input capture",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Sdmmcsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ICLK",
                    description: Some(
                        "ICLK clock selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL1_P",
                    description: Some(
                        "PLL1 P (pll1_p_ck) selected, in case higher than 48 MHz is needed (for SDR50 mode)",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Lptimsel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PCLK1",
                    description: Some(
                        "PCLK1 selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LSI",
                    description: Some(
                        "LSI selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSI16",
                    description: Some(
                        "HSI16 selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE selected",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Msibias",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CONTINUOUS",
                    description: Some(
                        "MSI bias continuous mode (clock accuracy fast settling time)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SAMPLING",
                    description: Some(
                        "MSI bias sampling mode (ultra-low-power mode)",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Sw",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "MSIS",
                    description: Some(
                        "MSIS selected as system clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HSI16",
                    description: Some(
                        "HSI16 selected as system clock",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE selected as system clock",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "PLL1_R",
                    description: Some(
                        "PLL pll1_r_ck selected as system clock",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Msipllsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "MSIK",
                    description: Some(
                        "PLL mode applied to MSIK (MSI kernel) clock output",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MSIS",
                    description: Some(
                        "PLL mode applied to MSIS (MSI system) clock output",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Fdcansel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE clock selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL1_Q",
                    description: Some(
                        "PLL1 Q (pll1_q_ck) selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL2_P",
                    description: Some(
                        "PLL2 P (pll2_p_ck) selected",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Dpre",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: Some(
                        "DCLK not divided",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "DCLK divided by 2",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "DCLK divided by 4",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "DCLK divided by 8",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "DIV16",
                    description: Some(
                        "DCLK divided by 16",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Pllm",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "BYPASS",
                    description: Some(
                        "division by 1 (bypass)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "division by 2",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV3",
                    description: Some(
                        "division by 3",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV16",
                    description: Some(
                        "division by 16",
                    ),
                    value: 15,
                },
            ],
        },
        Enum {
            name: "Stopwuck",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "MSIS",
                    description: Some(
                        "MSIS oscillator selected as wakeup from stop clock and CSS backup clock",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HSI16",
                    description: Some(
                        "HSI16 oscillator selected as wakeup from stop clock and CSS backup clock",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Lscosel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "LSI",
                    description: Some(
                        "LSI clock selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE clock selected",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Adcdacsel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "HCLK",
                    description: Some(
                        "HCLK clock selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYSCLK",
                    description: Some(
                        "SYSCLK selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL2_R",
                    description: Some(
                        "PLL2 R (pll2_r_ck) selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE clock selected",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "HSI16",
                    description: Some(
                        "HSI16 clock selected",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "MSI_K",
                    description: Some(
                        "MSIK clock selected",
                    ),
                    value: 5,
                },
            ],
        },
        Enum {
            name: "Hspisel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "SYSCLK",
                    description: Some(
                        "SYSCLK selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL1_Q",
                    description: Some(
                        "PLL1 “Q” (pll1_q_ck) selected, can be up to 200 MHz",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL2_Q",
                    description: Some(
                        "PLL2 “Q” (pll2_q_ck) selected, can be up to 200 MHz",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "PLL3_R",
                    description: Some(
                        "PLL3 “R” (pll3_r_ck) selected, can be up to 200 MHz",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Stopkerwuck",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "MSIK",
                    description: Some(
                        "MSIK oscillator automatically enabled when exiting Stop mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HSI16",
                    description: Some(
                        "HSI16 oscillator automatically enabled when exiting Stop mode",
                    ),
                    value: 1,
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
                        "MCO divided by 1",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "MCO divided by 2",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "MCO divided by 4",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "MCO divided by 8",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "DIV16",
                    description: Some(
                        "MCO divided by 16",
                    ),
                    value: 4,
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
                    name: "MEDIUMLOW",
                    description: Some(
                        "Medium low driving capability",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "MEDIUMHIGH",
                    description: Some(
                        "Medium high driving capability",
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
            name: "Lsiprediv",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DIV1",
                    description: Some(
                        "LSI not divided",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV128",
                    description: Some(
                        "LSI divided by 128",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pllmboost",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "BYPASS",
                    description: Some(
                        "division by 1 (bypass)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIV2",
                    description: Some(
                        "division by 2",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "DIV4",
                    description: Some(
                        "division by 4",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "DIV6",
                    description: Some(
                        "division by 6",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "DIV8",
                    description: Some(
                        "division by 8",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "DIV10",
                    description: Some(
                        "division by 10",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "DIV12",
                    description: Some(
                        "division by 12",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "DIV14",
                    description: Some(
                        "division by 14",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "DIV16",
                    description: Some(
                        "division by 16",
                    ),
                    value: 8,
                },
            ],
        },
        Enum {
            name: "Mcosel",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "DISABLE",
                    description: Some(
                        "MCO output disabled, no clock on MCO",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYSCLK",
                    description: Some(
                        "SYSCLK system clock selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "MSIS",
                    description: Some(
                        "MSIS clock selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HSI16",
                    description: Some(
                        "HSI16 clock selected",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE clock selected",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "PLL1_R",
                    description: Some(
                        "Main PLL clock pll1_r_ck selected",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "LSI",
                    description: Some(
                        "LSI clock selected",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE clock selected",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "HSI48",
                    description: Some(
                        "Internal HSI48 clock selected",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "MSIK",
                    description: Some(
                        "MSIK clock selected",
                    ),
                    value: 9,
                },
            ],
        },
        Enum {
            name: "Priv",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "UNPRIVILEGED",
                    description: Some(
                        "Read and write to secure functions can be done by privileged or unprivileged access.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PRIVILEGED",
                    description: Some(
                        "Read and write to secure functions can be done by privileged access only.",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Hseext",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ANALOG",
                    description: Some(
                        "external HSE clock analog mode",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DIGITAL",
                    description: Some(
                        "external HSE clock digital mode (through I/O Schmitt trigger)",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Dsisel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "PLL3_P",
                    description: Some(
                        "PLL3 “P” (pll3_p_ck) selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "DCLK",
                    description: Some(
                        "DSI PHY PLL output selected",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Lpuartsel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "PCLK3",
                    description: Some(
                        "PCLK3 selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYSCLK",
                    description: Some(
                        "SYSCLK selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSI16",
                    description: Some(
                        "HSI16 selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE selected",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "MSIK",
                    description: Some(
                        "MSIK selected",
                    ),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "Saisel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "PLL2_P",
                    description: Some(
                        "PLL2 P (pll2_p_ck) selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL3_P",
                    description: Some(
                        "PLL3 P (pll3_p_ck) selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL1_P",
                    description: Some(
                        "PLL1 P (pll1_p_ck) selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "AUDIOCLK",
                    description: Some(
                        "input pin AUDIOCLK selected",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "HSI16",
                    description: Some(
                        "HSI16 clock selected",
                    ),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "Security",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NON_SECURE",
                    description: Some(
                        "non secure",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SECURE",
                    description: Some(
                        "secure",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Dacsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LSI",
                    description: Some(
                        "LSI selected",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Adfsel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "HCLK",
                    description: Some(
                        "HCLK selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL1_P",
                    description: Some(
                        "PLL1 P (pll1_p_ck) selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "PLL3_Q",
                    description: Some(
                        "PLL3 Q (pll3_q_ck) selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "AUDIOCLK",
                    description: Some(
                        "input pin AUDIOCLK selected",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "MSIK",
                    description: Some(
                        "MSIK clock selected",
                    ),
                    value: 4,
                },
            ],
        },
        Enum {
            name: "Saessel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "SHSI",
                    description: Some(
                        "SHSI selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SHSI_DIV2",
                    description: Some(
                        "SHSI / 2 selected, can be used in Range 4",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Spisel",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "PCLK2",
                    description: Some(
                        "PCLK2 selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYSCLK",
                    description: Some(
                        "SYSCLK selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSI16",
                    description: Some(
                        "HSI16 selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "MSIK",
                    description: Some(
                        "MSIK selected",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Msirgsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "RCC_CSR",
                    description: Some(
                        "MSIS/MSIK ranges provided by MSISSRANGE[3:0] and MSIKSRANGE[3:0] in RCC_CSR",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RCC_ICSCR1",
                    description: Some(
                        "MSIS/MSIK ranges provided by MSISRANGE[3:0] and MSIKRANGE[3:0] in RCC_ICSCR1",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ltdcsel",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "PLL3_R",
                    description: Some(
                        "PLL3 “R” (pll3_r_ck) selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "PLL2_R",
                    description: Some(
                        "PLL2 “R” (pll2_r_ck) selected",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Msirange",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "RANGE_48MHZ",
                    description: Some(
                        "range 0 around 48 MHz",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RANGE_24MHZ",
                    description: Some(
                        "range 1 around 24 MHz",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "RANGE_16MHZ",
                    description: Some(
                        "range 2 around 16 MHz",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "RANGE_12MHZ",
                    description: Some(
                        "range 3 around 12 MHz",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "RANGE_4MHZ",
                    description: Some(
                        "range 4 around 4 MHz (reset value)",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "RANGE_2MHZ",
                    description: Some(
                        "range 5 around 2 MHz",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "RANGE_1_33MHZ",
                    description: Some(
                        "range 6 around 1.33 MHz",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "RANGE_1MHZ",
                    description: Some(
                        "range 7 around 1 MHz",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "RANGE_3_072MHZ",
                    description: Some(
                        "range 8 around 3.072 MHz",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "RANGE_1_536MHZ",
                    description: Some(
                        "range 9 around 1.536 MHz",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "RANGE_1_024MHZ",
                    description: Some(
                        "range 10 around 1.024 MHz",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "RANGE_768KHZ",
                    description: Some(
                        "range 11 around 768 kHz",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "RANGE_400KHZ",
                    description: Some(
                        "range 12 around 400 kHz",
                    ),
                    value: 12,
                },
                EnumVariant {
                    name: "RANGE_200KHZ",
                    description: Some(
                        "range 13 around 200 kHz",
                    ),
                    value: 13,
                },
                EnumVariant {
                    name: "RANGE_133KHZ",
                    description: Some(
                        "range 14 around 133 kHz",
                    ),
                    value: 14,
                },
                EnumVariant {
                    name: "RANGE_100KHZ",
                    description: Some(
                        "range 15 around 100 kHz",
                    ),
                    value: 15,
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
                        "No clock selected",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "LSE",
                    description: Some(
                        "LSE oscillator clock selected",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "LSI",
                    description: Some(
                        "LSI oscillator clock selected",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE oscillator clock divided by 32 selected",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Pllsrc",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "NONE",
                    description: Some(
                        "No clock sent to PLL3",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "MSIS",
                    description: Some(
                        "MSIS clock selected as PLL3 clock entry",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "HSI16",
                    description: Some(
                        "HSI16 clock selected as PLL3 clock entry",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "HSE",
                    description: Some(
                        "HSE clock selected as PLL3 clock entry",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Pllrge",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "FREQ_4TO8MHZ",
                    description: Some(
                        "PLL2 input (ref2_ck) clock range frequency between 4 and 8 MHz",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FREQ_8TO16MHZ",
                    description: Some(
                        "PLL2 input (ref2_ck) clock range frequency between 8 and 16 MHz",
                    ),
                    value: 3,
                },
            ],
        },
    ],
};
