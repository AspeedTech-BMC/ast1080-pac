#[doc = "Register `SPIF0B4` reader"]
pub type R = crate::R<Spif0b4Spec>;
#[doc = "Register `SPIF0B4` writer"]
pub type W = crate::W<Spif0b4Spec>;
#[doc = "Field `WTABLE13` reader - WTABLE13"]
pub type Wtable13R = crate::FieldReader<u32>;
#[doc = "Field `WTABLE13` writer - WTABLE13"]
pub type Wtable13W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WTABLE13"]
    #[inline(always)]
    pub fn wtable13(&self) -> Wtable13R {
        Wtable13R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WTABLE13"]
    #[inline(always)]
    pub fn wtable13(&mut self) -> Wtable13W<Spif0b4Spec> {
        Wtable13W::new(self, 0)
    }
}
#[doc = "SPIF\\_WTABLE13\n\nYou can [`read`](crate::Reg::read) this register and get [`spif0b4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif0b4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif0b4Spec;
impl crate::RegisterSpec for Spif0b4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif0b4::R`](R) reader structure"]
impl crate::Readable for Spif0b4Spec {}
#[doc = "`write(|w| ..)` method takes [`spif0b4::W`](W) writer structure"]
impl crate::Writable for Spif0b4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF0B4 to value 0"]
impl crate::Resettable for Spif0b4Spec {}
