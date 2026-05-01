#[doc = "Register `HCIRHS06C` reader"]
pub type R = crate::R<Hcirhs06cSpec>;
#[doc = "Register `HCIRHS06C` writer"]
pub type W = crate::W<Hcirhs06cSpec>;
#[doc = "Field `REGRESPRINGBASEHI` reader - REG_RESP_RING_BASE_HI"]
pub type RegrespringbasehiR = crate::FieldReader<u32>;
#[doc = "Field `REGRESPRINGBASEHI` writer - REG_RESP_RING_BASE_HI"]
pub type RegrespringbasehiW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_RESP_RING_BASE_HI"]
    #[inline(always)]
    pub fn regrespringbasehi(&self) -> RegrespringbasehiR {
        RegrespringbasehiR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_RESP_RING_BASE_HI"]
    #[inline(always)]
    pub fn regrespringbasehi(&mut self) -> RegrespringbasehiW<Hcirhs06cSpec> {
        RegrespringbasehiW::new(self, 0)
    }
}
#[doc = "RH\\_RESP\\_RING\\_BASE\\_HI\n\nYou can [`read`](crate::Reg::read) this register and get [`hcirhs06c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcirhs06c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcirhs06cSpec;
impl crate::RegisterSpec for Hcirhs06cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcirhs06c::R`](R) reader structure"]
impl crate::Readable for Hcirhs06cSpec {}
#[doc = "`write(|w| ..)` method takes [`hcirhs06c::W`](W) writer structure"]
impl crate::Writable for Hcirhs06cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIRHS06C to value 0"]
impl crate::Resettable for Hcirhs06cSpec {}
