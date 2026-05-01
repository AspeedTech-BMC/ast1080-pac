#[doc = "Register `EHCI084` reader"]
pub type R = crate::R<Ehci084Spec>;
#[doc = "Register `EHCI084` writer"]
pub type W = crate::W<Ehci084Spec>;
#[doc = "Field `ProgrammableFrameListFlag` reader - Programmable Frame List Flag"]
pub type ProgrammableFrameListFlagR = crate::BitReader;
#[doc = "Field `ProgrammableFrameListFlag` writer - Programmable Frame List Flag"]
pub type ProgrammableFrameListFlagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AsynchronousScheduleParkCapability` reader - Asynchronous Schedule Park Capability"]
pub type AsynchronousScheduleParkCapabilityR = crate::BitReader;
#[doc = "Field `AsynchronousScheduleParkCapability` writer - Asynchronous Schedule Park Capability"]
pub type AsynchronousScheduleParkCapabilityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IsochronousSchedulingThreshold` reader - Isochronous Scheduling Threshold"]
pub type IsochronousSchedulingThresholdR = crate::FieldReader;
#[doc = "Field `IsochronousSchedulingThreshold` writer - Isochronous Scheduling Threshold"]
pub type IsochronousSchedulingThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Transmit FIFO Threshold\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TxFifothreshold {
    #[doc = "0: 128 bytes"]
    _128Bytes = 0,
    #[doc = "1: 256 bytes"]
    _256Bytes = 1,
    #[doc = "2: 512 bytes"]
    _512Bytes = 2,
    #[doc = "3: 768 bytes"]
    _768Bytes = 3,
}
impl From<TxFifothreshold> for u8 {
    #[inline(always)]
    fn from(variant: TxFifothreshold) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TxFifothreshold {
    type Ux = u8;
}
impl crate::IsEnum for TxFifothreshold {}
#[doc = "Field `TxFIFOThreshold` reader - Transmit FIFO Threshold"]
pub type TxFifothresholdR = crate::FieldReader<TxFifothreshold>;
impl TxFifothresholdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxFifothreshold {
        match self.bits {
            0 => TxFifothreshold::_128Bytes,
            1 => TxFifothreshold::_256Bytes,
            2 => TxFifothreshold::_512Bytes,
            3 => TxFifothreshold::_768Bytes,
            _ => unreachable!(),
        }
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn is_128_bytes(&self) -> bool {
        *self == TxFifothreshold::_128Bytes
    }
    #[doc = "256 bytes"]
    #[inline(always)]
    pub fn is_256_bytes(&self) -> bool {
        *self == TxFifothreshold::_256Bytes
    }
    #[doc = "512 bytes"]
    #[inline(always)]
    pub fn is_512_bytes(&self) -> bool {
        *self == TxFifothreshold::_512Bytes
    }
    #[doc = "768 bytes"]
    #[inline(always)]
    pub fn is_768_bytes(&self) -> bool {
        *self == TxFifothreshold::_768Bytes
    }
}
#[doc = "Field `TxFIFOThreshold` writer - Transmit FIFO Threshold"]
pub type TxFifothresholdW<'a, REG> = crate::FieldWriter<'a, REG, 2, TxFifothreshold, crate::Safe>;
impl<'a, REG> TxFifothresholdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn _128_bytes(self) -> &'a mut crate::W<REG> {
        self.variant(TxFifothreshold::_128Bytes)
    }
    #[doc = "256 bytes"]
    #[inline(always)]
    pub fn _256_bytes(self) -> &'a mut crate::W<REG> {
        self.variant(TxFifothreshold::_256Bytes)
    }
    #[doc = "512 bytes"]
    #[inline(always)]
    pub fn _512_bytes(self) -> &'a mut crate::W<REG> {
        self.variant(TxFifothreshold::_512Bytes)
    }
    #[doc = "768 bytes"]
    #[inline(always)]
    pub fn _768_bytes(self) -> &'a mut crate::W<REG> {
        self.variant(TxFifothreshold::_768Bytes)
    }
}
#[doc = "High speed Isochronous IN MaxPacketSize selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HighSpeedIsochronousInmaxpktSizeSel {
    #[doc = "0: MaxPacketSize is determined by Transaction\\_X\\_Length or Maximum Packet Size (whichever is less)."]
    MaxPacketSizeIsDeterminedByTransactionXlengthOrMaximumPacketSizeWhicheverIsLess = 0,
    #[doc = "1: MaxPacketSize is determined by Maximum Packet Size field."]
    MaxPacketSizeIsDeterminedByMaximumPacketSizeField = 1,
}
impl From<HighSpeedIsochronousInmaxpktSizeSel> for bool {
    #[inline(always)]
    fn from(variant: HighSpeedIsochronousInmaxpktSizeSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HighSpeedIsochronousINMaxpktSizeSel` reader - High speed Isochronous IN MaxPacketSize selection"]
pub type HighSpeedIsochronousInmaxpktSizeSelR =
    crate::BitReader<HighSpeedIsochronousInmaxpktSizeSel>;
impl HighSpeedIsochronousInmaxpktSizeSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HighSpeedIsochronousInmaxpktSizeSel {
        match self . bits { false => HighSpeedIsochronousInmaxpktSizeSel :: MaxPacketSizeIsDeterminedByTransactionXlengthOrMaximumPacketSizeWhicheverIsLess , true => HighSpeedIsochronousInmaxpktSizeSel :: MaxPacketSizeIsDeterminedByMaximumPacketSizeField , }
    }
    #[doc = "MaxPacketSize is determined by Transaction\\_X\\_Length or Maximum Packet Size (whichever is less)."]
    #[inline(always)]
    pub fn is_max_packet_size_is_determined_by_transaction_xlength_or_maximum_packet_size_whichever_is_less(
        &self,
    ) -> bool {
        * self == HighSpeedIsochronousInmaxpktSizeSel :: MaxPacketSizeIsDeterminedByTransactionXlengthOrMaximumPacketSizeWhicheverIsLess
    }
    #[doc = "MaxPacketSize is determined by Maximum Packet Size field."]
    #[inline(always)]
    pub fn is_max_packet_size_is_determined_by_maximum_packet_size_field(&self) -> bool {
        * self == HighSpeedIsochronousInmaxpktSizeSel :: MaxPacketSizeIsDeterminedByMaximumPacketSizeField
    }
}
#[doc = "Field `HighSpeedIsochronousINMaxpktSizeSel` writer - High speed Isochronous IN MaxPacketSize selection"]
pub type HighSpeedIsochronousInmaxpktSizeSelW<'a, REG> =
    crate::BitWriter<'a, REG, HighSpeedIsochronousInmaxpktSizeSel>;
impl<'a, REG> HighSpeedIsochronousInmaxpktSizeSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MaxPacketSize is determined by Transaction\\_X\\_Length or Maximum Packet Size (whichever is less)."]
    #[inline(always)]
    pub fn max_packet_size_is_determined_by_transaction_xlength_or_maximum_packet_size_whichever_is_less(
        self,
    ) -> &'a mut crate::W<REG> {
        self . variant (HighSpeedIsochronousInmaxpktSizeSel :: MaxPacketSizeIsDeterminedByTransactionXlengthOrMaximumPacketSizeWhicheverIsLess)
    }
    #[doc = "MaxPacketSize is determined by Maximum Packet Size field."]
    #[inline(always)]
    pub fn max_packet_size_is_determined_by_maximum_packet_size_field(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(
            HighSpeedIsochronousInmaxpktSizeSel::MaxPacketSizeIsDeterminedByMaximumPacketSizeField,
        )
    }
}
#[doc = "Companion Controller Selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompanionCtrlSel {
    #[doc = "0: No companion controller reported."]
    NoCompanionControllerReported = 0,
    #[doc = "1: One companion controller reported."]
    OneCompanionControllerReported = 1,
}
impl From<CompanionCtrlSel> for bool {
    #[inline(always)]
    fn from(variant: CompanionCtrlSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CompanionCtrlSel` reader - Companion Controller Selection"]
pub type CompanionCtrlSelR = crate::BitReader<CompanionCtrlSel>;
impl CompanionCtrlSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CompanionCtrlSel {
        match self.bits {
            false => CompanionCtrlSel::NoCompanionControllerReported,
            true => CompanionCtrlSel::OneCompanionControllerReported,
        }
    }
    #[doc = "No companion controller reported."]
    #[inline(always)]
    pub fn is_no_companion_controller_reported(&self) -> bool {
        *self == CompanionCtrlSel::NoCompanionControllerReported
    }
    #[doc = "One companion controller reported."]
    #[inline(always)]
    pub fn is_one_companion_controller_reported(&self) -> bool {
        *self == CompanionCtrlSel::OneCompanionControllerReported
    }
}
#[doc = "Field `CompanionCtrlSel` writer - Companion Controller Selection"]
pub type CompanionCtrlSelW<'a, REG> = crate::BitWriter<'a, REG, CompanionCtrlSel>;
impl<'a, REG> CompanionCtrlSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No companion controller reported."]
    #[inline(always)]
    pub fn no_companion_controller_reported(self) -> &'a mut crate::W<REG> {
        self.variant(CompanionCtrlSel::NoCompanionControllerReported)
    }
    #[doc = "One companion controller reported."]
    #[inline(always)]
    pub fn one_companion_controller_reported(self) -> &'a mut crate::W<REG> {
        self.variant(CompanionCtrlSel::OneCompanionControllerReported)
    }
}
#[doc = "Enable FIFO auto power down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblFifoautoPowerDown {
    #[doc = "0: Disable (default)"]
    DisableDefault = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<EnblFifoautoPowerDown> for bool {
    #[inline(always)]
    fn from(variant: EnblFifoautoPowerDown) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblFIFOAutoPowerDown` reader - Enable FIFO auto power down"]
pub type EnblFifoautoPowerDownR = crate::BitReader<EnblFifoautoPowerDown>;
impl EnblFifoautoPowerDownR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblFifoautoPowerDown {
        match self.bits {
            false => EnblFifoautoPowerDown::DisableDefault,
            true => EnblFifoautoPowerDown::Enable,
        }
    }
    #[doc = "Disable (default)"]
    #[inline(always)]
    pub fn is_disable_default(&self) -> bool {
        *self == EnblFifoautoPowerDown::DisableDefault
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EnblFifoautoPowerDown::Enable
    }
}
#[doc = "Field `EnblFIFOAutoPowerDown` writer - Enable FIFO auto power down"]
pub type EnblFifoautoPowerDownW<'a, REG> = crate::BitWriter<'a, REG, EnblFifoautoPowerDown>;
impl<'a, REG> EnblFifoautoPowerDownW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable (default)"]
    #[inline(always)]
    pub fn disable_default(self) -> &'a mut crate::W<REG> {
        self.variant(EnblFifoautoPowerDown::DisableDefault)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EnblFifoautoPowerDown::Enable)
    }
}
#[doc = "Field `EnblSupport64BitAddrMode` reader - Enable support 64 bit address mode"]
pub type EnblSupport64bitAddrModeR = crate::BitReader;
#[doc = "Field `EnblSupport64BitAddrMode` writer - Enable support 64 bit address mode"]
pub type EnblSupport64bitAddrModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Programmable Frame List Flag"]
    #[inline(always)]
    pub fn programmable_frame_list_flag(&self) -> ProgrammableFrameListFlagR {
        ProgrammableFrameListFlagR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Asynchronous Schedule Park Capability"]
    #[inline(always)]
    pub fn asynchronous_schedule_park_capability(&self) -> AsynchronousScheduleParkCapabilityR {
        AsynchronousScheduleParkCapabilityR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - Isochronous Scheduling Threshold"]
    #[inline(always)]
    pub fn isochronous_scheduling_threshold(&self) -> IsochronousSchedulingThresholdR {
        IsochronousSchedulingThresholdR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:7 - Transmit FIFO Threshold"]
    #[inline(always)]
    pub fn tx_fifothreshold(&self) -> TxFifothresholdR {
        TxFifothresholdR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - High speed Isochronous IN MaxPacketSize selection"]
    #[inline(always)]
    pub fn high_speed_isochronous_inmaxpkt_size_sel(&self) -> HighSpeedIsochronousInmaxpktSizeSelR {
        HighSpeedIsochronousInmaxpktSizeSelR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Companion Controller Selection"]
    #[inline(always)]
    pub fn companion_ctrl_sel(&self) -> CompanionCtrlSelR {
        CompanionCtrlSelR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable FIFO auto power down"]
    #[inline(always)]
    pub fn enbl_fifoauto_power_down(&self) -> EnblFifoautoPowerDownR {
        EnblFifoautoPowerDownR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable support 64 bit address mode"]
    #[inline(always)]
    pub fn enbl_support64bit_addr_mode(&self) -> EnblSupport64bitAddrModeR {
        EnblSupport64bitAddrModeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Programmable Frame List Flag"]
    #[inline(always)]
    pub fn programmable_frame_list_flag(&mut self) -> ProgrammableFrameListFlagW<Ehci084Spec> {
        ProgrammableFrameListFlagW::new(self, 0)
    }
    #[doc = "Bit 1 - Asynchronous Schedule Park Capability"]
    #[inline(always)]
    pub fn asynchronous_schedule_park_capability(
        &mut self,
    ) -> AsynchronousScheduleParkCapabilityW<Ehci084Spec> {
        AsynchronousScheduleParkCapabilityW::new(self, 1)
    }
    #[doc = "Bits 2:5 - Isochronous Scheduling Threshold"]
    #[inline(always)]
    pub fn isochronous_scheduling_threshold(
        &mut self,
    ) -> IsochronousSchedulingThresholdW<Ehci084Spec> {
        IsochronousSchedulingThresholdW::new(self, 2)
    }
    #[doc = "Bits 6:7 - Transmit FIFO Threshold"]
    #[inline(always)]
    pub fn tx_fifothreshold(&mut self) -> TxFifothresholdW<Ehci084Spec> {
        TxFifothresholdW::new(self, 6)
    }
    #[doc = "Bit 8 - High speed Isochronous IN MaxPacketSize selection"]
    #[inline(always)]
    pub fn high_speed_isochronous_inmaxpkt_size_sel(
        &mut self,
    ) -> HighSpeedIsochronousInmaxpktSizeSelW<Ehci084Spec> {
        HighSpeedIsochronousInmaxpktSizeSelW::new(self, 8)
    }
    #[doc = "Bit 9 - Companion Controller Selection"]
    #[inline(always)]
    pub fn companion_ctrl_sel(&mut self) -> CompanionCtrlSelW<Ehci084Spec> {
        CompanionCtrlSelW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable FIFO auto power down"]
    #[inline(always)]
    pub fn enbl_fifoauto_power_down(&mut self) -> EnblFifoautoPowerDownW<Ehci084Spec> {
        EnblFifoautoPowerDownW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable support 64 bit address mode"]
    #[inline(always)]
    pub fn enbl_support64bit_addr_mode(&mut self) -> EnblSupport64bitAddrModeW<Ehci084Spec> {
        EnblSupport64bitAddrModeW::new(self, 11)
    }
}
#[doc = "Controller Fine-tune Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ehci084::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehci084::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ehci084Spec;
impl crate::RegisterSpec for Ehci084Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ehci084::R`](R) reader structure"]
impl crate::Readable for Ehci084Spec {}
#[doc = "`write(|w| ..)` method takes [`ehci084::W`](W) writer structure"]
impl crate::Writable for Ehci084Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EHCI084 to value 0x0a47"]
impl crate::Resettable for Ehci084Spec {
    const RESET_VALUE: u32 = 0x0a47;
}
