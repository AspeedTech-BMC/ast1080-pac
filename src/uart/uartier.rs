#[doc = "Register `UARTIER` reader"]
pub type R = crate::R<UartierSpec>;
#[doc = "Register `UARTIER` writer"]
pub type W = crate::W<UartierSpec>;
#[doc = "ERBFI: Enable Received Data Available Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErbfienblRxdDataAvailableInt {
    #[doc = "0: Disable interrupt"]
    DisableInterrupt = 0,
    #[doc = "1: Enable interrupt"]
    EnableInterrupt = 1,
}
impl From<ErbfienblRxdDataAvailableInt> for bool {
    #[inline(always)]
    fn from(variant: ErbfienblRxdDataAvailableInt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERBFIEnblRxdDataAvailableINT` reader - ERBFI: Enable Received Data Available Interrupt"]
pub type ErbfienblRxdDataAvailableIntR = crate::BitReader<ErbfienblRxdDataAvailableInt>;
impl ErbfienblRxdDataAvailableIntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ErbfienblRxdDataAvailableInt {
        match self.bits {
            false => ErbfienblRxdDataAvailableInt::DisableInterrupt,
            true => ErbfienblRxdDataAvailableInt::EnableInterrupt,
        }
    }
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn is_disable_interrupt(&self) -> bool {
        *self == ErbfienblRxdDataAvailableInt::DisableInterrupt
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn is_enable_interrupt(&self) -> bool {
        *self == ErbfienblRxdDataAvailableInt::EnableInterrupt
    }
}
#[doc = "Field `ERBFIEnblRxdDataAvailableINT` writer - ERBFI: Enable Received Data Available Interrupt"]
pub type ErbfienblRxdDataAvailableIntW<'a, REG> =
    crate::BitWriter<'a, REG, ErbfienblRxdDataAvailableInt>;
impl<'a, REG> ErbfienblRxdDataAvailableIntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable_interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(ErbfienblRxdDataAvailableInt::DisableInterrupt)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable_interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(ErbfienblRxdDataAvailableInt::EnableInterrupt)
    }
}
#[doc = "ETBEI: Enable Transmitter Holding Register Empty Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EtbeienblTxterHoldingRegEmptyInt {
    #[doc = "0: Disable interrupt"]
    DisableInterrupt = 0,
    #[doc = "1: Enable interrupt"]
    EnableInterrupt = 1,
}
impl From<EtbeienblTxterHoldingRegEmptyInt> for bool {
    #[inline(always)]
    fn from(variant: EtbeienblTxterHoldingRegEmptyInt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETBEIEnblTxterHoldingRegEmptyINT` reader - ETBEI: Enable Transmitter Holding Register Empty Interrupt"]
pub type EtbeienblTxterHoldingRegEmptyIntR = crate::BitReader<EtbeienblTxterHoldingRegEmptyInt>;
impl EtbeienblTxterHoldingRegEmptyIntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EtbeienblTxterHoldingRegEmptyInt {
        match self.bits {
            false => EtbeienblTxterHoldingRegEmptyInt::DisableInterrupt,
            true => EtbeienblTxterHoldingRegEmptyInt::EnableInterrupt,
        }
    }
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn is_disable_interrupt(&self) -> bool {
        *self == EtbeienblTxterHoldingRegEmptyInt::DisableInterrupt
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn is_enable_interrupt(&self) -> bool {
        *self == EtbeienblTxterHoldingRegEmptyInt::EnableInterrupt
    }
}
#[doc = "Field `ETBEIEnblTxterHoldingRegEmptyINT` writer - ETBEI: Enable Transmitter Holding Register Empty Interrupt"]
pub type EtbeienblTxterHoldingRegEmptyIntW<'a, REG> =
    crate::BitWriter<'a, REG, EtbeienblTxterHoldingRegEmptyInt>;
impl<'a, REG> EtbeienblTxterHoldingRegEmptyIntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable_interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(EtbeienblTxterHoldingRegEmptyInt::DisableInterrupt)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable_interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(EtbeienblTxterHoldingRegEmptyInt::EnableInterrupt)
    }
}
#[doc = "ELSI: Enable Receiver Line Status Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ElsienblRxrLineStatusInt {
    #[doc = "0: Disable interrupt"]
    DisableInterrupt = 0,
    #[doc = "1: Enable interrupt"]
    EnableInterrupt = 1,
}
impl From<ElsienblRxrLineStatusInt> for bool {
    #[inline(always)]
    fn from(variant: ElsienblRxrLineStatusInt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ELSIEnblRxrLineStatusINT` reader - ELSI: Enable Receiver Line Status Interrupt"]
pub type ElsienblRxrLineStatusIntR = crate::BitReader<ElsienblRxrLineStatusInt>;
impl ElsienblRxrLineStatusIntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ElsienblRxrLineStatusInt {
        match self.bits {
            false => ElsienblRxrLineStatusInt::DisableInterrupt,
            true => ElsienblRxrLineStatusInt::EnableInterrupt,
        }
    }
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn is_disable_interrupt(&self) -> bool {
        *self == ElsienblRxrLineStatusInt::DisableInterrupt
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn is_enable_interrupt(&self) -> bool {
        *self == ElsienblRxrLineStatusInt::EnableInterrupt
    }
}
#[doc = "Field `ELSIEnblRxrLineStatusINT` writer - ELSI: Enable Receiver Line Status Interrupt"]
pub type ElsienblRxrLineStatusIntW<'a, REG> = crate::BitWriter<'a, REG, ElsienblRxrLineStatusInt>;
impl<'a, REG> ElsienblRxrLineStatusIntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable_interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(ElsienblRxrLineStatusInt::DisableInterrupt)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable_interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(ElsienblRxrLineStatusInt::EnableInterrupt)
    }
}
#[doc = "EDSSI: Enable Modem Status Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EdssienblModemStatusInt {
    #[doc = "0: Disable interrupt"]
    DisableInterrupt = 0,
    #[doc = "1: Enable interrupt"]
    EnableInterrupt = 1,
}
impl From<EdssienblModemStatusInt> for bool {
    #[inline(always)]
    fn from(variant: EdssienblModemStatusInt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDSSIEnblModemStatusINT` reader - EDSSI: Enable Modem Status Interrupt"]
pub type EdssienblModemStatusIntR = crate::BitReader<EdssienblModemStatusInt>;
impl EdssienblModemStatusIntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EdssienblModemStatusInt {
        match self.bits {
            false => EdssienblModemStatusInt::DisableInterrupt,
            true => EdssienblModemStatusInt::EnableInterrupt,
        }
    }
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn is_disable_interrupt(&self) -> bool {
        *self == EdssienblModemStatusInt::DisableInterrupt
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn is_enable_interrupt(&self) -> bool {
        *self == EdssienblModemStatusInt::EnableInterrupt
    }
}
#[doc = "Field `EDSSIEnblModemStatusINT` writer - EDSSI: Enable Modem Status Interrupt"]
pub type EdssienblModemStatusIntW<'a, REG> = crate::BitWriter<'a, REG, EdssienblModemStatusInt>;
impl<'a, REG> EdssienblModemStatusIntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable_interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(EdssienblModemStatusInt::DisableInterrupt)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable_interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(EdssienblModemStatusInt::EnableInterrupt)
    }
}
#[doc = "Field `Reserved01` reader - Reserved (0)"]
pub type Reserved01R = crate::FieldReader;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - ERBFI: Enable Received Data Available Interrupt"]
    #[inline(always)]
    pub fn erbfienbl_rxd_data_available_int(&self) -> ErbfienblRxdDataAvailableIntR {
        ErbfienblRxdDataAvailableIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ETBEI: Enable Transmitter Holding Register Empty Interrupt"]
    #[inline(always)]
    pub fn etbeienbl_txter_holding_reg_empty_int(&self) -> EtbeienblTxterHoldingRegEmptyIntR {
        EtbeienblTxterHoldingRegEmptyIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ELSI: Enable Receiver Line Status Interrupt"]
    #[inline(always)]
    pub fn elsienbl_rxr_line_status_int(&self) -> ElsienblRxrLineStatusIntR {
        ElsienblRxrLineStatusIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EDSSI: Enable Modem Status Interrupt"]
    #[inline(always)]
    pub fn edssienbl_modem_status_int(&self) -> EdssienblModemStatusIntR {
        EdssienblModemStatusIntR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - ERBFI: Enable Received Data Available Interrupt"]
    #[inline(always)]
    pub fn erbfienbl_rxd_data_available_int(
        &mut self,
    ) -> ErbfienblRxdDataAvailableIntW<UartierSpec> {
        ErbfienblRxdDataAvailableIntW::new(self, 0)
    }
    #[doc = "Bit 1 - ETBEI: Enable Transmitter Holding Register Empty Interrupt"]
    #[inline(always)]
    pub fn etbeienbl_txter_holding_reg_empty_int(
        &mut self,
    ) -> EtbeienblTxterHoldingRegEmptyIntW<UartierSpec> {
        EtbeienblTxterHoldingRegEmptyIntW::new(self, 1)
    }
    #[doc = "Bit 2 - ELSI: Enable Receiver Line Status Interrupt"]
    #[inline(always)]
    pub fn elsienbl_rxr_line_status_int(&mut self) -> ElsienblRxrLineStatusIntW<UartierSpec> {
        ElsienblRxrLineStatusIntW::new(self, 2)
    }
    #[doc = "Bit 3 - EDSSI: Enable Modem Status Interrupt"]
    #[inline(always)]
    pub fn edssienbl_modem_status_int(&mut self) -> EdssienblModemStatusIntW<UartierSpec> {
        EdssienblModemStatusIntW::new(self, 3)
    }
}
#[doc = "Interrupt Enable Register (DLAB = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`uartier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartierSpec;
impl crate::RegisterSpec for UartierSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartier::R`](R) reader structure"]
impl crate::Readable for UartierSpec {}
#[doc = "`write(|w| ..)` method takes [`uartier::W`](W) writer structure"]
impl crate::Writable for UartierSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTIER to value 0"]
impl crate::Resettable for UartierSpec {}
