#[doc = "Register `SPIF0C4` reader"]
pub type R = crate::R<Spif0c4Spec>;
#[doc = "Register `SPIF0C4` writer"]
pub type W = crate::W<Spif0c4Spec>;
#[doc = "Field `WTABLE17` reader - WTABLE17"]
pub type Wtable17R = crate::FieldReader<u32>;
#[doc = "Field `WTABLE17` writer - WTABLE17"]
pub type Wtable17W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WTABLE17"]
    #[inline(always)]
    pub fn wtable17(&self) -> Wtable17R {
        Wtable17R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WTABLE17"]
    #[inline(always)]
    pub fn wtable17(&mut self) -> Wtable17W<Spif0c4Spec> {
        Wtable17W::new(self, 0)
    }
}
#[doc = "SPIF\\_WTABLE17\n\nYou can [`read`](crate::Reg::read) this register and get [`spif0c4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif0c4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif0c4Spec;
impl crate::RegisterSpec for Spif0c4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif0c4::R`](R) reader structure"]
impl crate::Readable for Spif0c4Spec {}
#[doc = "`write(|w| ..)` method takes [`spif0c4::W`](W) writer structure"]
impl crate::Writable for Spif0c4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF0C4 to value 0"]
impl crate::Resettable for Spif0c4Spec {}
