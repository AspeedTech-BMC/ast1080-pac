#[doc = "Register `SPIF0B0` reader"]
pub type R = crate::R<Spif0b0Spec>;
#[doc = "Register `SPIF0B0` writer"]
pub type W = crate::W<Spif0b0Spec>;
#[doc = "Field `WTABLE12` reader - WTABLE12"]
pub type Wtable12R = crate::FieldReader<u32>;
#[doc = "Field `WTABLE12` writer - WTABLE12"]
pub type Wtable12W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WTABLE12"]
    #[inline(always)]
    pub fn wtable12(&self) -> Wtable12R {
        Wtable12R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WTABLE12"]
    #[inline(always)]
    pub fn wtable12(&mut self) -> Wtable12W<Spif0b0Spec> {
        Wtable12W::new(self, 0)
    }
}
#[doc = "SPIF\\_WTABLE12\n\nYou can [`read`](crate::Reg::read) this register and get [`spif0b0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif0b0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif0b0Spec;
impl crate::RegisterSpec for Spif0b0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif0b0::R`](R) reader structure"]
impl crate::Readable for Spif0b0Spec {}
#[doc = "`write(|w| ..)` method takes [`spif0b0::W`](W) writer structure"]
impl crate::Writable for Spif0b0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF0B0 to value 0"]
impl crate::Resettable for Spif0b0Spec {}
