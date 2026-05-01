#[doc = "Register `JTAG00C` reader"]
pub type R = crate::R<Jtag00cSpec>;
#[doc = "Register `JTAG00C` writer"]
pub type W = crate::W<Jtag00cSpec>;
#[doc = "Enable of Data transmission completed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblOfDataTransmissionCompleted {
    #[doc = "0: Disable interrupt"]
    DisableInterrupt = 0,
    #[doc = "1: Enable interrupt"]
    EnableInterrupt = 1,
}
impl From<EnblOfDataTransmissionCompleted> for bool {
    #[inline(always)]
    fn from(variant: EnblOfDataTransmissionCompleted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblOfDataTransmissionCompleted` reader - Enable of Data transmission completed."]
pub type EnblOfDataTransmissionCompletedR = crate::BitReader<EnblOfDataTransmissionCompleted>;
impl EnblOfDataTransmissionCompletedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblOfDataTransmissionCompleted {
        match self.bits {
            false => EnblOfDataTransmissionCompleted::DisableInterrupt,
            true => EnblOfDataTransmissionCompleted::EnableInterrupt,
        }
    }
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn is_disable_interrupt(&self) -> bool {
        *self == EnblOfDataTransmissionCompleted::DisableInterrupt
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn is_enable_interrupt(&self) -> bool {
        *self == EnblOfDataTransmissionCompleted::EnableInterrupt
    }
}
#[doc = "Field `EnblOfDataTransmissionCompleted` writer - Enable of Data transmission completed."]
pub type EnblOfDataTransmissionCompletedW<'a, REG> =
    crate::BitWriter<'a, REG, EnblOfDataTransmissionCompleted>;
impl<'a, REG> EnblOfDataTransmissionCompletedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable_interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(EnblOfDataTransmissionCompleted::DisableInterrupt)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable_interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(EnblOfDataTransmissionCompleted::EnableInterrupt)
    }
}
#[doc = "Enable of Data transmission pause.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblOfDataTransmissionPause {
    #[doc = "0: Disable interrupt"]
    DisableInterrupt = 0,
    #[doc = "1: Enable interrupt"]
    EnableInterrupt = 1,
}
impl From<EnblOfDataTransmissionPause> for bool {
    #[inline(always)]
    fn from(variant: EnblOfDataTransmissionPause) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblOfDataTransmissionPause` reader - Enable of Data transmission pause."]
pub type EnblOfDataTransmissionPauseR = crate::BitReader<EnblOfDataTransmissionPause>;
impl EnblOfDataTransmissionPauseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblOfDataTransmissionPause {
        match self.bits {
            false => EnblOfDataTransmissionPause::DisableInterrupt,
            true => EnblOfDataTransmissionPause::EnableInterrupt,
        }
    }
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn is_disable_interrupt(&self) -> bool {
        *self == EnblOfDataTransmissionPause::DisableInterrupt
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn is_enable_interrupt(&self) -> bool {
        *self == EnblOfDataTransmissionPause::EnableInterrupt
    }
}
#[doc = "Field `EnblOfDataTransmissionPause` writer - Enable of Data transmission pause."]
pub type EnblOfDataTransmissionPauseW<'a, REG> =
    crate::BitWriter<'a, REG, EnblOfDataTransmissionPause>;
impl<'a, REG> EnblOfDataTransmissionPauseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable_interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(EnblOfDataTransmissionPause::DisableInterrupt)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable_interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(EnblOfDataTransmissionPause::EnableInterrupt)
    }
}
#[doc = "Enable of Instruction transmission completed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblOfInstructionTransmissionCompleted {
    #[doc = "0: Disable interrupt"]
    DisableInterrupt = 0,
    #[doc = "1: Enable interrupt"]
    EnableInterrupt = 1,
}
impl From<EnblOfInstructionTransmissionCompleted> for bool {
    #[inline(always)]
    fn from(variant: EnblOfInstructionTransmissionCompleted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblOfInstructionTransmissionCompleted` reader - Enable of Instruction transmission completed."]
pub type EnblOfInstructionTransmissionCompletedR =
    crate::BitReader<EnblOfInstructionTransmissionCompleted>;
impl EnblOfInstructionTransmissionCompletedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblOfInstructionTransmissionCompleted {
        match self.bits {
            false => EnblOfInstructionTransmissionCompleted::DisableInterrupt,
            true => EnblOfInstructionTransmissionCompleted::EnableInterrupt,
        }
    }
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn is_disable_interrupt(&self) -> bool {
        *self == EnblOfInstructionTransmissionCompleted::DisableInterrupt
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn is_enable_interrupt(&self) -> bool {
        *self == EnblOfInstructionTransmissionCompleted::EnableInterrupt
    }
}
#[doc = "Field `EnblOfInstructionTransmissionCompleted` writer - Enable of Instruction transmission completed."]
pub type EnblOfInstructionTransmissionCompletedW<'a, REG> =
    crate::BitWriter<'a, REG, EnblOfInstructionTransmissionCompleted>;
impl<'a, REG> EnblOfInstructionTransmissionCompletedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable_interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(EnblOfInstructionTransmissionCompleted::DisableInterrupt)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable_interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(EnblOfInstructionTransmissionCompleted::EnableInterrupt)
    }
}
#[doc = "Enable of Instruction transmission pause.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblOfInstructionTransmissionPause {
    #[doc = "0: Disable interrupt"]
    DisableInterrupt = 0,
    #[doc = "1: Enable interrupt"]
    EnableInterrupt = 1,
}
impl From<EnblOfInstructionTransmissionPause> for bool {
    #[inline(always)]
    fn from(variant: EnblOfInstructionTransmissionPause) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblOfInstructionTransmissionPause` reader - Enable of Instruction transmission pause."]
pub type EnblOfInstructionTransmissionPauseR = crate::BitReader<EnblOfInstructionTransmissionPause>;
impl EnblOfInstructionTransmissionPauseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblOfInstructionTransmissionPause {
        match self.bits {
            false => EnblOfInstructionTransmissionPause::DisableInterrupt,
            true => EnblOfInstructionTransmissionPause::EnableInterrupt,
        }
    }
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn is_disable_interrupt(&self) -> bool {
        *self == EnblOfInstructionTransmissionPause::DisableInterrupt
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn is_enable_interrupt(&self) -> bool {
        *self == EnblOfInstructionTransmissionPause::EnableInterrupt
    }
}
#[doc = "Field `EnblOfInstructionTransmissionPause` writer - Enable of Instruction transmission pause."]
pub type EnblOfInstructionTransmissionPauseW<'a, REG> =
    crate::BitWriter<'a, REG, EnblOfInstructionTransmissionPause>;
impl<'a, REG> EnblOfInstructionTransmissionPauseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable_interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(EnblOfInstructionTransmissionPause::DisableInterrupt)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable_interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(EnblOfInstructionTransmissionPause::EnableInterrupt)
    }
}
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader<u16>;
#[doc = "Data transmission completed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataTransmissionCompleted {
    #[doc = "0: No interrupt pending"]
    NoInterruptPending = 0,
    #[doc = "1: interrupt pending"]
    InterruptPending = 1,
}
impl From<DataTransmissionCompleted> for bool {
    #[inline(always)]
    fn from(variant: DataTransmissionCompleted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DataTransmissionCompleted` reader - Data transmission completed."]
pub type DataTransmissionCompletedR = crate::BitReader<DataTransmissionCompleted>;
impl DataTransmissionCompletedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DataTransmissionCompleted {
        match self.bits {
            false => DataTransmissionCompleted::NoInterruptPending,
            true => DataTransmissionCompleted::InterruptPending,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_no_interrupt_pending(&self) -> bool {
        *self == DataTransmissionCompleted::NoInterruptPending
    }
    #[doc = "interrupt pending"]
    #[inline(always)]
    pub fn is_interrupt_pending(&self) -> bool {
        *self == DataTransmissionCompleted::InterruptPending
    }
}
#[doc = "Field `DataTransmissionCompleted` writer - Data transmission completed."]
pub type DataTransmissionCompletedW<'a, REG> = crate::BitWriter<'a, REG, DataTransmissionCompleted>;
impl<'a, REG> DataTransmissionCompletedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn no_interrupt_pending(self) -> &'a mut crate::W<REG> {
        self.variant(DataTransmissionCompleted::NoInterruptPending)
    }
    #[doc = "interrupt pending"]
    #[inline(always)]
    pub fn interrupt_pending(self) -> &'a mut crate::W<REG> {
        self.variant(DataTransmissionCompleted::InterruptPending)
    }
}
#[doc = "Data transmission pause.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataTransmissionPause {
    #[doc = "0: No interrupt pending"]
    NoInterruptPending = 0,
    #[doc = "1: interrupt pending"]
    InterruptPending = 1,
}
impl From<DataTransmissionPause> for bool {
    #[inline(always)]
    fn from(variant: DataTransmissionPause) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DataTransmissionPause` reader - Data transmission pause."]
pub type DataTransmissionPauseR = crate::BitReader<DataTransmissionPause>;
impl DataTransmissionPauseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DataTransmissionPause {
        match self.bits {
            false => DataTransmissionPause::NoInterruptPending,
            true => DataTransmissionPause::InterruptPending,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_no_interrupt_pending(&self) -> bool {
        *self == DataTransmissionPause::NoInterruptPending
    }
    #[doc = "interrupt pending"]
    #[inline(always)]
    pub fn is_interrupt_pending(&self) -> bool {
        *self == DataTransmissionPause::InterruptPending
    }
}
#[doc = "Field `DataTransmissionPause` writer - Data transmission pause."]
pub type DataTransmissionPauseW<'a, REG> = crate::BitWriter<'a, REG, DataTransmissionPause>;
impl<'a, REG> DataTransmissionPauseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn no_interrupt_pending(self) -> &'a mut crate::W<REG> {
        self.variant(DataTransmissionPause::NoInterruptPending)
    }
    #[doc = "interrupt pending"]
    #[inline(always)]
    pub fn interrupt_pending(self) -> &'a mut crate::W<REG> {
        self.variant(DataTransmissionPause::InterruptPending)
    }
}
#[doc = "Instruction transmission completed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InstructionTransmissionCompleted {
    #[doc = "0: No interrupt pending"]
    NoInterruptPending = 0,
    #[doc = "1: interrupt pending"]
    InterruptPending = 1,
}
impl From<InstructionTransmissionCompleted> for bool {
    #[inline(always)]
    fn from(variant: InstructionTransmissionCompleted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `InstructionTransmissionCompleted` reader - Instruction transmission completed."]
pub type InstructionTransmissionCompletedR = crate::BitReader<InstructionTransmissionCompleted>;
impl InstructionTransmissionCompletedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> InstructionTransmissionCompleted {
        match self.bits {
            false => InstructionTransmissionCompleted::NoInterruptPending,
            true => InstructionTransmissionCompleted::InterruptPending,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_no_interrupt_pending(&self) -> bool {
        *self == InstructionTransmissionCompleted::NoInterruptPending
    }
    #[doc = "interrupt pending"]
    #[inline(always)]
    pub fn is_interrupt_pending(&self) -> bool {
        *self == InstructionTransmissionCompleted::InterruptPending
    }
}
#[doc = "Field `InstructionTransmissionCompleted` writer - Instruction transmission completed."]
pub type InstructionTransmissionCompletedW<'a, REG> =
    crate::BitWriter<'a, REG, InstructionTransmissionCompleted>;
impl<'a, REG> InstructionTransmissionCompletedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn no_interrupt_pending(self) -> &'a mut crate::W<REG> {
        self.variant(InstructionTransmissionCompleted::NoInterruptPending)
    }
    #[doc = "interrupt pending"]
    #[inline(always)]
    pub fn interrupt_pending(self) -> &'a mut crate::W<REG> {
        self.variant(InstructionTransmissionCompleted::InterruptPending)
    }
}
#[doc = "Interrupt status of instruction transmission pause.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntstsOfInstructionTransmissionPause {
    #[doc = "0: No interrupt pending"]
    NoInterruptPending = 0,
    #[doc = "1: interrupt pending"]
    InterruptPending = 1,
}
impl From<IntstsOfInstructionTransmissionPause> for bool {
    #[inline(always)]
    fn from(variant: IntstsOfInstructionTransmissionPause) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTStsOfInstructionTransmissionPause` reader - Interrupt status of instruction transmission pause."]
pub type IntstsOfInstructionTransmissionPauseR =
    crate::BitReader<IntstsOfInstructionTransmissionPause>;
impl IntstsOfInstructionTransmissionPauseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntstsOfInstructionTransmissionPause {
        match self.bits {
            false => IntstsOfInstructionTransmissionPause::NoInterruptPending,
            true => IntstsOfInstructionTransmissionPause::InterruptPending,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_no_interrupt_pending(&self) -> bool {
        *self == IntstsOfInstructionTransmissionPause::NoInterruptPending
    }
    #[doc = "interrupt pending"]
    #[inline(always)]
    pub fn is_interrupt_pending(&self) -> bool {
        *self == IntstsOfInstructionTransmissionPause::InterruptPending
    }
}
#[doc = "Field `INTStsOfInstructionTransmissionPause` writer - Interrupt status of instruction transmission pause."]
pub type IntstsOfInstructionTransmissionPauseW<'a, REG> =
    crate::BitWriter<'a, REG, IntstsOfInstructionTransmissionPause>;
impl<'a, REG> IntstsOfInstructionTransmissionPauseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn no_interrupt_pending(self) -> &'a mut crate::W<REG> {
        self.variant(IntstsOfInstructionTransmissionPause::NoInterruptPending)
    }
    #[doc = "interrupt pending"]
    #[inline(always)]
    pub fn interrupt_pending(self) -> &'a mut crate::W<REG> {
        self.variant(IntstsOfInstructionTransmissionPause::InterruptPending)
    }
}
impl R {
    #[doc = "Bit 0 - Enable of Data transmission completed."]
    #[inline(always)]
    pub fn enbl_of_data_transmission_completed(&self) -> EnblOfDataTransmissionCompletedR {
        EnblOfDataTransmissionCompletedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable of Data transmission pause."]
    #[inline(always)]
    pub fn enbl_of_data_transmission_pause(&self) -> EnblOfDataTransmissionPauseR {
        EnblOfDataTransmissionPauseR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable of Instruction transmission completed."]
    #[inline(always)]
    pub fn enbl_of_instruction_transmission_completed(
        &self,
    ) -> EnblOfInstructionTransmissionCompletedR {
        EnblOfInstructionTransmissionCompletedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable of Instruction transmission pause."]
    #[inline(always)]
    pub fn enbl_of_instruction_transmission_pause(&self) -> EnblOfInstructionTransmissionPauseR {
        EnblOfInstructionTransmissionPauseR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:15 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bit 16 - Data transmission completed."]
    #[inline(always)]
    pub fn data_transmission_completed(&self) -> DataTransmissionCompletedR {
        DataTransmissionCompletedR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Data transmission pause."]
    #[inline(always)]
    pub fn data_transmission_pause(&self) -> DataTransmissionPauseR {
        DataTransmissionPauseR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Instruction transmission completed."]
    #[inline(always)]
    pub fn instruction_transmission_completed(&self) -> InstructionTransmissionCompletedR {
        InstructionTransmissionCompletedR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt status of instruction transmission pause."]
    #[inline(always)]
    pub fn intsts_of_instruction_transmission_pause(
        &self,
    ) -> IntstsOfInstructionTransmissionPauseR {
        IntstsOfInstructionTransmissionPauseR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable of Data transmission completed."]
    #[inline(always)]
    pub fn enbl_of_data_transmission_completed(
        &mut self,
    ) -> EnblOfDataTransmissionCompletedW<Jtag00cSpec> {
        EnblOfDataTransmissionCompletedW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable of Data transmission pause."]
    #[inline(always)]
    pub fn enbl_of_data_transmission_pause(&mut self) -> EnblOfDataTransmissionPauseW<Jtag00cSpec> {
        EnblOfDataTransmissionPauseW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable of Instruction transmission completed."]
    #[inline(always)]
    pub fn enbl_of_instruction_transmission_completed(
        &mut self,
    ) -> EnblOfInstructionTransmissionCompletedW<Jtag00cSpec> {
        EnblOfInstructionTransmissionCompletedW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable of Instruction transmission pause."]
    #[inline(always)]
    pub fn enbl_of_instruction_transmission_pause(
        &mut self,
    ) -> EnblOfInstructionTransmissionPauseW<Jtag00cSpec> {
        EnblOfInstructionTransmissionPauseW::new(self, 3)
    }
    #[doc = "Bit 16 - Data transmission completed."]
    #[inline(always)]
    pub fn data_transmission_completed(&mut self) -> DataTransmissionCompletedW<Jtag00cSpec> {
        DataTransmissionCompletedW::new(self, 16)
    }
    #[doc = "Bit 17 - Data transmission pause."]
    #[inline(always)]
    pub fn data_transmission_pause(&mut self) -> DataTransmissionPauseW<Jtag00cSpec> {
        DataTransmissionPauseW::new(self, 17)
    }
    #[doc = "Bit 18 - Instruction transmission completed."]
    #[inline(always)]
    pub fn instruction_transmission_completed(
        &mut self,
    ) -> InstructionTransmissionCompletedW<Jtag00cSpec> {
        InstructionTransmissionCompletedW::new(self, 18)
    }
    #[doc = "Bit 19 - Interrupt status of instruction transmission pause."]
    #[inline(always)]
    pub fn intsts_of_instruction_transmission_pause(
        &mut self,
    ) -> IntstsOfInstructionTransmissionPauseW<Jtag00cSpec> {
        IntstsOfInstructionTransmissionPauseW::new(self, 19)
    }
}
#[doc = "Interrupt status and enable\n\nYou can [`read`](crate::Reg::read) this register and get [`jtag00c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jtag00c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Jtag00cSpec;
impl crate::RegisterSpec for Jtag00cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jtag00c::R`](R) reader structure"]
impl crate::Readable for Jtag00cSpec {}
#[doc = "`write(|w| ..)` method takes [`jtag00c::W`](W) writer structure"]
impl crate::Writable for Jtag00cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets JTAG00C to value 0"]
impl crate::Resettable for Jtag00cSpec {}
