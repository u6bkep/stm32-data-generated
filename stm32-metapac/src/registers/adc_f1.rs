
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[Block {
        name: "Adc",
        extends: None,
        description: Some("Analog-to-digital converter"),
        items: &[
            BlockItem {
                name: "sr",
                description: Some("status register"),
                array: None,
                byte_offset: 0,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Sr"),
                }),
            },
            BlockItem {
                name: "cr1",
                description: Some("control register 1"),
                array: None,
                byte_offset: 4,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cr1"),
                }),
            },
            BlockItem {
                name: "cr2",
                description: Some("control register 2"),
                array: None,
                byte_offset: 8,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Cr2"),
                }),
            },
            BlockItem {
                name: "smpr1",
                description: Some("sample time register 1"),
                array: None,
                byte_offset: 12,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Smpr1"),
                }),
            },
            BlockItem {
                name: "smpr2",
                description: Some("sample time register 2"),
                array: None,
                byte_offset: 16,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Smpr2"),
                }),
            },
            BlockItem {
                name: "jofr",
                description: Some("injected channel data offset register x"),
                array: Some(Array::Regular(RegularArray { len: 4, stride: 4 })),
                byte_offset: 20,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Jofr"),
                }),
            },
            BlockItem {
                name: "htr",
                description: Some("watchdog higher threshold register"),
                array: None,
                byte_offset: 36,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Htr"),
                }),
            },
            BlockItem {
                name: "ltr",
                description: Some("watchdog lower threshold register"),
                array: None,
                byte_offset: 40,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Ltr"),
                }),
            },
            BlockItem {
                name: "sqr1",
                description: Some("regular sequence register 1"),
                array: None,
                byte_offset: 44,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Sqr1"),
                }),
            },
            BlockItem {
                name: "sqr2",
                description: Some("regular sequence register 2"),
                array: None,
                byte_offset: 48,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Sqr2"),
                }),
            },
            BlockItem {
                name: "sqr3",
                description: Some("regular sequence register 3"),
                array: None,
                byte_offset: 52,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Sqr3"),
                }),
            },
            BlockItem {
                name: "jsqr",
                description: Some("injected sequence register"),
                array: None,
                byte_offset: 56,
                inner: BlockItemInner::Register(Register {
                    access: Access::ReadWrite,
                    bit_size: 32,
                    fieldset: Some("Jsqr"),
                }),
            },
            BlockItem {
                name: "jdr",
                description: Some("injected data register x"),
                array: Some(Array::Regular(RegularArray { len: 4, stride: 4 })),
                byte_offset: 60,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Jdr"),
                }),
            },
            BlockItem {
                name: "dr",
                description: Some("regular data register"),
                array: None,
                byte_offset: 76,
                inner: BlockItemInner::Register(Register {
                    access: Access::Read,
                    bit_size: 32,
                    fieldset: Some("Dr"),
                }),
            },
        ],
    }],
    fieldsets: &[
        FieldSet {
            name: "Cr1",
            extends: None,
            description: Some("control register 1"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "awdch",
                    description: Some("Analog watchdog channel select bits"),
                    bit_offset: 0,
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eocie",
                    description: Some("Interrupt enable for EOC"),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "awdie",
                    description: Some("Analog watchdog interrupt enable"),
                    bit_offset: 6,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jeocie",
                    description: Some("Interrupt enable for injected channels"),
                    bit_offset: 7,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "scan",
                    description: Some("Scan mode"),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "awdsgl",
                    description: Some("Enable the watchdog on a single channel in scan mode"),
                    bit_offset: 9,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jauto",
                    description: Some("Automatic injected group conversion"),
                    bit_offset: 10,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "discen",
                    description: Some("Discontinuous mode on regular channels"),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jdiscen",
                    description: Some("Discontinuous mode on injected channels"),
                    bit_offset: 12,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "discnum",
                    description: Some("Discontinuous mode channel count"),
                    bit_offset: 13,
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dualmod",
                    description: Some("Dual mode selection"),
                    bit_offset: 16,
                    bit_size: 4,
                    array: None,
                    enumm: Some("Dualmod"),
                },
                Field {
                    name: "jawden",
                    description: Some("Analog watchdog enable on injected channels"),
                    bit_offset: 22,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "awden",
                    description: Some("Analog watchdog enable on regular channels"),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Smpr2",
            extends: None,
            description: Some("sample time register 2"),
            bit_size: 32,
            fields: &[Field {
                name: "smp",
                description: Some("Channel 0 sampling time selection"),
                bit_offset: 0,
                bit_size: 3,
                array: Some(Array::Regular(RegularArray { len: 10, stride: 3 })),
                enumm: Some("SampleTime"),
            }],
        },
        FieldSet {
            name: "Htr",
            extends: None,
            description: Some("watchdog higher threshold register"),
            bit_size: 32,
            fields: &[Field {
                name: "ht",
                description: Some("Analog watchdog higher threshold"),
                bit_offset: 0,
                bit_size: 12,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Smpr1",
            extends: None,
            description: Some("sample time register 1"),
            bit_size: 32,
            fields: &[Field {
                name: "smp",
                description: Some("Channel x sample time selection"),
                bit_offset: 0,
                bit_size: 3,
                array: Some(Array::Regular(RegularArray { len: 8, stride: 3 })),
                enumm: Some("SampleTime"),
            }],
        },
        FieldSet {
            name: "Sqr3",
            extends: None,
            description: Some("regular sequence register 3"),
            bit_size: 32,
            fields: &[Field {
                name: "sq",
                description: Some("1st to 6th conversion in regular sequence"),
                bit_offset: 0,
                bit_size: 5,
                array: Some(Array::Regular(RegularArray { len: 6, stride: 5 })),
                enumm: None,
            }],
        },
        FieldSet {
            name: "Sqr2",
            extends: None,
            description: Some("regular sequence register 2"),
            bit_size: 32,
            fields: &[Field {
                name: "sq",
                description: Some("7th to 12th conversion in regular sequence"),
                bit_offset: 0,
                bit_size: 5,
                array: Some(Array::Regular(RegularArray { len: 6, stride: 5 })),
                enumm: None,
            }],
        },
        FieldSet {
            name: "Jofr",
            extends: None,
            description: Some("injected channel data offset register x"),
            bit_size: 32,
            fields: &[Field {
                name: "joffset",
                description: Some("Data offset for injected channel x"),
                bit_offset: 0,
                bit_size: 12,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Ltr",
            extends: None,
            description: Some("watchdog lower threshold register"),
            bit_size: 32,
            fields: &[Field {
                name: "lt",
                description: Some("Analog watchdog lower threshold"),
                bit_offset: 0,
                bit_size: 12,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Sr",
            extends: None,
            description: Some("status register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "awd",
                    description: Some("Analog watchdog flag"),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eoc",
                    description: Some("Regular channel end of conversion"),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jeoc",
                    description: Some("Injected channel end of conversion"),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jstrt",
                    description: Some("Injected channel start flag"),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "strt",
                    description: Some("Regular channel start flag"),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dr",
            extends: None,
            description: Some("regular data register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data",
                    description: Some("Regular data"),
                    bit_offset: 0,
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "adc2data",
                    description: Some("ADC2 data"),
                    bit_offset: 16,
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sqr1",
            extends: None,
            description: Some("regular sequence register 1"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sq",
                    description: Some("13th to 16th conversion in regular sequence"),
                    bit_offset: 0,
                    bit_size: 5,
                    array: Some(Array::Regular(RegularArray { len: 4, stride: 5 })),
                    enumm: None,
                },
                Field {
                    name: "l",
                    description: Some("Regular channel sequence length"),
                    bit_offset: 20,
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Jdr",
            extends: None,
            description: Some("injected data register x"),
            bit_size: 32,
            fields: &[Field {
                name: "jdata",
                description: Some("Injected data"),
                bit_offset: 0,
                bit_size: 16,
                array: None,
                enumm: None,
            }],
        },
        FieldSet {
            name: "Jsqr",
            extends: None,
            description: Some("injected sequence register"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "jsq",
                    description: Some("1st conversion in injected sequence"),
                    bit_offset: 0,
                    bit_size: 5,
                    array: Some(Array::Regular(RegularArray { len: 4, stride: 5 })),
                    enumm: None,
                },
                Field {
                    name: "jl",
                    description: Some("Injected sequence length"),
                    bit_offset: 20,
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr2",
            extends: None,
            description: Some("control register 2"),
            bit_size: 32,
            fields: &[
                Field {
                    name: "adon",
                    description: Some("A/D Converter ON / OFF"),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cont",
                    description: Some("Continuous conversion"),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cal",
                    description: Some("A/D Calibration"),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rstcal",
                    description: Some("Reset calibration"),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dma",
                    description: Some("Direct memory access mode (for single ADC mode)"),
                    bit_offset: 8,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "align",
                    description: Some("Data alignment"),
                    bit_offset: 11,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jextsel",
                    description: Some("External event select for injected group"),
                    bit_offset: 12,
                    bit_size: 3,
                    array: None,
                    enumm: Some("Extsel"),
                },
                Field {
                    name: "jexttrig",
                    description: Some("External trigger conversion mode for injected channels"),
                    bit_offset: 15,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "extsel",
                    description: Some("External event select for regular group"),
                    bit_offset: 17,
                    bit_size: 3,
                    array: None,
                    enumm: Some("Extsel"),
                },
                Field {
                    name: "exttrig",
                    description: Some("External trigger conversion mode for regular channels"),
                    bit_offset: 20,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "jswstart",
                    description: Some("Start conversion of injected channels"),
                    bit_offset: 21,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "swstart",
                    description: Some("Start conversion of regular channels"),
                    bit_offset: 22,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tsvrefe",
                    description: Some("Temperature sensor and VREFINT enable"),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Extsel",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "TIM1TRGO",
                    description: Some("Timer 1 TRGO event"),
                    value: 0,
                },
                EnumVariant {
                    name: "TIM1CC4",
                    description: Some("Timer 1 CC4 event"),
                    value: 1,
                },
                EnumVariant {
                    name: "TIM2TRGO",
                    description: Some("Timer 2 TRGO event"),
                    value: 2,
                },
                EnumVariant {
                    name: "TIM2CC1",
                    description: Some("Timer 2 CC1 event"),
                    value: 3,
                },
                EnumVariant {
                    name: "TIM3CC4",
                    description: Some("Timer 3 CC4 event"),
                    value: 4,
                },
                EnumVariant {
                    name: "TIM4TRGO",
                    description: Some("Timer 4 TRGO event"),
                    value: 5,
                },
                EnumVariant {
                    name: "TIM8CC4",
                    description: Some("EXTI line 15/Timer 8 CC4 event"),
                    value: 6,
                },
                EnumVariant {
                    name: "SWSTART",
                    description: Some("SWSTART"),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "SampleTime",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "CYCLES1_5",
                    description: Some("1.5 cycles"),
                    value: 0,
                },
                EnumVariant {
                    name: "CYCLES7_5",
                    description: Some("7.5 cycles"),
                    value: 1,
                },
                EnumVariant {
                    name: "CYCLES13_5",
                    description: Some("13.5 cycles"),
                    value: 2,
                },
                EnumVariant {
                    name: "CYCLES28_5",
                    description: Some("28.5 cycles"),
                    value: 3,
                },
                EnumVariant {
                    name: "CYCLES41_5",
                    description: Some("41.5 cycles"),
                    value: 4,
                },
                EnumVariant {
                    name: "CYCLES55_5",
                    description: Some("55.5 cycles"),
                    value: 5,
                },
                EnumVariant {
                    name: "CYCLES71_5",
                    description: Some("71.5 cycles"),
                    value: 6,
                },
                EnumVariant {
                    name: "CYCLES239_5",
                    description: Some("239.5 cycles"),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Dualmod",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "INDEPENDENT",
                    description: Some("Independent mode."),
                    value: 0,
                },
                EnumVariant {
                    name: "REGULARINJECTED",
                    description: Some("Combined regular simultaneous + injected simultaneous mode"),
                    value: 1,
                },
                EnumVariant {
                    name: "REGULARALTERNATETRIGGER",
                    description: Some("Combined regular simultaneous + alternate trigger mode"),
                    value: 2,
                },
                EnumVariant {
                    name: "INJECTEDFASTINTERLEAVED",
                    description: Some("Combined injected simultaneous + fast interleaved mode"),
                    value: 3,
                },
                EnumVariant {
                    name: "INJECTEDSLOWINTERLEAVED",
                    description: Some("Combined injected simultaneous + slow Interleaved mode"),
                    value: 4,
                },
                EnumVariant {
                    name: "INJECTED",
                    description: Some("Injected simultaneous mode only"),
                    value: 5,
                },
                EnumVariant {
                    name: "REGULAR",
                    description: Some("Regular simultaneous mode only"),
                    value: 6,
                },
                EnumVariant {
                    name: "FASTINTERLEAVED",
                    description: Some("Fast interleaved mode only"),
                    value: 7,
                },
                EnumVariant {
                    name: "SLOWINTERLEAVED",
                    description: Some("Slow interleaved mode only"),
                    value: 8,
                },
                EnumVariant {
                    name: "ALTERNATETRIGGER",
                    description: Some("Alternate trigger mode only"),
                    value: 9,
                },
            ],
        },
    ],
};
