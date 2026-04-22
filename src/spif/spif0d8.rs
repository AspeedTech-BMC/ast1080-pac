#[doc = "Register `SPIF0D8` reader"]
pub type R = crate::R<Spif0d8Spec>;
#[doc = "Register `SPIF0D8` writer"]
pub type W = crate::W<Spif0d8Spec>;
#[doc = "Field `WTABLE22` reader - WTABLE22"]
pub type Wtable22R = crate::FieldReader<u32>;
#[doc = "Field `WTABLE22` writer - WTABLE22"]
pub type Wtable22W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WTABLE22"]
    #[inline(always)]
    pub fn wtable22(&self) -> Wtable22R {
        Wtable22R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WTABLE22"]
    #[inline(always)]
    pub fn wtable22(&mut self) -> Wtable22W<Spif0d8Spec> {
        Wtable22W::new(self, 0)
    }
}
#[doc = "SPIF\\_WTABLE22\n\nYou can [`read`](crate::Reg::read) this register and get [`spif0d8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif0d8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif0d8Spec;
impl crate::RegisterSpec for Spif0d8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif0d8::R`](R) reader structure"]
impl crate::Readable for Spif0d8Spec {}
#[doc = "`write(|w| ..)` method takes [`spif0d8::W`](W) writer structure"]
impl crate::Writable for Spif0d8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF0D8 to value 0"]
impl crate::Resettable for Spif0d8Spec {}
