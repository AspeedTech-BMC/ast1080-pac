#[doc = "Register `SPIF0F8` reader"]
pub type R = crate::R<Spif0f8Spec>;
#[doc = "Register `SPIF0F8` writer"]
pub type W = crate::W<Spif0f8Spec>;
#[doc = "Field `WTABLE30` reader - WTABLE30"]
pub type Wtable30R = crate::FieldReader<u32>;
#[doc = "Field `WTABLE30` writer - WTABLE30"]
pub type Wtable30W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WTABLE30"]
    #[inline(always)]
    pub fn wtable30(&self) -> Wtable30R {
        Wtable30R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WTABLE30"]
    #[inline(always)]
    pub fn wtable30(&mut self) -> Wtable30W<Spif0f8Spec> {
        Wtable30W::new(self, 0)
    }
}
#[doc = "SPIF\\_WTABLE30\n\nYou can [`read`](crate::Reg::read) this register and get [`spif0f8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif0f8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif0f8Spec;
impl crate::RegisterSpec for Spif0f8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif0f8::R`](R) reader structure"]
impl crate::Readable for Spif0f8Spec {}
#[doc = "`write(|w| ..)` method takes [`spif0f8::W`](W) writer structure"]
impl crate::Writable for Spif0f8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF0F8 to value 0"]
impl crate::Resettable for Spif0f8Spec {}
