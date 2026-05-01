#[doc = "Register `JTAG008` reader"]
pub type R = crate::R<Jtag008Spec>;
#[doc = "Register `JTAG008` writer"]
pub type W = crate::W<Jtag008Spec>;
#[doc = "Field `EnblTransmissionOfData` reader - Enable transmission of data."]
pub type EnblTransmissionOfDataR = crate::BitReader;
#[doc = "Field `EnblTransmissionOfData` writer - Enable transmission of data."]
pub type EnblTransmissionOfDataW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblTransmissionOfInstruction` reader - Enable transmission of instruction."]
pub type EnblTransmissionOfInstructionR = crate::BitReader;
#[doc = "Field `EnblTransmissionOfInstruction` writer - Enable transmission of instruction."]
pub type EnblTransmissionOfInstructionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Last transmission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LastTransmission {
    #[doc = "1: last transmission."]
    LastTransmission = 1,
    #[doc = "0: more data waited."]
    MoreDataWaited = 0,
}
impl From<LastTransmission> for bool {
    #[inline(always)]
    fn from(variant: LastTransmission) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LastTransmission` reader - Last transmission"]
pub type LastTransmissionR = crate::BitReader<LastTransmission>;
impl LastTransmissionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LastTransmission {
        match self.bits {
            true => LastTransmission::LastTransmission,
            false => LastTransmission::MoreDataWaited,
        }
    }
    #[doc = "last transmission."]
    #[inline(always)]
    pub fn is_last_transmission(&self) -> bool {
        *self == LastTransmission::LastTransmission
    }
    #[doc = "more data waited."]
    #[inline(always)]
    pub fn is_more_data_waited(&self) -> bool {
        *self == LastTransmission::MoreDataWaited
    }
}
#[doc = "Field `LastTransmission` writer - Last transmission"]
pub type LastTransmissionW<'a, REG> = crate::BitWriter<'a, REG, LastTransmission>;
impl<'a, REG> LastTransmissionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "last transmission."]
    #[inline(always)]
    pub fn last_transmission(self) -> &'a mut crate::W<REG> {
        self.variant(LastTransmission::LastTransmission)
    }
    #[doc = "more data waited."]
    #[inline(always)]
    pub fn more_data_waited(self) -> &'a mut crate::W<REG> {
        self.variant(LastTransmission::MoreDataWaited)
    }
}
#[doc = "Terminating transmission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TerminatingTransmission {
    #[doc = "1: terminate."]
    Terminate = 1,
    #[doc = "0: normal operation."]
    NormalOperation = 0,
}
impl From<TerminatingTransmission> for bool {
    #[inline(always)]
    fn from(variant: TerminatingTransmission) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TerminatingTransmission` reader - Terminating transmission"]
pub type TerminatingTransmissionR = crate::BitReader<TerminatingTransmission>;
impl TerminatingTransmissionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TerminatingTransmission {
        match self.bits {
            true => TerminatingTransmission::Terminate,
            false => TerminatingTransmission::NormalOperation,
        }
    }
    #[doc = "terminate."]
    #[inline(always)]
    pub fn is_terminate(&self) -> bool {
        *self == TerminatingTransmission::Terminate
    }
    #[doc = "normal operation."]
    #[inline(always)]
    pub fn is_normal_operation(&self) -> bool {
        *self == TerminatingTransmission::NormalOperation
    }
}
#[doc = "Field `TerminatingTransmission` writer - Terminating transmission"]
pub type TerminatingTransmissionW<'a, REG> = crate::BitWriter<'a, REG, TerminatingTransmission>;
impl<'a, REG> TerminatingTransmissionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "terminate."]
    #[inline(always)]
    pub fn terminate(self) -> &'a mut crate::W<REG> {
        self.variant(TerminatingTransmission::Terminate)
    }
    #[doc = "normal operation."]
    #[inline(always)]
    pub fn normal_operation(self) -> &'a mut crate::W<REG> {
        self.variant(TerminatingTransmission::NormalOperation)
    }
}
#[doc = "MSB first.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msbfirst {
    #[doc = "1: MSB first."]
    MsbFirst = 1,
    #[doc = "0: LSB first."]
    LsbFirst = 0,
}
impl From<Msbfirst> for bool {
    #[inline(always)]
    fn from(variant: Msbfirst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSBFirst` reader - MSB first."]
pub type MsbfirstR = crate::BitReader<Msbfirst>;
impl MsbfirstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msbfirst {
        match self.bits {
            true => Msbfirst::MsbFirst,
            false => Msbfirst::LsbFirst,
        }
    }
    #[doc = "MSB first."]
    #[inline(always)]
    pub fn is_msb_first(&self) -> bool {
        *self == Msbfirst::MsbFirst
    }
    #[doc = "LSB first."]
    #[inline(always)]
    pub fn is_lsb_first(&self) -> bool {
        *self == Msbfirst::LsbFirst
    }
}
#[doc = "Field `MSBFirst` writer - MSB first."]
pub type MsbfirstW<'a, REG> = crate::BitWriter<'a, REG, Msbfirst>;
impl<'a, REG> MsbfirstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MSB first."]
    #[inline(always)]
    pub fn msb_first(self) -> &'a mut crate::W<REG> {
        self.variant(Msbfirst::MsbFirst)
    }
    #[doc = "LSB first."]
    #[inline(always)]
    pub fn lsb_first(self) -> &'a mut crate::W<REG> {
        self.variant(Msbfirst::LsbFirst)
    }
}
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `LengthInOneTransmission` reader - Length in one transmission"]
pub type LengthInOneTransmissionR = crate::FieldReader<u16>;
#[doc = "Field `LengthInOneTransmission` writer - Length in one transmission"]
pub type LengthInOneTransmissionW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
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
#[doc = "Field `EngEnbl` reader - Engine enable."]
pub type EngEnblR = crate::BitReader;
#[doc = "Field `EngEnbl` writer - Engine enable."]
pub type EngEnblW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable transmission of data."]
    #[inline(always)]
    pub fn enbl_transmission_of_data(&self) -> EnblTransmissionOfDataR {
        EnblTransmissionOfDataR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable transmission of instruction."]
    #[inline(always)]
    pub fn enbl_transmission_of_instruction(&self) -> EnblTransmissionOfInstructionR {
        EnblTransmissionOfInstructionR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Last transmission"]
    #[inline(always)]
    pub fn last_transmission(&self) -> LastTransmissionR {
        LastTransmissionR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Terminating transmission"]
    #[inline(always)]
    pub fn terminating_transmission(&self) -> TerminatingTransmissionR {
        TerminatingTransmissionR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MSB first."]
    #[inline(always)]
    pub fn msbfirst(&self) -> MsbfirstR {
        MsbfirstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:17 - Length in one transmission"]
    #[inline(always)]
    pub fn length_in_one_transmission(&self) -> LengthInOneTransmissionR {
        LengthInOneTransmissionR::new(((self.bits >> 8) & 0x03ff) as u16)
    }
    #[doc = "Bits 18:19 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Internal FIFO mode"]
    #[inline(always)]
    pub fn int_fifomode(&self) -> IntFifomodeR {
        IntFifomodeR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Reset Internal FIFO"]
    #[inline(always)]
    pub fn rst_int_fifo(&self) -> RstIntFifoR {
        RstIntFifoR::new(((self.bits >> 21) & 1) != 0)
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
    #[doc = "Bit 31 - Engine enable."]
    #[inline(always)]
    pub fn eng_enbl(&self) -> EngEnblR {
        EngEnblR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable transmission of data."]
    #[inline(always)]
    pub fn enbl_transmission_of_data(&mut self) -> EnblTransmissionOfDataW<Jtag008Spec> {
        EnblTransmissionOfDataW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable transmission of instruction."]
    #[inline(always)]
    pub fn enbl_transmission_of_instruction(
        &mut self,
    ) -> EnblTransmissionOfInstructionW<Jtag008Spec> {
        EnblTransmissionOfInstructionW::new(self, 1)
    }
    #[doc = "Bit 4 - Last transmission"]
    #[inline(always)]
    pub fn last_transmission(&mut self) -> LastTransmissionW<Jtag008Spec> {
        LastTransmissionW::new(self, 4)
    }
    #[doc = "Bit 5 - Terminating transmission"]
    #[inline(always)]
    pub fn terminating_transmission(&mut self) -> TerminatingTransmissionW<Jtag008Spec> {
        TerminatingTransmissionW::new(self, 5)
    }
    #[doc = "Bit 6 - MSB first."]
    #[inline(always)]
    pub fn msbfirst(&mut self) -> MsbfirstW<Jtag008Spec> {
        MsbfirstW::new(self, 6)
    }
    #[doc = "Bits 8:17 - Length in one transmission"]
    #[inline(always)]
    pub fn length_in_one_transmission(&mut self) -> LengthInOneTransmissionW<Jtag008Spec> {
        LengthInOneTransmissionW::new(self, 8)
    }
    #[doc = "Bit 21 - Reset Internal FIFO"]
    #[inline(always)]
    pub fn rst_int_fifo(&mut self) -> RstIntFifoW<Jtag008Spec> {
        RstIntFifoW::new(self, 21)
    }
    #[doc = "Bit 29 - Force controller and slave into Reset state by TMS."]
    #[inline(always)]
    pub fn force_ctrl_and_slave_into_rst_state_by_tms(
        &mut self,
    ) -> ForceCtrlAndSlaveIntoRstStateByTmsW<Jtag008Spec> {
        ForceCtrlAndSlaveIntoRstStateByTmsW::new(self, 29)
    }
    #[doc = "Bit 30 - Engine output enable."]
    #[inline(always)]
    pub fn eng_output_enbl(&mut self) -> EngOutputEnblW<Jtag008Spec> {
        EngOutputEnblW::new(self, 30)
    }
    #[doc = "Bit 31 - Engine enable."]
    #[inline(always)]
    pub fn eng_enbl(&mut self) -> EngEnblW<Jtag008Spec> {
        EngEnblW::new(self, 31)
    }
}
#[doc = "Engine control\n\nYou can [`read`](crate::Reg::read) this register and get [`jtag008::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jtag008::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Jtag008Spec;
impl crate::RegisterSpec for Jtag008Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jtag008::R`](R) reader structure"]
impl crate::Readable for Jtag008Spec {}
#[doc = "`write(|w| ..)` method takes [`jtag008::W`](W) writer structure"]
impl crate::Writable for Jtag008Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets JTAG008 to value 0"]
impl crate::Resettable for Jtag008Spec {}
