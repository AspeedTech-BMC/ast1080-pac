#[doc = "Register `UHCI008` reader"]
pub type R = crate::R<Uhci008Spec>;
#[doc = "Register `UHCI008` writer"]
pub type W = crate::W<Uhci008Spec>;
#[doc = "Timeout/CRC Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeoutCrcintenbl {
    #[doc = "0: Disabled."]
    Disabled = 0,
    #[doc = "1: Enabled."]
    Enabled = 1,
}
impl From<TimeoutCrcintenbl> for bool {
    #[inline(always)]
    fn from(variant: TimeoutCrcintenbl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TimeoutCRCINTEnbl` reader - Timeout/CRC Interrupt Enable"]
pub type TimeoutCrcintenblR = crate::BitReader<TimeoutCrcintenbl>;
impl TimeoutCrcintenblR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TimeoutCrcintenbl {
        match self.bits {
            false => TimeoutCrcintenbl::Disabled,
            true => TimeoutCrcintenbl::Enabled,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TimeoutCrcintenbl::Disabled
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TimeoutCrcintenbl::Enabled
    }
}
#[doc = "Field `TimeoutCRCINTEnbl` writer - Timeout/CRC Interrupt Enable"]
pub type TimeoutCrcintenblW<'a, REG> = crate::BitWriter<'a, REG, TimeoutCrcintenbl>;
impl<'a, REG> TimeoutCrcintenblW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TimeoutCrcintenbl::Disabled)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TimeoutCrcintenbl::Enabled)
    }
}
#[doc = "Resume Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResumeIntenbl {
    #[doc = "0: Disabled."]
    Disabled = 0,
    #[doc = "1: Enabled."]
    Enabled = 1,
}
impl From<ResumeIntenbl> for bool {
    #[inline(always)]
    fn from(variant: ResumeIntenbl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ResumeINTEnbl` reader - Resume Interrupt Enable"]
pub type ResumeIntenblR = crate::BitReader<ResumeIntenbl>;
impl ResumeIntenblR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ResumeIntenbl {
        match self.bits {
            false => ResumeIntenbl::Disabled,
            true => ResumeIntenbl::Enabled,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ResumeIntenbl::Disabled
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ResumeIntenbl::Enabled
    }
}
#[doc = "Field `ResumeINTEnbl` writer - Resume Interrupt Enable"]
pub type ResumeIntenblW<'a, REG> = crate::BitWriter<'a, REG, ResumeIntenbl>;
impl<'a, REG> ResumeIntenblW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ResumeIntenbl::Disabled)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ResumeIntenbl::Enabled)
    }
}
#[doc = "Interrupt On Complete (IOC) Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntonCompleteIocenbl {
    #[doc = "0: Disabled."]
    Disabled = 0,
    #[doc = "1: Enabled."]
    Enabled = 1,
}
impl From<IntonCompleteIocenbl> for bool {
    #[inline(always)]
    fn from(variant: IntonCompleteIocenbl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTOnCompleteIOCEnbl` reader - Interrupt On Complete (IOC) Enable"]
pub type IntonCompleteIocenblR = crate::BitReader<IntonCompleteIocenbl>;
impl IntonCompleteIocenblR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntonCompleteIocenbl {
        match self.bits {
            false => IntonCompleteIocenbl::Disabled,
            true => IntonCompleteIocenbl::Enabled,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IntonCompleteIocenbl::Disabled
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IntonCompleteIocenbl::Enabled
    }
}
#[doc = "Field `INTOnCompleteIOCEnbl` writer - Interrupt On Complete (IOC) Enable"]
pub type IntonCompleteIocenblW<'a, REG> = crate::BitWriter<'a, REG, IntonCompleteIocenbl>;
impl<'a, REG> IntonCompleteIocenblW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IntonCompleteIocenbl::Disabled)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IntonCompleteIocenbl::Enabled)
    }
}
#[doc = "Short Packet Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShortPktIntenbl {
    #[doc = "0: Disabled."]
    Disabled = 0,
    #[doc = "1: Enabled."]
    Enabled = 1,
}
impl From<ShortPktIntenbl> for bool {
    #[inline(always)]
    fn from(variant: ShortPktIntenbl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ShortPktINTEnbl` reader - Short Packet Interrupt Enable"]
pub type ShortPktIntenblR = crate::BitReader<ShortPktIntenbl>;
impl ShortPktIntenblR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ShortPktIntenbl {
        match self.bits {
            false => ShortPktIntenbl::Disabled,
            true => ShortPktIntenbl::Enabled,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ShortPktIntenbl::Disabled
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ShortPktIntenbl::Enabled
    }
}
#[doc = "Field `ShortPktINTEnbl` writer - Short Packet Interrupt Enable"]
pub type ShortPktIntenblW<'a, REG> = crate::BitWriter<'a, REG, ShortPktIntenbl>;
impl<'a, REG> ShortPktIntenblW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ShortPktIntenbl::Disabled)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ShortPktIntenbl::Enabled)
    }
}
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Timeout/CRC Interrupt Enable"]
    #[inline(always)]
    pub fn timeout_crcintenbl(&self) -> TimeoutCrcintenblR {
        TimeoutCrcintenblR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Resume Interrupt Enable"]
    #[inline(always)]
    pub fn resume_intenbl(&self) -> ResumeIntenblR {
        ResumeIntenblR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt On Complete (IOC) Enable"]
    #[inline(always)]
    pub fn inton_complete_iocenbl(&self) -> IntonCompleteIocenblR {
        IntonCompleteIocenblR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Short Packet Interrupt Enable"]
    #[inline(always)]
    pub fn short_pkt_intenbl(&self) -> ShortPktIntenblR {
        ShortPktIntenblR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Timeout/CRC Interrupt Enable"]
    #[inline(always)]
    pub fn timeout_crcintenbl(&mut self) -> TimeoutCrcintenblW<Uhci008Spec> {
        TimeoutCrcintenblW::new(self, 0)
    }
    #[doc = "Bit 1 - Resume Interrupt Enable"]
    #[inline(always)]
    pub fn resume_intenbl(&mut self) -> ResumeIntenblW<Uhci008Spec> {
        ResumeIntenblW::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt On Complete (IOC) Enable"]
    #[inline(always)]
    pub fn inton_complete_iocenbl(&mut self) -> IntonCompleteIocenblW<Uhci008Spec> {
        IntonCompleteIocenblW::new(self, 2)
    }
    #[doc = "Bit 3 - Short Packet Interrupt Enable"]
    #[inline(always)]
    pub fn short_pkt_intenbl(&mut self) -> ShortPktIntenblW<Uhci008Spec> {
        ShortPktIntenblW::new(self, 3)
    }
}
#[doc = "USB Interrupt Enable Register (USBINT)\n\nYou can [`read`](crate::Reg::read) this register and get [`uhci008::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhci008::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uhci008Spec;
impl crate::RegisterSpec for Uhci008Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhci008::R`](R) reader structure"]
impl crate::Readable for Uhci008Spec {}
#[doc = "`write(|w| ..)` method takes [`uhci008::W`](W) writer structure"]
impl crate::Writable for Uhci008Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UHCI008 to value 0"]
impl crate::Resettable for Uhci008Spec {}
