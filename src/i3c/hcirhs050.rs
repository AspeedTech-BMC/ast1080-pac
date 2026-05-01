#[doc = "Register `HCIRHS050` reader"]
pub type R = crate::R<Hcirhs050Spec>;
#[doc = "Register `HCIRHS050` writer"]
pub type W = crate::W<Hcirhs050Spec>;
#[doc = "Field `REGRINGENABLED` reader - REG_RING_ENABLED"]
pub type RegringenabledR = crate::BitReader;
#[doc = "Field `REGRINGRUNNING` reader - REG_RING_RUNNING"]
pub type RegringrunningR = crate::BitReader;
#[doc = "Field `REGRINGABORTED` reader - REG_RING_ABORTED"]
pub type RegringabortedR = crate::BitReader;
#[doc = "Field `REGRINGLOCKED` reader - REG_RING_LOCKED"]
pub type RegringlockedR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - REG_RING_ENABLED"]
    #[inline(always)]
    pub fn regringenabled(&self) -> RegringenabledR {
        RegringenabledR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - REG_RING_RUNNING"]
    #[inline(always)]
    pub fn regringrunning(&self) -> RegringrunningR {
        RegringrunningR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - REG_RING_ABORTED"]
    #[inline(always)]
    pub fn regringaborted(&self) -> RegringabortedR {
        RegringabortedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - REG_RING_LOCKED"]
    #[inline(always)]
    pub fn regringlocked(&self) -> RegringlockedR {
        RegringlockedR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {}
#[doc = "RH\\_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`hcirhs050::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcirhs050::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcirhs050Spec;
impl crate::RegisterSpec for Hcirhs050Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcirhs050::R`](R) reader structure"]
impl crate::Readable for Hcirhs050Spec {}
#[doc = "`write(|w| ..)` method takes [`hcirhs050::W`](W) writer structure"]
impl crate::Writable for Hcirhs050Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIRHS050 to value 0"]
impl crate::Resettable for Hcirhs050Spec {}
