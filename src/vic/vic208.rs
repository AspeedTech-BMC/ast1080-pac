#[doc = "Register `VIC208` reader"]
pub type R = crate::R<Vic208Spec>;
#[doc = "Register `VIC208` writer"]
pub type W = crate::W<Vic208Spec>;
#[doc = "Field `VICSIRQCSEL2` reader - VIC_SIRQ_CSEL_2"]
pub type Vicsirqcsel2R = crate::FieldReader<u32>;
#[doc = "Field `VICSIRQCSEL2` writer - VIC_SIRQ_CSEL_2"]
pub type Vicsirqcsel2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - VIC_SIRQ_CSEL_2"]
    #[inline(always)]
    pub fn vicsirqcsel2(&self) -> Vicsirqcsel2R {
        Vicsirqcsel2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - VIC_SIRQ_CSEL_2"]
    #[inline(always)]
    pub fn vicsirqcsel2(&mut self) -> Vicsirqcsel2W<Vic208Spec> {
        Vicsirqcsel2W::new(self, 0)
    }
}
#[doc = "Int Routing Select 2\n\nYou can [`read`](crate::Reg::read) this register and get [`vic208::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic208::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic208Spec;
impl crate::RegisterSpec for Vic208Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic208::R`](R) reader structure"]
impl crate::Readable for Vic208Spec {}
#[doc = "`write(|w| ..)` method takes [`vic208::W`](W) writer structure"]
impl crate::Writable for Vic208Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC208 to value 0"]
impl crate::Resettable for Vic208Spec {}
