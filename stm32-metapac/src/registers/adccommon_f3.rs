
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "AdcCommon",
        extends: None,
        description: Some("ADC common registers"),
        items: &[
            BlockItem {
                name: "csr",
                description: Some("ADC Common status register"),
                array: None,
                byte_offset: 0,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Csr"),
                }),
            },
            BlockItem {
                name: "ccr",
                description: Some("ADC common control register"),
                array: None,
                byte_offset: 8,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ccr"),
                }),
            },
            BlockItem {
                name: "cdr",
                description: Some("ADC common regular data register for dual and triple modes"),
                array: None,
                byte_offset: 12,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Cdr"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Cdr",
            extends: None,
            description: Some("ADC common regular data register for dual and triple modes"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdata_mst",
                    description: Some("Regular data of the master ADC"),
                    bit_offset: 0,
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rdata_slv",
                    description: Some("Regular data of the master ADC"),
                    bit_offset: 16,
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ccr",
            extends: None,
            description: Some("ADC common control register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dual",
                    description: Some("Dual ADC mode selection"),
                    bit_offset: 0,
                    bit_size: 5,
                    array: None,
                    enumm: Some("Dual"),
                },
                Field {
                    name: "delay",
                    description: Some("Delay between 2 sampling phases"),
                    bit_offset: 8,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dmacfg",
                    description: Some("DMA configuration (for multi-ADC mode)"),
                    bit_offset: 13,
                    bit_size: 1,
                    array: None,
                    enumm: Some("Dmacfg"),
                },
                Field {
                    name: "mdma",
                    description: Some("Direct memory access mode for multi ADC mode"),
                    bit_offset: 14,
                    bit_size: 2,
                    array: None,
                    enumm: Some("Mdma"),
                },
                Field {
                    name: "ckmode",
                    description: Some("ADC clock mode"),
                    bit_offset: 16,
                    bit_size: 2,
                    array: None,
                    enumm: Some("Ckmode"),
                },
                Field {
                    name: "vrefen",
                    description: Some("VREFINT enable"),
                    bit_offset: 22,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tsen",
                    description: Some("Temperature sensor enable"),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vbaten",
                    description: Some("VBAT enable"),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Csr",
            extends: None,
            description: None,
            bit_size: 32,
            fields: &[
                Field {
                    name: "adrdy_mst",
                    description: Some("Master ADC ready"),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eosmp_mst",
                    description: Some("End of sampling phase flag of the master ADC"),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ended"),
                },
                Field {
                    name: "eoc_mst",
                    description: Some("End of regular conversion of the master ADC"),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ended"),
                },
                Field {
                    name: "eos_mst",
                    description: Some("End of regular sequence flag of the master ADC"),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ended"),
                },
                Field {
                    name: "ovr_mst",
                    description: Some("Overrun flag of the master ADC"),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ovr"),
                },
                Field {
                    name: "jeoc_mst",
                    description: Some("End of injected conversion of the master ADC"),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ended"),
                },
                Field {
                    name: "jeos",
                    description: Some("End of injected sequence flag of the master ADC"),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ended"),
                },
                Field {
                    name: "awd1_mst",
                    description: Some("Analog watchdog 1 flag of the master ADC"),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: Some("Awd"),
                },
                Field {
                    name: "awd2_mst",
                    description: Some("Analog watchdog 2 flag of the master ADC"),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: Some("Awd"),
                },
                Field {
                    name: "awd3_mst",
                    description: Some("Analog watchdog 3 flag of the master ADC"),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: Some("Awd"),
                },
                Field {
                    name: "jqovf_mst",
                    description: Some("Injected context queue overflow flag of the master ADC"),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: Some("Jqovf"),
                },
                Field {
                    name: "adrdy_slv",
                    description: Some("Slave ADC ready"),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eosmp_slv",
                    description: Some("End of sampling phase flag of the slave ADC"),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ended"),
                },
                Field {
                    name: "eoc_slv",
                    description: Some("End of regular conversion of the slave ADC"),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ended"),
                },
                Field {
                    name: "eos_slv",
                    description: Some("End of regular sequence flag of the slave ADC"),
                    bit_offset: 19,
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ended"),
                },
                Field {
                    name: "ovr_slv",
                    description: Some("Overrun flag of the slave ADC"),
                    bit_offset: 20,
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ovr"),
                },
                Field {
                    name: "jeoc_slv",
                    description: Some("End of injected conversion of the slave ADC"),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ended"),
                },
                Field {
                    name: "jeos_slv",
                    description: Some("End of injected sequence flag of the slave ADC"),
                    bit_offset: 22,
                    bit_size: 1,
                    array: None,
                    enumm: Some("Ended"),
                },
                Field {
                    name: "awd1_slv",
                    description: Some("Analog watchdog 1 flag of the slave ADC"),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: Some("Awd"),
                },
                Field {
                    name: "awd2_slv",
                    description: Some("Analog watchdog 2 flag of the slave ADC"),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: Some("Awd"),
                },
                Field {
                    name: "awd3_slv",
                    description: Some("Analog watchdog 3 flag of the slave ADC"),
                    bit_offset: 25,
                    bit_size: 1,
                    array: None,
                    enumm: Some("Awd"),
                },
                Field {
                    name: "jqovf_slv",
                    description: Some("Injected context queue overflow flag of the slave ADC"),
                    bit_offset: 26,
                    bit_size: 1,
                    array: None,
                    enumm: Some("Jqovf"),
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Ovr",
            description: Some("Overrun flag"),
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOOVERRUN",
                    description: Some("No overrun occurred"),
                    value: 0,
                },
                EnumVariant {
                    name: "OVERRUN",
                    description: Some("Overrun occurred"),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Dmacfg",
            description: Some("DMA configuration (for multi-ADC mode)"),
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ONESHOT",
                    description: Some("DMA one shot mode selected"),
                    value: 0,
                },
                EnumVariant {
                    name: "CIRCULATOR",
                    description: Some("DMA circular mode selected"),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Mdma",
            description: Some("Direct memory access mode for multi ADC mode"),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some("MDMA mode disabled"),
                    value: 0,
                },
                EnumVariant {
                    name: "BITS12_10",
                    description: Some("MDMA mode enabled for 12 and 10-bit resolution"),
                    value: 2,
                },
                EnumVariant {
                    name: "BIT8_6",
                    description: Some("MDMA mode enabled for 8 and 6-bit resolution"),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Awd",
            description: Some("Analog watchdog flag"),
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOEVENT",
                    description: Some("No analog watchdog event occurred"),
                    value: 0,
                },
                EnumVariant {
                    name: "EVENT",
                    description: Some("Analog watchdog event occurred"),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Jqovf",
            description: Some("Injected context queue overflow flag"),
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOOVERFLOW",
                    description: Some("No injected context queue overflow"),
                    value: 0,
                },
                EnumVariant {
                    name: "OVERFLOW",
                    description: Some("Injected context queue overflow"),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Dual",
            description: Some("Dual ADC mode selection"),
            bit_size: 5,
            variants: &[
                EnumVariant {
                    name: "INDEPENDENT",
                    description: Some("Independent mode"),
                    value: 0,
                },
                EnumVariant {
                    name: "DUALRJ",
                    description: Some("Dual, combined regular simultaneous + injected simultaneous mode"),
                    value: 1,
                },
                EnumVariant {
                    name: "DUALRA",
                    description: Some("Dual, combined regular simultaneous + alternate trigger mode"),
                    value: 2,
                },
                EnumVariant {
                    name: "DUALIJ",
                    description: Some("Dual, combined injected simultaneous + fast interleaved mode"),
                    value: 3,
                },
                EnumVariant {
                    name: "DUALJ",
                    description: Some("Dual, injected simultaneous mode only"),
                    value: 5,
                },
                EnumVariant {
                    name: "DUALR",
                    description: Some("Dual, regular simultaneous mode only"),
                    value: 6,
                },
                EnumVariant {
                    name: "DUALI",
                    description: Some("dual, interleaved mode only"),
                    value: 7,
                },
                EnumVariant {
                    name: "DUALA",
                    description: Some("Dual, alternate trigger mode only"),
                    value: 9,
                },
            ],
        },
        Enum {
            name: "Ended",
            description: Some("End of operation"),
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOTENDED",
                    description: Some("Operation is not ended"),
                    value: 0,
                },
                EnumVariant {
                    name: "ENDED",
                    description: Some("Operation is ended"),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ckmode",
            description: Some("ADC clock mode"),
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "ASYNCHRONOUS",
                    description: Some("Use Kernel Clock adc_ker_ck_input divided by PRESC. Asynchronous mode"),
                    value: 0,
                },
                EnumVariant {
                    name: "SYNCDIV1",
                    description: Some("Use AHB clock rcc_hclk3. In this case rcc_hclk must equal sys_d1cpre_ck."),
                    value: 1,
                },
                EnumVariant {
                    name: "SYNCDIV2",
                    description: Some("Use AHB clock rcc_hclk3 divided by 2."),
                    value: 2,
                },
                EnumVariant {
                    name: "SYNCDIV4",
                    description: Some("Use AHB clock rcc_hclk3 divided by 4."),
                    value: 3,
                },
            ],
        },
    ],
};
