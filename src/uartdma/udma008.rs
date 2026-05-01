#[doc = "Register `UDMA008` reader"]
pub type R = crate::R<Udma008Spec>;
#[doc = "Register `UDMA008` writer"]
pub type W = crate::W<Udma008Spec>;
#[doc = "Field `Reserved2` reader - reserved(0)"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `Reserved2` writer - reserved(0)"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DisableTimeOut` reader - disable time out"]
pub type DisableTimeOutR = crate::BitReader;
#[doc = "Field `DisableTimeOut` writer - disable time out"]
pub type DisableTimeOutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved(0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - reserved(0)"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - disable time out"]
    #[inline(always)]
    pub fn disable_time_out(&self) -> DisableTimeOutR {
        DisableTimeOutR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 5) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Udma008Spec> {
        Reserved2W::new(self, 0)
    }
    #[doc = "Bit 4 - disable time out"]
    #[inline(always)]
    pub fn disable_time_out(&mut self) -> DisableTimeOutW<Udma008Spec> {
        DisableTimeOutW::new(self, 4)
    }
    #[doc = "Bits 5:6 - reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Udma008Spec> {
        Reserved1W::new(self, 5)
    }
}
#[doc = "Misc control\n\nYou can [`read`](crate::Reg::read) this register and get [`udma008::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma008::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma008Spec;
impl crate::RegisterSpec for Udma008Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma008::R`](R) reader structure"]
impl crate::Readable for Udma008Spec {}
#[doc = "`write(|w| ..)` method takes [`udma008::W`](W) writer structure"]
impl crate::Writable for Udma008Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA008 to value 0"]
impl crate::Resettable for Udma008Spec {}
