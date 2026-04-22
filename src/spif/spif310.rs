#[doc = "Register `SPIF310` reader"]
pub type R = crate::R<Spif310Spec>;
#[doc = "Register `SPIF310` writer"]
pub type W = crate::W<Spif310Spec>;
#[doc = "Field `WPADDRCTL` reader - WP_ADDR_CTL"]
pub type WpaddrctlR = crate::FieldReader<u32>;
#[doc = "Field `WPADDRCTL` writer - WP_ADDR_CTL"]
pub type WpaddrctlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WP_ADDR_CTL"]
    #[inline(always)]
    pub fn wpaddrctl(&self) -> WpaddrctlR {
        WpaddrctlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WP_ADDR_CTL"]
    #[inline(always)]
    pub fn wpaddrctl(&mut self) -> WpaddrctlW<Spif310Spec> {
        WpaddrctlW::new(self, 0)
    }
}
#[doc = "SPIF\\_WPADDRCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`spif310::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif310::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif310Spec;
impl crate::RegisterSpec for Spif310Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif310::R`](R) reader structure"]
impl crate::Readable for Spif310Spec {}
#[doc = "`write(|w| ..)` method takes [`spif310::W`](W) writer structure"]
impl crate::Writable for Spif310Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF310 to value 0"]
impl crate::Resettable for Spif310Spec {}
