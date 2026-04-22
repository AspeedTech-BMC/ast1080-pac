#[doc = "Register `SPIF0F0` reader"]
pub type R = crate::R<Spif0f0Spec>;
#[doc = "Register `SPIF0F0` writer"]
pub type W = crate::W<Spif0f0Spec>;
#[doc = "Field `WTABLE28` reader - WTABLE28"]
pub type Wtable28R = crate::FieldReader<u32>;
#[doc = "Field `WTABLE28` writer - WTABLE28"]
pub type Wtable28W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WTABLE28"]
    #[inline(always)]
    pub fn wtable28(&self) -> Wtable28R {
        Wtable28R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WTABLE28"]
    #[inline(always)]
    pub fn wtable28(&mut self) -> Wtable28W<Spif0f0Spec> {
        Wtable28W::new(self, 0)
    }
}
#[doc = "SPIF\\_WTABLE28\n\nYou can [`read`](crate::Reg::read) this register and get [`spif0f0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif0f0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif0f0Spec;
impl crate::RegisterSpec for Spif0f0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif0f0::R`](R) reader structure"]
impl crate::Readable for Spif0f0Spec {}
#[doc = "`write(|w| ..)` method takes [`spif0f0::W`](W) writer structure"]
impl crate::Writable for Spif0f0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF0F0 to value 0"]
impl crate::Resettable for Spif0f0Spec {}
