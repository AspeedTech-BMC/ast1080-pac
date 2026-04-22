#[doc = "Register `SPIF050` reader"]
pub type R = crate::R<Spif050Spec>;
#[doc = "Register `SPIF050` writer"]
pub type W = crate::W<Spif050Spec>;
#[doc = "Field `ELOG04` reader - ELOG04"]
pub type Elog04R = crate::FieldReader<u32>;
#[doc = "Field `ELOG04` writer - ELOG04"]
pub type Elog04W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ELOG04"]
    #[inline(always)]
    pub fn elog04(&self) -> Elog04R {
        Elog04R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ELOG04"]
    #[inline(always)]
    pub fn elog04(&mut self) -> Elog04W<Spif050Spec> {
        Elog04W::new(self, 0)
    }
}
#[doc = "SPIF\\_ELOG04\n\nYou can [`read`](crate::Reg::read) this register and get [`spif050::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif050::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif050Spec;
impl crate::RegisterSpec for Spif050Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif050::R`](R) reader structure"]
impl crate::Readable for Spif050Spec {}
#[doc = "`write(|w| ..)` method takes [`spif050::W`](W) writer structure"]
impl crate::Writable for Spif050Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF050 to value 0"]
impl crate::Resettable for Spif050Spec {}
