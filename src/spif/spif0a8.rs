#[doc = "Register `SPIF0A8` reader"]
pub type R = crate::R<Spif0a8Spec>;
#[doc = "Register `SPIF0A8` writer"]
pub type W = crate::W<Spif0a8Spec>;
#[doc = "Field `WTABLE10` reader - WTABLE10"]
pub type Wtable10R = crate::FieldReader<u32>;
#[doc = "Field `WTABLE10` writer - WTABLE10"]
pub type Wtable10W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WTABLE10"]
    #[inline(always)]
    pub fn wtable10(&self) -> Wtable10R {
        Wtable10R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WTABLE10"]
    #[inline(always)]
    pub fn wtable10(&mut self) -> Wtable10W<Spif0a8Spec> {
        Wtable10W::new(self, 0)
    }
}
#[doc = "SPIF\\_WTABLE10\n\nYou can [`read`](crate::Reg::read) this register and get [`spif0a8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif0a8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif0a8Spec;
impl crate::RegisterSpec for Spif0a8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif0a8::R`](R) reader structure"]
impl crate::Readable for Spif0a8Spec {}
#[doc = "`write(|w| ..)` method takes [`spif0a8::W`](W) writer structure"]
impl crate::Writable for Spif0a8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF0A8 to value 0"]
impl crate::Resettable for Spif0a8Spec {}
