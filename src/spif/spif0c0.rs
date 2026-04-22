#[doc = "Register `SPIF0C0` reader"]
pub type R = crate::R<Spif0c0Spec>;
#[doc = "Register `SPIF0C0` writer"]
pub type W = crate::W<Spif0c0Spec>;
#[doc = "Field `WTABLE16` reader - WTABLE16"]
pub type Wtable16R = crate::FieldReader<u32>;
#[doc = "Field `WTABLE16` writer - WTABLE16"]
pub type Wtable16W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WTABLE16"]
    #[inline(always)]
    pub fn wtable16(&self) -> Wtable16R {
        Wtable16R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WTABLE16"]
    #[inline(always)]
    pub fn wtable16(&mut self) -> Wtable16W<Spif0c0Spec> {
        Wtable16W::new(self, 0)
    }
}
#[doc = "SPIF\\_WTABLE16\n\nYou can [`read`](crate::Reg::read) this register and get [`spif0c0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif0c0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif0c0Spec;
impl crate::RegisterSpec for Spif0c0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif0c0::R`](R) reader structure"]
impl crate::Readable for Spif0c0Spec {}
#[doc = "`write(|w| ..)` method takes [`spif0c0::W`](W) writer structure"]
impl crate::Writable for Spif0c0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF0C0 to value 0"]
impl crate::Resettable for Spif0c0Spec {}
