#[doc = "Register `HCIRHS068` reader"]
pub type R = crate::R<Hcirhs068Spec>;
#[doc = "Register `HCIRHS068` writer"]
pub type W = crate::W<Hcirhs068Spec>;
#[doc = "Field `REGRESPRINGBASELO` reader - REG_RESP_RING_BASE_LO"]
pub type RegrespringbaseloR = crate::FieldReader<u32>;
#[doc = "Field `REGRESPRINGBASELO` writer - REG_RESP_RING_BASE_LO"]
pub type RegrespringbaseloW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_RESP_RING_BASE_LO"]
    #[inline(always)]
    pub fn regrespringbaselo(&self) -> RegrespringbaseloR {
        RegrespringbaseloR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_RESP_RING_BASE_LO"]
    #[inline(always)]
    pub fn regrespringbaselo(&mut self) -> RegrespringbaseloW<Hcirhs068Spec> {
        RegrespringbaseloW::new(self, 0)
    }
}
#[doc = "RH\\_RESP\\_RING\\_BASE\\_LO\n\nYou can [`read`](crate::Reg::read) this register and get [`hcirhs068::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcirhs068::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcirhs068Spec;
impl crate::RegisterSpec for Hcirhs068Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcirhs068::R`](R) reader structure"]
impl crate::Readable for Hcirhs068Spec {}
#[doc = "`write(|w| ..)` method takes [`hcirhs068::W`](W) writer structure"]
impl crate::Writable for Hcirhs068Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIRHS068 to value 0"]
impl crate::Resettable for Hcirhs068Spec {}
