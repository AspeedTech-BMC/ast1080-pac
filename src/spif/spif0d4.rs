#[doc = "Register `SPIF0D4` reader"]
pub type R = crate::R<Spif0d4Spec>;
#[doc = "Register `SPIF0D4` writer"]
pub type W = crate::W<Spif0d4Spec>;
#[doc = "Field `WTABLE21` reader - WTABLE21"]
pub type Wtable21R = crate::FieldReader<u32>;
#[doc = "Field `WTABLE21` writer - WTABLE21"]
pub type Wtable21W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WTABLE21"]
    #[inline(always)]
    pub fn wtable21(&self) -> Wtable21R {
        Wtable21R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WTABLE21"]
    #[inline(always)]
    pub fn wtable21(&mut self) -> Wtable21W<Spif0d4Spec> {
        Wtable21W::new(self, 0)
    }
}
#[doc = "SPIF\\_WTABLE21\n\nYou can [`read`](crate::Reg::read) this register and get [`spif0d4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif0d4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif0d4Spec;
impl crate::RegisterSpec for Spif0d4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif0d4::R`](R) reader structure"]
impl crate::Readable for Spif0d4Spec {}
#[doc = "`write(|w| ..)` method takes [`spif0d4::W`](W) writer structure"]
impl crate::Writable for Spif0d4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF0D4 to value 0"]
impl crate::Resettable for Spif0d4Spec {}
