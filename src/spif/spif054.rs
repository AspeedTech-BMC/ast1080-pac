#[doc = "Register `SPIF054` reader"]
pub type R = crate::R<Spif054Spec>;
#[doc = "Register `SPIF054` writer"]
pub type W = crate::W<Spif054Spec>;
#[doc = "Field `ELOG05` reader - ELOG05"]
pub type Elog05R = crate::FieldReader<u32>;
#[doc = "Field `ELOG05` writer - ELOG05"]
pub type Elog05W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ELOG05"]
    #[inline(always)]
    pub fn elog05(&self) -> Elog05R {
        Elog05R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ELOG05"]
    #[inline(always)]
    pub fn elog05(&mut self) -> Elog05W<Spif054Spec> {
        Elog05W::new(self, 0)
    }
}
#[doc = "SPIF\\_ELOG05\n\nYou can [`read`](crate::Reg::read) this register and get [`spif054::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif054::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif054Spec;
impl crate::RegisterSpec for Spif054Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif054::R`](R) reader structure"]
impl crate::Readable for Spif054Spec {}
#[doc = "`write(|w| ..)` method takes [`spif054::W`](W) writer structure"]
impl crate::Writable for Spif054Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF054 to value 0"]
impl crate::Resettable for Spif054Spec {}
