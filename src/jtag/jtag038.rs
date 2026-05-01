#[doc = "Register `JTAG038` reader"]
pub type R = crate::R<Jtag038Spec>;
#[doc = "Register `JTAG038` writer"]
pub type W = crate::W<Jtag038Spec>;
#[doc = "Shift Complete Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShiftCompleteIntstatus {
    #[doc = "0: No interrupt pending"]
    NoInterruptPending = 0,
    #[doc = "1: interrupt pending"]
    InterruptPending = 1,
}
impl From<ShiftCompleteIntstatus> for bool {
    #[inline(always)]
    fn from(variant: ShiftCompleteIntstatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ShiftCompleteINTStatus` reader - Shift Complete Interrupt Status"]
pub type ShiftCompleteIntstatusR = crate::BitReader<ShiftCompleteIntstatus>;
impl ShiftCompleteIntstatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ShiftCompleteIntstatus {
        match self.bits {
            false => ShiftCompleteIntstatus::NoInterruptPending,
            true => ShiftCompleteIntstatus::InterruptPending,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_no_interrupt_pending(&self) -> bool {
        *self == ShiftCompleteIntstatus::NoInterruptPending
    }
    #[doc = "interrupt pending"]
    #[inline(always)]
    pub fn is_interrupt_pending(&self) -> bool {
        *self == ShiftCompleteIntstatus::InterruptPending
    }
}
#[doc = "Field `ShiftCompleteINTStatus` writer - Shift Complete Interrupt Status"]
pub type ShiftCompleteIntstatusW<'a, REG> = crate::BitWriter<'a, REG, ShiftCompleteIntstatus>;
impl<'a, REG> ShiftCompleteIntstatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn no_interrupt_pending(self) -> &'a mut crate::W<REG> {
        self.variant(ShiftCompleteIntstatus::NoInterruptPending)
    }
    #[doc = "interrupt pending"]
    #[inline(always)]
    pub fn interrupt_pending(self) -> &'a mut crate::W<REG> {
        self.variant(ShiftCompleteIntstatus::InterruptPending)
    }
}
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader<u16>;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `ShiftCompleteINTEnbl` reader - Shift Complete Interrupt Enable"]
pub type ShiftCompleteIntenblR = crate::BitReader;
#[doc = "Field `ShiftCompleteINTEnbl` writer - Shift Complete Interrupt Enable"]
pub type ShiftCompleteIntenblW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Shift Complete Interrupt Status"]
    #[inline(always)]
    pub fn shift_complete_intstatus(&self) -> ShiftCompleteIntstatusR {
        ShiftCompleteIntstatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:15 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 0x7fff) as u16)
    }
    #[doc = "Bit 16 - Shift Complete Interrupt Enable"]
    #[inline(always)]
    pub fn shift_complete_intenbl(&self) -> ShiftCompleteIntenblR {
        ShiftCompleteIntenblR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shift Complete Interrupt Status"]
    #[inline(always)]
    pub fn shift_complete_intstatus(&mut self) -> ShiftCompleteIntstatusW<Jtag038Spec> {
        ShiftCompleteIntstatusW::new(self, 0)
    }
    #[doc = "Bits 1:15 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Jtag038Spec> {
        Reserved1W::new(self, 1)
    }
    #[doc = "Bit 16 - Shift Complete Interrupt Enable"]
    #[inline(always)]
    pub fn shift_complete_intenbl(&mut self) -> ShiftCompleteIntenblW<Jtag038Spec> {
        ShiftCompleteIntenblW::new(self, 16)
    }
}
#[doc = "Interrupt control\n\nYou can [`read`](crate::Reg::read) this register and get [`jtag038::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jtag038::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Jtag038Spec;
impl crate::RegisterSpec for Jtag038Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jtag038::R`](R) reader structure"]
impl crate::Readable for Jtag038Spec {}
#[doc = "`write(|w| ..)` method takes [`jtag038::W`](W) writer structure"]
impl crate::Writable for Jtag038Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets JTAG038 to value 0"]
impl crate::Resettable for Jtag038Spec {}
