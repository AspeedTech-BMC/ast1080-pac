#[doc = "Register `VIC310` reader"]
pub type R = crate::R<Vic310Spec>;
#[doc = "Register `VIC310` writer"]
pub type W = crate::W<Vic310Spec>;
#[doc = "Field `VICSIRQCSEL24` reader - VIC_SIRQ_CSEL2_4"]
pub type Vicsirqcsel24R = crate::FieldReader<u32>;
#[doc = "Field `VICSIRQCSEL24` writer - VIC_SIRQ_CSEL2_4"]
pub type Vicsirqcsel24W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - VIC_SIRQ_CSEL2_4"]
    #[inline(always)]
    pub fn vicsirqcsel24(&self) -> Vicsirqcsel24R {
        Vicsirqcsel24R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - VIC_SIRQ_CSEL2_4"]
    #[inline(always)]
    pub fn vicsirqcsel24(&mut self) -> Vicsirqcsel24W<Vic310Spec> {
        Vicsirqcsel24W::new(self, 0)
    }
}
#[doc = "Int Routing Select2 4\n\nYou can [`read`](crate::Reg::read) this register and get [`vic310::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic310::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic310Spec;
impl crate::RegisterSpec for Vic310Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic310::R`](R) reader structure"]
impl crate::Readable for Vic310Spec {}
#[doc = "`write(|w| ..)` method takes [`vic310::W`](W) writer structure"]
impl crate::Writable for Vic310Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC310 to value 0"]
impl crate::Resettable for Vic310Spec {}
