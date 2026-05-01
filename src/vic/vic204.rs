#[doc = "Register `VIC204` reader"]
pub type R = crate::R<Vic204Spec>;
#[doc = "Register `VIC204` writer"]
pub type W = crate::W<Vic204Spec>;
#[doc = "Field `VICSIRQCSEL1` reader - VIC_SIRQ_CSEL_1"]
pub type Vicsirqcsel1R = crate::FieldReader<u32>;
#[doc = "Field `VICSIRQCSEL1` writer - VIC_SIRQ_CSEL_1"]
pub type Vicsirqcsel1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - VIC_SIRQ_CSEL_1"]
    #[inline(always)]
    pub fn vicsirqcsel1(&self) -> Vicsirqcsel1R {
        Vicsirqcsel1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - VIC_SIRQ_CSEL_1"]
    #[inline(always)]
    pub fn vicsirqcsel1(&mut self) -> Vicsirqcsel1W<Vic204Spec> {
        Vicsirqcsel1W::new(self, 0)
    }
}
#[doc = "Int Routing Select 1\n\nYou can [`read`](crate::Reg::read) this register and get [`vic204::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic204::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic204Spec;
impl crate::RegisterSpec for Vic204Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic204::R`](R) reader structure"]
impl crate::Readable for Vic204Spec {}
#[doc = "`write(|w| ..)` method takes [`vic204::W`](W) writer structure"]
impl crate::Writable for Vic204Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC204 to value 0"]
impl crate::Resettable for Vic204Spec {}
