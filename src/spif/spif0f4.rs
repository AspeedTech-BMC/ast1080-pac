#[doc = "Register `SPIF0F4` reader"]
pub type R = crate::R<Spif0f4Spec>;
#[doc = "Register `SPIF0F4` writer"]
pub type W = crate::W<Spif0f4Spec>;
#[doc = "Field `WTABLE29` reader - WTABLE29"]
pub type Wtable29R = crate::FieldReader<u32>;
#[doc = "Field `WTABLE29` writer - WTABLE29"]
pub type Wtable29W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WTABLE29"]
    #[inline(always)]
    pub fn wtable29(&self) -> Wtable29R {
        Wtable29R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WTABLE29"]
    #[inline(always)]
    pub fn wtable29(&mut self) -> Wtable29W<Spif0f4Spec> {
        Wtable29W::new(self, 0)
    }
}
#[doc = "SPIF\\_WTABLE29\n\nYou can [`read`](crate::Reg::read) this register and get [`spif0f4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif0f4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif0f4Spec;
impl crate::RegisterSpec for Spif0f4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif0f4::R`](R) reader structure"]
impl crate::Readable for Spif0f4Spec {}
#[doc = "`write(|w| ..)` method takes [`spif0f4::W`](W) writer structure"]
impl crate::Writable for Spif0f4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF0F4 to value 0"]
impl crate::Resettable for Spif0f4Spec {}
