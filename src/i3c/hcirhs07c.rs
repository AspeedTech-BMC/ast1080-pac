#[doc = "Register `HCIRHS07C` reader"]
pub type R = crate::R<Hcirhs07cSpec>;
#[doc = "Register `HCIRHS07C` writer"]
pub type W = crate::W<Hcirhs07cSpec>;
#[doc = "Field `REGIBIDATARINGBASEHI` reader - REG_IBI_DATA_RING_BASE_HI"]
pub type RegibidataringbasehiR = crate::FieldReader<u32>;
#[doc = "Field `REGIBIDATARINGBASEHI` writer - REG_IBI_DATA_RING_BASE_HI"]
pub type RegibidataringbasehiW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_IBI_DATA_RING_BASE_HI"]
    #[inline(always)]
    pub fn regibidataringbasehi(&self) -> RegibidataringbasehiR {
        RegibidataringbasehiR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_IBI_DATA_RING_BASE_HI"]
    #[inline(always)]
    pub fn regibidataringbasehi(&mut self) -> RegibidataringbasehiW<Hcirhs07cSpec> {
        RegibidataringbasehiW::new(self, 0)
    }
}
#[doc = "RH\\_IBI\\_DATA\\_RING\\_BASE\\_HI\n\nYou can [`read`](crate::Reg::read) this register and get [`hcirhs07c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcirhs07c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcirhs07cSpec;
impl crate::RegisterSpec for Hcirhs07cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcirhs07c::R`](R) reader structure"]
impl crate::Readable for Hcirhs07cSpec {}
#[doc = "`write(|w| ..)` method takes [`hcirhs07c::W`](W) writer structure"]
impl crate::Writable for Hcirhs07cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIRHS07C to value 0"]
impl crate::Resettable for Hcirhs07cSpec {}
