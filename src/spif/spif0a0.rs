#[doc = "Register `SPIF0A0` reader"]
pub type R = crate::R<Spif0a0Spec>;
#[doc = "Register `SPIF0A0` writer"]
pub type W = crate::W<Spif0a0Spec>;
#[doc = "Field `WTABLE08` reader - WTABLE08"]
pub type Wtable08R = crate::FieldReader<u32>;
#[doc = "Field `WTABLE08` writer - WTABLE08"]
pub type Wtable08W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WTABLE08"]
    #[inline(always)]
    pub fn wtable08(&self) -> Wtable08R {
        Wtable08R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WTABLE08"]
    #[inline(always)]
    pub fn wtable08(&mut self) -> Wtable08W<Spif0a0Spec> {
        Wtable08W::new(self, 0)
    }
}
#[doc = "SPIF\\_WTABLE08\n\nYou can [`read`](crate::Reg::read) this register and get [`spif0a0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif0a0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif0a0Spec;
impl crate::RegisterSpec for Spif0a0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif0a0::R`](R) reader structure"]
impl crate::Readable for Spif0a0Spec {}
#[doc = "`write(|w| ..)` method takes [`spif0a0::W`](W) writer structure"]
impl crate::Writable for Spif0a0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF0A0 to value 0"]
impl crate::Resettable for Spif0a0Spec {}
