#[doc = "Register `UARTDMA008` reader"]
pub type R = crate::R<Uartdma008Spec>;
#[doc = "Register `UARTDMA008` writer"]
pub type W = crate::W<Uartdma008Spec>;
#[doc = "Field `Reserved02` reader - reserved(0)"]
pub type Reserved02R = crate::FieldReader;
#[doc = "Field `Reserved02` writer - reserved(0)"]
pub type Reserved02W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "disable time out\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisableTimeOut {
    #[doc = "0: enable time out timer"]
    EnableTimeOutTimer = 0,
    #[doc = "1: disable time out timer"]
    DisableTimeOutTimer = 1,
}
impl From<DisableTimeOut> for bool {
    #[inline(always)]
    fn from(variant: DisableTimeOut) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DisableTimeOut` reader - disable time out"]
pub type DisableTimeOutR = crate::BitReader<DisableTimeOut>;
impl DisableTimeOutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DisableTimeOut {
        match self.bits {
            false => DisableTimeOut::EnableTimeOutTimer,
            true => DisableTimeOut::DisableTimeOutTimer,
        }
    }
    #[doc = "enable time out timer"]
    #[inline(always)]
    pub fn is_enable_time_out_timer(&self) -> bool {
        *self == DisableTimeOut::EnableTimeOutTimer
    }
    #[doc = "disable time out timer"]
    #[inline(always)]
    pub fn is_disable_time_out_timer(&self) -> bool {
        *self == DisableTimeOut::DisableTimeOutTimer
    }
}
#[doc = "Field `DisableTimeOut` writer - disable time out"]
pub type DisableTimeOutW<'a, REG> = crate::BitWriter<'a, REG, DisableTimeOut>;
impl<'a, REG> DisableTimeOutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable time out timer"]
    #[inline(always)]
    pub fn enable_time_out_timer(self) -> &'a mut crate::W<REG> {
        self.variant(DisableTimeOut::EnableTimeOutTimer)
    }
    #[doc = "disable time out timer"]
    #[inline(always)]
    pub fn disable_time_out_timer(self) -> &'a mut crate::W<REG> {
        self.variant(DisableTimeOut::DisableTimeOutTimer)
    }
}
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader<u32>;
#[doc = "Field `Reserved01` reader - reserved(0)"]
pub type Reserved01R = crate::FieldReader;
#[doc = "Field `Reserved01` writer - reserved(0)"]
pub type Reserved01W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - reserved(0)"]
    #[inline(always)]
    pub fn reserved02(&self) -> Reserved02R {
        Reserved02R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - disable time out"]
    #[inline(always)]
    pub fn disable_time_out(&self) -> DisableTimeOutR {
        DisableTimeOutR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 5) & 0x07ff_ffff)
    }
    #[doc = "Bits 5:6 - reserved(0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 5) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - reserved(0)"]
    #[inline(always)]
    pub fn reserved02(&mut self) -> Reserved02W<Uartdma008Spec> {
        Reserved02W::new(self, 0)
    }
    #[doc = "Bit 4 - disable time out"]
    #[inline(always)]
    pub fn disable_time_out(&mut self) -> DisableTimeOutW<Uartdma008Spec> {
        DisableTimeOutW::new(self, 4)
    }
    #[doc = "Bits 5:6 - reserved(0)"]
    #[inline(always)]
    pub fn reserved01(&mut self) -> Reserved01W<Uartdma008Spec> {
        Reserved01W::new(self, 5)
    }
}
#[doc = "Misc control\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma008::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma008::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma008Spec;
impl crate::RegisterSpec for Uartdma008Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma008::R`](R) reader structure"]
impl crate::Readable for Uartdma008Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma008::W`](W) writer structure"]
impl crate::Writable for Uartdma008Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA008 to value 0"]
impl crate::Resettable for Uartdma008Spec {}
