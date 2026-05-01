#[doc = "Register `VIC400` reader"]
pub type R = crate::R<Vic400Spec>;
#[doc = "Register `VIC400` writer"]
pub type W = crate::W<Vic400Spec>;
#[doc = "Field `VICSIRQCSEL30` reader - VIC_SIRQ_CSEL3_0"]
pub type Vicsirqcsel30R = crate::FieldReader<u32>;
#[doc = "Field `VICSIRQCSEL30` writer - VIC_SIRQ_CSEL3_0"]
pub type Vicsirqcsel30W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - VIC_SIRQ_CSEL3_0"]
    #[inline(always)]
    pub fn vicsirqcsel30(&self) -> Vicsirqcsel30R {
        Vicsirqcsel30R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - VIC_SIRQ_CSEL3_0"]
    #[inline(always)]
    pub fn vicsirqcsel30(&mut self) -> Vicsirqcsel30W<Vic400Spec> {
        Vicsirqcsel30W::new(self, 0)
    }
}
#[doc = "Int Routing Select3 0\n\nYou can [`read`](crate::Reg::read) this register and get [`vic400::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic400::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic400Spec;
impl crate::RegisterSpec for Vic400Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic400::R`](R) reader structure"]
impl crate::Readable for Vic400Spec {}
#[doc = "`write(|w| ..)` method takes [`vic400::W`](W) writer structure"]
impl crate::Writable for Vic400Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC400 to value 0"]
impl crate::Resettable for Vic400Spec {}
