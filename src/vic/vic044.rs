#[doc = "Register `VIC044` reader"]
pub type R = crate::R<Vic044Spec>;
#[doc = "Register `VIC044` writer"]
pub type W = crate::W<Vic044Spec>;
#[doc = "Field `VICSIRQEXTMASTER` reader - VIC_SIRQ_EXT_MASTER"]
pub type VicsirqextmasterR = crate::FieldReader;
#[doc = "Field `VICSIRQEXTMASTER` writer - VIC_SIRQ_EXT_MASTER"]
pub type VicsirqextmasterW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - VIC_SIRQ_EXT_MASTER"]
    #[inline(always)]
    pub fn vicsirqextmaster(&self) -> VicsirqextmasterR {
        VicsirqextmasterR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - VIC_SIRQ_EXT_MASTER"]
    #[inline(always)]
    pub fn vicsirqextmaster(&mut self) -> VicsirqextmasterW<Vic044Spec> {
        VicsirqextmasterW::new(self, 0)
    }
}
#[doc = "External Master ID\n\nYou can [`read`](crate::Reg::read) this register and get [`vic044::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic044::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic044Spec;
impl crate::RegisterSpec for Vic044Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic044::R`](R) reader structure"]
impl crate::Readable for Vic044Spec {}
#[doc = "`write(|w| ..)` method takes [`vic044::W`](W) writer structure"]
impl crate::Writable for Vic044Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC044 to value 0"]
impl crate::Resettable for Vic044Spec {}
