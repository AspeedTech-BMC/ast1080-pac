#[doc = "Register `SPIF0C8` reader"]
pub type R = crate::R<Spif0c8Spec>;
#[doc = "Register `SPIF0C8` writer"]
pub type W = crate::W<Spif0c8Spec>;
#[doc = "Field `WTABLE18` reader - WTABLE18"]
pub type Wtable18R = crate::FieldReader<u32>;
#[doc = "Field `WTABLE18` writer - WTABLE18"]
pub type Wtable18W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WTABLE18"]
    #[inline(always)]
    pub fn wtable18(&self) -> Wtable18R {
        Wtable18R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WTABLE18"]
    #[inline(always)]
    pub fn wtable18(&mut self) -> Wtable18W<Spif0c8Spec> {
        Wtable18W::new(self, 0)
    }
}
#[doc = "SPIF\\_WTABLE18\n\nYou can [`read`](crate::Reg::read) this register and get [`spif0c8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif0c8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif0c8Spec;
impl crate::RegisterSpec for Spif0c8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif0c8::R`](R) reader structure"]
impl crate::Readable for Spif0c8Spec {}
#[doc = "`write(|w| ..)` method takes [`spif0c8::W`](W) writer structure"]
impl crate::Writable for Spif0c8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF0C8 to value 0"]
impl crate::Resettable for Spif0c8Spec {}
