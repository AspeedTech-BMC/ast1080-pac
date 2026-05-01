#[doc = "Register `VIC308` reader"]
pub type R = crate::R<Vic308Spec>;
#[doc = "Register `VIC308` writer"]
pub type W = crate::W<Vic308Spec>;
#[doc = "Field `VICSIRQCSEL22` reader - VIC_SIRQ_CSEL2_2"]
pub type Vicsirqcsel22R = crate::FieldReader<u32>;
#[doc = "Field `VICSIRQCSEL22` writer - VIC_SIRQ_CSEL2_2"]
pub type Vicsirqcsel22W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - VIC_SIRQ_CSEL2_2"]
    #[inline(always)]
    pub fn vicsirqcsel22(&self) -> Vicsirqcsel22R {
        Vicsirqcsel22R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - VIC_SIRQ_CSEL2_2"]
    #[inline(always)]
    pub fn vicsirqcsel22(&mut self) -> Vicsirqcsel22W<Vic308Spec> {
        Vicsirqcsel22W::new(self, 0)
    }
}
#[doc = "Int Routing Select2 2\n\nYou can [`read`](crate::Reg::read) this register and get [`vic308::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic308::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic308Spec;
impl crate::RegisterSpec for Vic308Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic308::R`](R) reader structure"]
impl crate::Readable for Vic308Spec {}
#[doc = "`write(|w| ..)` method takes [`vic308::W`](W) writer structure"]
impl crate::Writable for Vic308Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC308 to value 0"]
impl crate::Resettable for Vic308Spec {}
