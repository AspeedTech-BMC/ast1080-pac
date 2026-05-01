#[doc = "Register `SGPIO_%s_ControlRegister` reader"]
pub type R = crate::R<Sgpio_ControlRegisterSpec>;
#[doc = "Register `SGPIO_%s_ControlRegister` writer"]
pub type W = crate::W<Sgpio_ControlRegisterSpec>;
#[doc = "Field `SerialOutputCsrValue` reader - Serial output csr value"]
pub type SerialOutputCsrValueR = crate::BitReader;
#[doc = "Field `SerialOutputCsrValue` writer - Serial output csr value"]
pub type SerialOutputCsrValueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ParallelOutputCsrValue` reader - Parallel output csr value"]
pub type ParallelOutputCsrValueR = crate::BitReader;
#[doc = "Field `ParallelOutputCsrValue` writer - Parallel output csr value"]
pub type ParallelOutputCsrValueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTCtrl` reader - Interrupt control"]
pub type IntctrlR = crate::FieldReader;
#[doc = "Field `INTCtrl` writer - Interrupt control"]
pub type IntctrlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RstPolarity` reader - Reset Polarity"]
pub type RstPolarityR = crate::BitReader;
#[doc = "Field `RstPolarity` writer - Reset Polarity"]
pub type RstPolarityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `InputMask` reader - Input mask"]
pub type InputMaskR = crate::BitReader;
#[doc = "Field `InputMask` writer - Input mask"]
pub type InputMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INTSts` reader - Interrupt status"]
pub type IntstsR = crate::BitReader;
#[doc = "Field `INTSts` writer - Interrupt status"]
pub type IntstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SerialPinInputValue` reader - Serial pin input value"]
pub type SerialPinInputValueR = crate::BitReader;
#[doc = "Field `SerialPinInputValue` writer - Serial pin input value"]
pub type SerialPinInputValueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ParallleInputValue` reader - Parallle input value"]
pub type ParallleInputValueR = crate::BitReader;
#[doc = "Field `ParallleInputValue` writer - Parallle input value"]
pub type ParallleInputValueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SerialOutputSel` reader - Serial output selection"]
pub type SerialOutputSelR = crate::FieldReader;
#[doc = "Field `SerialOutputSel` writer - Serial output selection"]
pub type SerialOutputSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ParallelOutputSel` reader - Parallel output selection"]
pub type ParallelOutputSelR = crate::FieldReader;
#[doc = "Field `ParallelOutputSel` writer - Parallel output selection"]
pub type ParallelOutputSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WrProtRstTolerance1CanBeResetByWDT0CanOnlyBeResetByPOR` reader - Write protection reset tolerance. 1: can be reset by WDT, 0: can only be reset by POR."]
pub type WrProtRstTolerance1canBeResetByWdt0canOnlyBeResetByPorR = crate::BitReader;
#[doc = "Field `WrProtRstTolerance1CanBeResetByWDT0CanOnlyBeResetByPOR` writer - Write protection reset tolerance. 1: can be reset by WDT, 0: can only be reset by POR."]
pub type WrProtRstTolerance1canBeResetByWdt0canOnlyBeResetByPorW<'a, REG> =
    crate::BitWriter<'a, REG>;
#[doc = "Field `WrProt` reader - Write protection"]
pub type WrProtR = crate::BitReader;
#[doc = "Field `WrProt` writer - Write protection"]
pub type WrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Serial output csr value"]
    #[inline(always)]
    pub fn serial_output_csr_value(&self) -> SerialOutputCsrValueR {
        SerialOutputCsrValueR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parallel output csr value"]
    #[inline(always)]
    pub fn parallel_output_csr_value(&self) -> ParallelOutputCsrValueR {
        ParallelOutputCsrValueR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - Interrupt control"]
    #[inline(always)]
    pub fn intctrl(&self) -> IntctrlR {
        IntctrlR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Reset Polarity"]
    #[inline(always)]
    pub fn rst_polarity(&self) -> RstPolarityR {
        RstPolarityR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - Input mask"]
    #[inline(always)]
    pub fn input_mask(&self) -> InputMaskR {
        InputMaskR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Interrupt status"]
    #[inline(always)]
    pub fn intsts(&self) -> IntstsR {
        IntstsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Serial pin input value"]
    #[inline(always)]
    pub fn serial_pin_input_value(&self) -> SerialPinInputValueR {
        SerialPinInputValueR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Parallle input value"]
    #[inline(always)]
    pub fn parallle_input_value(&self) -> ParallleInputValueR {
        ParallleInputValueR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Serial output selection"]
    #[inline(always)]
    pub fn serial_output_sel(&self) -> SerialOutputSelR {
        SerialOutputSelR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Parallel output selection"]
    #[inline(always)]
    pub fn parallel_output_sel(&self) -> ParallelOutputSelR {
        ParallelOutputSelR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 30 - Write protection reset tolerance. 1: can be reset by WDT, 0: can only be reset by POR."]
    #[inline(always)]
    pub fn wr_prot_rst_tolerance1can_be_reset_by_wdt0can_only_be_reset_by_por(
        &self,
    ) -> WrProtRstTolerance1canBeResetByWdt0canOnlyBeResetByPorR {
        WrProtRstTolerance1canBeResetByWdt0canOnlyBeResetByPorR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Write protection"]
    #[inline(always)]
    pub fn wr_prot(&self) -> WrProtR {
        WrProtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Serial output csr value"]
    #[inline(always)]
    pub fn serial_output_csr_value(&mut self) -> SerialOutputCsrValueW<Sgpio_ControlRegisterSpec> {
        SerialOutputCsrValueW::new(self, 0)
    }
    #[doc = "Bit 1 - Parallel output csr value"]
    #[inline(always)]
    pub fn parallel_output_csr_value(
        &mut self,
    ) -> ParallelOutputCsrValueW<Sgpio_ControlRegisterSpec> {
        ParallelOutputCsrValueW::new(self, 1)
    }
    #[doc = "Bits 2:5 - Interrupt control"]
    #[inline(always)]
    pub fn intctrl(&mut self) -> IntctrlW<Sgpio_ControlRegisterSpec> {
        IntctrlW::new(self, 2)
    }
    #[doc = "Bit 6 - Reset Polarity"]
    #[inline(always)]
    pub fn rst_polarity(&mut self) -> RstPolarityW<Sgpio_ControlRegisterSpec> {
        RstPolarityW::new(self, 6)
    }
    #[doc = "Bits 7:8 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Sgpio_ControlRegisterSpec> {
        Reserved3W::new(self, 7)
    }
    #[doc = "Bit 9 - Input mask"]
    #[inline(always)]
    pub fn input_mask(&mut self) -> InputMaskW<Sgpio_ControlRegisterSpec> {
        InputMaskW::new(self, 9)
    }
    #[doc = "Bits 10:11 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Sgpio_ControlRegisterSpec> {
        Reserved2W::new(self, 10)
    }
    #[doc = "Bit 12 - Interrupt status"]
    #[inline(always)]
    pub fn intsts(&mut self) -> IntstsW<Sgpio_ControlRegisterSpec> {
        IntstsW::new(self, 12)
    }
    #[doc = "Bit 13 - Serial pin input value"]
    #[inline(always)]
    pub fn serial_pin_input_value(&mut self) -> SerialPinInputValueW<Sgpio_ControlRegisterSpec> {
        SerialPinInputValueW::new(self, 13)
    }
    #[doc = "Bit 14 - Parallle input value"]
    #[inline(always)]
    pub fn parallle_input_value(&mut self) -> ParallleInputValueW<Sgpio_ControlRegisterSpec> {
        ParallleInputValueW::new(self, 14)
    }
    #[doc = "Bit 15 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Sgpio_ControlRegisterSpec> {
        Reserved1W::new(self, 15)
    }
    #[doc = "Bits 16:17 - Serial output selection"]
    #[inline(always)]
    pub fn serial_output_sel(&mut self) -> SerialOutputSelW<Sgpio_ControlRegisterSpec> {
        SerialOutputSelW::new(self, 16)
    }
    #[doc = "Bits 18:19 - Parallel output selection"]
    #[inline(always)]
    pub fn parallel_output_sel(&mut self) -> ParallelOutputSelW<Sgpio_ControlRegisterSpec> {
        ParallelOutputSelW::new(self, 18)
    }
    #[doc = "Bit 30 - Write protection reset tolerance. 1: can be reset by WDT, 0: can only be reset by POR."]
    #[inline(always)]
    pub fn wr_prot_rst_tolerance1can_be_reset_by_wdt0can_only_be_reset_by_por(
        &mut self,
    ) -> WrProtRstTolerance1canBeResetByWdt0canOnlyBeResetByPorW<Sgpio_ControlRegisterSpec> {
        WrProtRstTolerance1canBeResetByWdt0canOnlyBeResetByPorW::new(self, 30)
    }
    #[doc = "Bit 31 - Write protection"]
    #[inline(always)]
    pub fn wr_prot(&mut self) -> WrProtW<Sgpio_ControlRegisterSpec> {
        WrProtW::new(self, 31)
    }
}
#[doc = "SGPIO\\_N Control Register N=0~255\n\nYou can [`read`](crate::Reg::read) this register and get [`sgpio__control_register::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgpio__control_register::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sgpio_ControlRegisterSpec;
impl crate::RegisterSpec for Sgpio_ControlRegisterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sgpio__control_register::R`](R) reader structure"]
impl crate::Readable for Sgpio_ControlRegisterSpec {}
#[doc = "`write(|w| ..)` method takes [`sgpio__control_register::W`](W) writer structure"]
impl crate::Writable for Sgpio_ControlRegisterSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SGPIO_%s_ControlRegister to value 0"]
impl crate::Resettable for Sgpio_ControlRegisterSpec {}
