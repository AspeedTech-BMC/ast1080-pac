#[doc = "Register `JTAG034` reader"]
pub type R = crate::R<Jtag034Spec>;
#[doc = "Register `JTAG034` writer"]
pub type W = crate::W<Jtag034Spec>;
#[doc = "Field `ClkDivisor` reader - Clock divisor"]
pub type ClkDivisorR = crate::FieldReader<u16>;
#[doc = "Field `ClkDivisor` writer - Clock divisor"]
pub type ClkDivisorW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TckState` reader - Tck state"]
pub type TckStateR = crate::BitReader;
#[doc = "Field `TckState` writer - Tck state"]
pub type TckStateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRSTValue` reader - TRST value"]
pub type TrstvalueR = crate::BitReader;
#[doc = "Field `TRSTValue` writer - TRST value"]
pub type TrstvalueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `StaticShiftValue` reader - Static Shift Value"]
pub type StaticShiftValueR = crate::BitReader;
#[doc = "Field `StaticShiftValue` writer - Static Shift Value"]
pub type StaticShiftValueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `UpperDataShiftNumber` reader - Upper Data Shift Number."]
pub type UpperDataShiftNumberR = crate::FieldReader;
#[doc = "Field `UpperDataShiftNumber` writer - Upper Data Shift Number."]
pub type UpperDataShiftNumberW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Internal FIFO mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntFifomode {
    #[doc = "0: CPU mode"]
    CpuMode = 0,
    #[doc = "1: Controller mode"]
    ControllerMode = 1,
}
impl From<IntFifomode> for bool {
    #[inline(always)]
    fn from(variant: IntFifomode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IntFIFOMode` reader - Internal FIFO mode"]
pub type IntFifomodeR = crate::BitReader<IntFifomode>;
impl IntFifomodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntFifomode {
        match self.bits {
            false => IntFifomode::CpuMode,
            true => IntFifomode::ControllerMode,
        }
    }
    #[doc = "CPU mode"]
    #[inline(always)]
    pub fn is_cpu_mode(&self) -> bool {
        *self == IntFifomode::CpuMode
    }
    #[doc = "Controller mode"]
    #[inline(always)]
    pub fn is_controller_mode(&self) -> bool {
        *self == IntFifomode::ControllerMode
    }
}
#[doc = "Field `RstIntFIFO` reader - Reset Internal FIFO"]
pub type RstIntFifoR = crate::BitReader;
#[doc = "Field `RstIntFIFO` writer - Reset Internal FIFO"]
pub type RstIntFifoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ForceCtrlAndSlaveIntoRstStateByTMS` reader - Force controller and slave into Reset state by TMS."]
pub type ForceCtrlAndSlaveIntoRstStateByTmsR = crate::BitReader;
#[doc = "Field `ForceCtrlAndSlaveIntoRstStateByTMS` writer - Force controller and slave into Reset state by TMS."]
pub type ForceCtrlAndSlaveIntoRstStateByTmsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Engine output enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EngOutputEnbl {
    #[doc = "1: output enable."]
    OutputEnable = 1,
    #[doc = "0: output disable."]
    OutputDisable = 0,
}
impl From<EngOutputEnbl> for bool {
    #[inline(always)]
    fn from(variant: EngOutputEnbl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EngOutputEnbl` reader - Engine output enable."]
pub type EngOutputEnblR = crate::BitReader<EngOutputEnbl>;
impl EngOutputEnblR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EngOutputEnbl {
        match self.bits {
            true => EngOutputEnbl::OutputEnable,
            false => EngOutputEnbl::OutputDisable,
        }
    }
    #[doc = "output enable."]
    #[inline(always)]
    pub fn is_output_enable(&self) -> bool {
        *self == EngOutputEnbl::OutputEnable
    }
    #[doc = "output disable."]
    #[inline(always)]
    pub fn is_output_disable(&self) -> bool {
        *self == EngOutputEnbl::OutputDisable
    }
}
#[doc = "Field `EngOutputEnbl` writer - Engine output enable."]
pub type EngOutputEnblW<'a, REG> = crate::BitWriter<'a, REG, EngOutputEnbl>;
impl<'a, REG> EngOutputEnblW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "output enable."]
    #[inline(always)]
    pub fn output_enable(self) -> &'a mut crate::W<REG> {
        self.variant(EngOutputEnbl::OutputEnable)
    }
    #[doc = "output disable."]
    #[inline(always)]
    pub fn output_disable(self) -> &'a mut crate::W<REG> {
        self.variant(EngOutputEnbl::OutputDisable)
    }
}
#[doc = "Field `EngAndModeEnbl` reader - Engine and Mode Enable"]
pub type EngAndModeEnblR = crate::BitReader;
#[doc = "Field `EngAndModeEnbl` writer - Engine and Mode Enable"]
pub type EngAndModeEnblW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - Clock divisor"]
    #[inline(always)]
    pub fn clk_divisor(&self) -> ClkDivisorR {
        ClkDivisorR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:14 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 12 - Tck state"]
    #[inline(always)]
    pub fn tck_state(&self) -> TckStateR {
        TckStateR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - TRST value"]
    #[inline(always)]
    pub fn trstvalue(&self) -> TrstvalueR {
        TrstvalueR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Static Shift Value"]
    #[inline(always)]
    pub fn static_shift_value(&self) -> StaticShiftValueR {
        StaticShiftValueR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Upper Data Shift Number."]
    #[inline(always)]
    pub fn upper_data_shift_number(&self) -> UpperDataShiftNumberR {
        UpperDataShiftNumberR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Internal FIFO mode"]
    #[inline(always)]
    pub fn int_fifomode(&self) -> IntFifomodeR {
        IntFifomodeR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reset Internal FIFO"]
    #[inline(always)]
    pub fn rst_int_fifo(&self) -> RstIntFifoR {
        RstIntFifoR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 29 - Force controller and slave into Reset state by TMS."]
    #[inline(always)]
    pub fn force_ctrl_and_slave_into_rst_state_by_tms(
        &self,
    ) -> ForceCtrlAndSlaveIntoRstStateByTmsR {
        ForceCtrlAndSlaveIntoRstStateByTmsR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Engine output enable."]
    #[inline(always)]
    pub fn eng_output_enbl(&self) -> EngOutputEnblR {
        EngOutputEnblR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Engine and Mode Enable"]
    #[inline(always)]
    pub fn eng_and_mode_enbl(&self) -> EngAndModeEnblR {
        EngAndModeEnblR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Clock divisor"]
    #[inline(always)]
    pub fn clk_divisor(&mut self) -> ClkDivisorW<Jtag034Spec> {
        ClkDivisorW::new(self, 0)
    }
    #[doc = "Bits 12:14 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Jtag034Spec> {
        Reserved3W::new(self, 12)
    }
    #[doc = "Bit 12 - Tck state"]
    #[inline(always)]
    pub fn tck_state(&mut self) -> TckStateW<Jtag034Spec> {
        TckStateW::new(self, 12)
    }
    #[doc = "Bit 15 - TRST value"]
    #[inline(always)]
    pub fn trstvalue(&mut self) -> TrstvalueW<Jtag034Spec> {
        TrstvalueW::new(self, 15)
    }
    #[doc = "Bit 16 - Static Shift Value"]
    #[inline(always)]
    pub fn static_shift_value(&mut self) -> StaticShiftValueW<Jtag034Spec> {
        StaticShiftValueW::new(self, 16)
    }
    #[doc = "Bits 17:19 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Jtag034Spec> {
        Reserved2W::new(self, 17)
    }
    #[doc = "Bits 20:22 - Upper Data Shift Number."]
    #[inline(always)]
    pub fn upper_data_shift_number(&mut self) -> UpperDataShiftNumberW<Jtag034Spec> {
        UpperDataShiftNumberW::new(self, 20)
    }
    #[doc = "Bit 23 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Jtag034Spec> {
        Reserved1W::new(self, 23)
    }
    #[doc = "Bit 25 - Reset Internal FIFO"]
    #[inline(always)]
    pub fn rst_int_fifo(&mut self) -> RstIntFifoW<Jtag034Spec> {
        RstIntFifoW::new(self, 25)
    }
    #[doc = "Bit 29 - Force controller and slave into Reset state by TMS."]
    #[inline(always)]
    pub fn force_ctrl_and_slave_into_rst_state_by_tms(
        &mut self,
    ) -> ForceCtrlAndSlaveIntoRstStateByTmsW<Jtag034Spec> {
        ForceCtrlAndSlaveIntoRstStateByTmsW::new(self, 29)
    }
    #[doc = "Bit 30 - Engine output enable."]
    #[inline(always)]
    pub fn eng_output_enbl(&mut self) -> EngOutputEnblW<Jtag034Spec> {
        EngOutputEnblW::new(self, 30)
    }
    #[doc = "Bit 31 - Engine and Mode Enable"]
    #[inline(always)]
    pub fn eng_and_mode_enbl(&mut self) -> EngAndModeEnblW<Jtag034Spec> {
        EngAndModeEnblW::new(self, 31)
    }
}
#[doc = "Global control\n\nYou can [`read`](crate::Reg::read) this register and get [`jtag034::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jtag034::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Jtag034Spec;
impl crate::RegisterSpec for Jtag034Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jtag034::R`](R) reader structure"]
impl crate::Readable for Jtag034Spec {}
#[doc = "`write(|w| ..)` method takes [`jtag034::W`](W) writer structure"]
impl crate::Writable for Jtag034Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets JTAG034 to value 0x8007"]
impl crate::Resettable for Jtag034Spec {
    const RESET_VALUE: u32 = 0x8007;
}
