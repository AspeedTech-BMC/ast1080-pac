#[doc = "Register `SPIF098` reader"]
pub type R = crate::R<Spif098Spec>;
#[doc = "Register `SPIF098` writer"]
pub type W = crate::W<Spif098Spec>;
#[doc = "Field `WTABLE06` reader - WTABLE06"]
pub type Wtable06R = crate::FieldReader<u32>;
#[doc = "Field `WTABLE06` writer - WTABLE06"]
pub type Wtable06W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WTABLE06"]
    #[inline(always)]
    pub fn wtable06(&self) -> Wtable06R {
        Wtable06R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WTABLE06"]
    #[inline(always)]
    pub fn wtable06(&mut self) -> Wtable06W<Spif098Spec> {
        Wtable06W::new(self, 0)
    }
}
#[doc = "SPIF\\_WTABLE06\n\nYou can [`read`](crate::Reg::read) this register and get [`spif098::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif098::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif098Spec;
impl crate::RegisterSpec for Spif098Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif098::R`](R) reader structure"]
impl crate::Readable for Spif098Spec {}
#[doc = "`write(|w| ..)` method takes [`spif098::W`](W) writer structure"]
impl crate::Writable for Spif098Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF098 to value 0"]
impl crate::Resettable for Spif098Spec {}
