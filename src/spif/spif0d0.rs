#[doc = "Register `SPIF0D0` reader"]
pub type R = crate::R<Spif0d0Spec>;
#[doc = "Register `SPIF0D0` writer"]
pub type W = crate::W<Spif0d0Spec>;
#[doc = "Field `WTABLE20` reader - WTABLE20"]
pub type Wtable20R = crate::FieldReader<u32>;
#[doc = "Field `WTABLE20` writer - WTABLE20"]
pub type Wtable20W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WTABLE20"]
    #[inline(always)]
    pub fn wtable20(&self) -> Wtable20R {
        Wtable20R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WTABLE20"]
    #[inline(always)]
    pub fn wtable20(&mut self) -> Wtable20W<Spif0d0Spec> {
        Wtable20W::new(self, 0)
    }
}
#[doc = "SPIF\\_WTABLE20\n\nYou can [`read`](crate::Reg::read) this register and get [`spif0d0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif0d0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif0d0Spec;
impl crate::RegisterSpec for Spif0d0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif0d0::R`](R) reader structure"]
impl crate::Readable for Spif0d0Spec {}
#[doc = "`write(|w| ..)` method takes [`spif0d0::W`](W) writer structure"]
impl crate::Writable for Spif0d0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF0D0 to value 0"]
impl crate::Resettable for Spif0d0Spec {}
