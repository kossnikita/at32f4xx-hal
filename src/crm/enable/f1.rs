use super::*;

bus! {
    DMA1 => (AHB, 0),
    CRC => (AHB, 6),
}

#[cfg(feature = "dma2")]
bus! {
    DMA2 => (AHB, 1),
}

#[cfg(not(any(feature = "at32f421", feature = "at32f425")))]
bus! {
    GPIOA => (APB2, 2),
    GPIOB => (APB2, 3),
    GPIOC => (APB2, 4),
}

#[cfg(any(feature = "at32f421", feature = "at32f425"))]
bus! {
    GPIOA => (AHB, 17),
    GPIOB => (AHB, 18),
    GPIOC => (AHB, 19),
    GPIOF => (AHB, 22),
}

#[cfg(feature = "at32f425")]
bus! {
    GPIOD => (AHB, 20),
}

#[cfg(feature = "iomux")]
bus! {
    IOMUX => (APB2, 0),
}

#[cfg(not(any(feature = "at32f421", feature = "at32f425")))]
#[cfg(feature = "gpiod")]
bus! {
    GPIOD => (APB2, 5),
}

#[cfg(not(any(feature = "at32f421", feature = "at32f425")))]
#[cfg(feature = "gpioe")]
bus! {
    GPIOE => (APB2, 6),
}

#[cfg(not(any(feature = "at32f421", feature = "at32f425")))]
#[cfg(feature = "gpiof")]
bus! {
    GPIOF => (APB2, 7),
}

bus! {
    SPI1 => (APB2, 12),
    SPI2 => (APB1, 14),
}

bus! {
    I2C1 => (APB1, 21),
    I2C2 => (APB1, 22),
}

bus! {
    USART1 => (APB2, 14),
    USART2 => (APB1, 17),
}

#[cfg(feature = "usart3")]
bus! {
    USART3 => (APB1, 18),
}

#[cfg(feature = "uart4")]
bus! {
    UART4 => (APB1, 19),
    UART5 => (APB1, 20),
}

bus! {
    ADC1 => (APB2, 8),
}

#[cfg(feature = "tmr1")]
bus! {
    TMR1 => (APB2, 11),
}

#[cfg(feature = "tmr2")]
bus! {
    TMR2 => (APB1, 0),
}

#[cfg(feature = "tmr3")]
bus! {
    TMR3 => (APB1, 1),
}

#[cfg(feature = "tmr4")]
bus! {
    TMR4 => (APB1, 2),
}

#[cfg(feature = "tmr5")]
bus! {
    TMR5 => (APB1, 3),
}

#[cfg(feature = "tmr6")]
bus! {
    TMR6 => (APB1, 4),
}

#[cfg(feature = "tmr7")]
bus! {
    TMR7 => (APB1, 5),
}

#[cfg(feature = "tmr8")]
bus! {
    TMR8 => (APB2, 13),
}

#[cfg(feature = "tmr9")]
bus! {
    TMR9 => (APB2, 19),
}

#[cfg(feature = "tmr10")]
bus! {
    TMR10 => (APB2, 10),
}

#[cfg(feature = "tmr11")]
bus! {
    TMR11 => (APB2, 21),
}

#[cfg(feature = "tmr12")]
bus! {
    TMR12 => (APB1, 6),
}

#[cfg(feature = "tmr13")]
bus! {
    TMR13 => (APB1, 7),
}

#[cfg(feature = "tmr14")]
bus! {
    TMR14 => (APB1, 8),
}

#[cfg(feature = "tmr15")]
bus! {
    TMR15 => (APB2, 16),
}

#[cfg(feature = "tmr16")]
bus! {
    TMR16 => (APB2, 17),
}

#[cfg(feature = "tmr17")]
bus! {
    TMR17 => (APB2, 18),
}

#[cfg(feature = "tmr20")]
bus! {
    TMR20 => (APB2, 11),
}
