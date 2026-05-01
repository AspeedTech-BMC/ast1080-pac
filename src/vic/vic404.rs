#[doc = "Register `VIC404` reader"]
pub type R = crate::R<Vic404Spec>;
#[doc = "Register `VIC404` writer"]
pub type W = crate::W<Vic404Spec>;
#[doc = "Field `VICSIRQCSEL31` reader - VIC_SIRQ_CSEL3_1"]
pub type Vicsirqcsel31R = crate::FieldReader<u32>;
#[doc = "Field `VICSIRQCSEL31` writer - VIC_SIRQ_CSEL3_1"]
pub type Vicsirqcsel31W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - VIC_SIRQ_CSEL3_1"]
    #[inline(always)]
    pub fn vicsirqcsel31(&self) -> Vicsirqcsel31R {
        Vicsirqcsel31R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - VIC_SIRQ_CSEL3_1"]
    #[inline(always)]
    pub fn vicsirqcsel31(&mut self) -> Vicsirqcsel31W<Vic404Spec> {
        Vicsirqcsel31W::new(self, 0)
    }
}
#[doc = "Int Routing Select3 1\n\nYou can [`read`](crate::Reg::read) this register and get [`vic404::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic404::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic404Spec;
impl crate::RegisterSpec for Vic404Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic404::R`](R) reader structure"]
impl crate::Readable for Vic404Spec {}
#[doc = "`write(|w| ..)` method takes [`vic404::W`](W) writer structure"]
impl crate::Writable for Vic404Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC404 to value 0"]
impl crate::Resettable for Vic404Spec {}
