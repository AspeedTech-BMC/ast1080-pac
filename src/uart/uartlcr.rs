#[doc = "Register `UARTLCR` reader"]
pub type R = crate::R<UartlcrSpec>;
#[doc = "Register `UARTLCR` writer"]
pub type W = crate::W<UartlcrSpec>;
#[doc = "Field `CLSSelectNumberOfBitsPerCharacter` reader - CLS: Select number of bits per character"]
pub type ClsselectNumberOfBitsPerCharacterR = crate::FieldReader;
#[doc = "Field `CLSSelectNumberOfBitsPerCharacter` writer - CLS: Select number of bits per character"]
pub type ClsselectNumberOfBitsPerCharacterW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `STOPNumberOfStopBitsTxted` reader - STOP: Number of stop bits transmitted"]
pub type StopnumberOfStopBitsTxtedR = crate::BitReader;
#[doc = "Field `STOPNumberOfStopBitsTxted` writer - STOP: Number of stop bits transmitted"]
pub type StopnumberOfStopBitsTxtedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PENEnblParityBit` reader - PEN: Enable parity bit"]
pub type PenenblParityBitR = crate::BitReader;
#[doc = "Field `PENEnblParityBit` writer - PEN: Enable parity bit"]
pub type PenenblParityBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPSParityModeSel` reader - EPS: Parity mode selection"]
pub type EpsparityModeSelR = crate::BitReader;
#[doc = "Field `EPSParityModeSel` writer - EPS: Parity mode selection"]
pub type EpsparityModeSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BreakCtrlBit` reader - Break Control bit."]
pub type BreakCtrlBitR = crate::BitReader;
#[doc = "Field `BreakCtrlBit` writer - Break Control bit."]
pub type BreakCtrlBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLABDivisorLatchAccessBit` reader - DLAB: Divisor latch access bit"]
pub type DlabdivisorLatchAccessBitR = crate::BitReader;
#[doc = "Field `DLABDivisorLatchAccessBit` writer - DLAB: Divisor latch access bit"]
pub type DlabdivisorLatchAccessBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:1 - CLS: Select number of bits per character"]
    #[inline(always)]
    pub fn clsselect_number_of_bits_per_character(&self) -> ClsselectNumberOfBitsPerCharacterR {
        ClsselectNumberOfBitsPerCharacterR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - STOP: Number of stop bits transmitted"]
    #[inline(always)]
    pub fn stopnumber_of_stop_bits_txted(&self) -> StopnumberOfStopBitsTxtedR {
        StopnumberOfStopBitsTxtedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PEN: Enable parity bit"]
    #[inline(always)]
    pub fn penenbl_parity_bit(&self) -> PenenblParityBitR {
        PenenblParityBitR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EPS: Parity mode selection"]
    #[inline(always)]
    pub fn epsparity_mode_sel(&self) -> EpsparityModeSelR {
        EpsparityModeSelR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Break Control bit."]
    #[inline(always)]
    pub fn break_ctrl_bit(&self) -> BreakCtrlBitR {
        BreakCtrlBitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DLAB: Divisor latch access bit"]
    #[inline(always)]
    pub fn dlabdivisor_latch_access_bit(&self) -> DlabdivisorLatchAccessBitR {
        DlabdivisorLatchAccessBitR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - CLS: Select number of bits per character"]
    #[inline(always)]
    pub fn clsselect_number_of_bits_per_character(
        &mut self,
    ) -> ClsselectNumberOfBitsPerCharacterW<UartlcrSpec> {
        ClsselectNumberOfBitsPerCharacterW::new(self, 0)
    }
    #[doc = "Bit 2 - STOP: Number of stop bits transmitted"]
    #[inline(always)]
    pub fn stopnumber_of_stop_bits_txted(&mut self) -> StopnumberOfStopBitsTxtedW<UartlcrSpec> {
        StopnumberOfStopBitsTxtedW::new(self, 2)
    }
    #[doc = "Bit 3 - PEN: Enable parity bit"]
    #[inline(always)]
    pub fn penenbl_parity_bit(&mut self) -> PenenblParityBitW<UartlcrSpec> {
        PenenblParityBitW::new(self, 3)
    }
    #[doc = "Bit 4 - EPS: Parity mode selection"]
    #[inline(always)]
    pub fn epsparity_mode_sel(&mut self) -> EpsparityModeSelW<UartlcrSpec> {
        EpsparityModeSelW::new(self, 4)
    }
    #[doc = "Bit 6 - Break Control bit."]
    #[inline(always)]
    pub fn break_ctrl_bit(&mut self) -> BreakCtrlBitW<UartlcrSpec> {
        BreakCtrlBitW::new(self, 6)
    }
    #[doc = "Bit 7 - DLAB: Divisor latch access bit"]
    #[inline(always)]
    pub fn dlabdivisor_latch_access_bit(&mut self) -> DlabdivisorLatchAccessBitW<UartlcrSpec> {
        DlabdivisorLatchAccessBitW::new(self, 7)
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
