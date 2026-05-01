#[doc = "Register `VIC304` reader"]
pub type R = crate::R<Vic304Spec>;
#[doc = "Register `VIC304` writer"]
pub type W = crate::W<Vic304Spec>;
#[doc = "Field `VICSIRQCSEL21` reader - VIC_SIRQ_CSEL2_1"]
pub type Vicsirqcsel21R = crate::FieldReader<u32>;
#[doc = "Field `VICSIRQCSEL21` writer - VIC_SIRQ_CSEL2_1"]
pub type Vicsirqcsel21W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - VIC_SIRQ_CSEL2_1"]
    #[inline(always)]
    pub fn vicsirqcsel21(&self) -> Vicsirqcsel21R {
        Vicsirqcsel21R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - VIC_SIRQ_CSEL2_1"]
    #[inline(always)]
    pub fn vicsirqcsel21(&mut self) -> Vicsirqcsel21W<Vic304Spec> {
        Vicsirqcsel21W::new(self, 0)
    }
}
#[doc = "Int Routing Select2 1\n\nYou can [`read`](crate::Reg::read) this register and get [`vic304::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic304::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic304Spec;
impl crate::RegisterSpec for Vic304Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic304::R`](R) reader structure"]
impl crate::Readable for Vic304Spec {}
#[doc = "`write(|w| ..)` method takes [`vic304::W`](W) writer structure"]
impl crate::Writable for Vic304Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC304 to value 0"]
impl crate::Resettable for Vic304Spec {}
