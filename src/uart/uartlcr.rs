#[doc = "Register `UARTLCR` reader"]
pub type R = crate::R<UartlcrSpec>;
#[doc = "Register `UARTLCR` writer"]
pub type W = crate::W<UartlcrSpec>;
#[doc = "Field `CLS` reader - Select number of bits per character"]
pub type ClsR = crate::FieldReader;
#[doc = "Field `CLS` writer - Select number of bits per character"]
pub type ClsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `STOP` reader - Number of stop bits transmitted"]
pub type StopR = crate::BitReader;
#[doc = "Field `STOP` writer - Number of stop bits transmitted"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN` reader - Enable parity bit"]
pub type PenR = crate::BitReader;
#[doc = "Field `PEN` writer - Enable parity bit"]
pub type PenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPS` reader - Parity mode selection"]
pub type EpsR = crate::BitReader;
#[doc = "Field `EPS` writer - Parity mode selection"]
pub type EpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BreakCtrlBit` reader - Break Control bit."]
pub type BreakCtrlBitR = crate::BitReader;
#[doc = "Field `BreakCtrlBit` writer - Break Control bit."]
pub type BreakCtrlBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLAB` reader - Divisor latch access bit"]
pub type DlabR = crate::BitReader;
#[doc = "Field `DLAB` writer - Divisor latch access bit"]
pub type DlabW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Select number of bits per character"]
    #[inline(always)]
    pub fn cls(&self) -> ClsR {
        ClsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Number of stop bits transmitted"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable parity bit"]
    #[inline(always)]
    pub fn pen(&self) -> PenR {
        PenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Parity mode selection"]
    #[inline(always)]
    pub fn eps(&self) -> EpsR {
        EpsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Break Control bit."]
    #[inline(always)]
    pub fn break_ctrl_bit(&self) -> BreakCtrlBitR {
        BreakCtrlBitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Divisor latch access bit"]
    #[inline(always)]
    pub fn dlab(&self) -> DlabR {
        DlabR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Select number of bits per character"]
    #[inline(always)]
    pub fn cls(&mut self) -> ClsW<UartlcrSpec> {
        ClsW::new(self, 0)
    }
    #[doc = "Bit 2 - Number of stop bits transmitted"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<UartlcrSpec> {
        StopW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable parity bit"]
    #[inline(always)]
    pub fn pen(&mut self) -> PenW<UartlcrSpec> {
        PenW::new(self, 3)
    }
    #[doc = "Bit 4 - Parity mode selection"]
    #[inline(always)]
    pub fn eps(&mut self) -> EpsW<UartlcrSpec> {
        EpsW::new(self, 4)
    }
    #[doc = "Bit 6 - Break Control bit."]
    #[inline(always)]
    pub fn break_ctrl_bit(&mut self) -> BreakCtrlBitW<UartlcrSpec> {
        BreakCtrlBitW::new(self, 6)
    }
    #[doc = "Bit 7 - Divisor latch access bit"]
    #[inline(always)]
    pub fn dlab(&mut self) -> DlabW<UartlcrSpec> {
        DlabW::new(self, 7)
    }
}
#[doc = "Line Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartlcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartlcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartlcrSpec;
impl crate::RegisterSpec for UartlcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartlcr::R`](R) reader structure"]
impl crate::Readable for UartlcrSpec {}
#[doc = "`write(|w| ..)` method takes [`uartlcr::W`](W) writer structure"]
impl crate::Writable for UartlcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTLCR to value 0"]
impl crate::Resettable for UartlcrSpec {}
