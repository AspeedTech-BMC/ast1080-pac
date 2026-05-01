#[doc = "Register `EHCIC[%s]` reader"]
pub type R = crate::R<EhcicSpec>;
#[doc = "Register `EHCIC[%s]` writer"]
pub type W = crate::W<EhcicSpec>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(self.bits)
    }
}
impl W {}
#[doc = "Companion Port Route Description (HCSP-PORTROUTE)\n\nYou can [`read`](crate::Reg::read) this register and get [`ehcic::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehcic::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EhcicSpec;
impl crate::RegisterSpec for EhcicSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ehcic::R`](R) reader structure"]
impl crate::Readable for EhcicSpec {}
#[doc = "`write(|w| ..)` method takes [`ehcic::W`](W) writer structure"]
impl crate::Writable for EhcicSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EHCIC[%s] to value 0"]
impl crate::Resettable for EhcicSpec {}
