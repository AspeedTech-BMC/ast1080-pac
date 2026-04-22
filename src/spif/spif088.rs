#[doc = "Register `SPIF088` reader"]
pub type R = crate::R<Spif088Spec>;
#[doc = "Register `SPIF088` writer"]
pub type W = crate::W<Spif088Spec>;
#[doc = "Field `WTABLE02` reader - WTABLE02"]
pub type Wtable02R = crate::FieldReader<u32>;
#[doc = "Field `WTABLE02` writer - WTABLE02"]
pub type Wtable02W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WTABLE02"]
    #[inline(always)]
    pub fn wtable02(&self) -> Wtable02R {
        Wtable02R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WTABLE02"]
    #[inline(always)]
    pub fn wtable02(&mut self) -> Wtable02W<Spif088Spec> {
        Wtable02W::new(self, 0)
    }
}
#[doc = "SPIF\\_WTABLE02\n\nYou can [`read`](crate::Reg::read) this register and get [`spif088::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif088::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif088Spec;
impl crate::RegisterSpec for Spif088Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif088::R`](R) reader structure"]
impl crate::Readable for Spif088Spec {}
#[doc = "`write(|w| ..)` method takes [`spif088::W`](W) writer structure"]
impl crate::Writable for Spif088Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF088 to value 0"]
impl crate::Resettable for Spif088Spec {}
