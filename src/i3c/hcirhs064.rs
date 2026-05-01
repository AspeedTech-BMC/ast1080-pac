#[doc = "Register `HCIRHS064` reader"]
pub type R = crate::R<Hcirhs064Spec>;
#[doc = "Register `HCIRHS064` writer"]
pub type W = crate::W<Hcirhs064Spec>;
#[doc = "Field `REGCMDRINGBASEHI` reader - REG_CMD_RING_BASE_HI"]
pub type RegcmdringbasehiR = crate::FieldReader<u32>;
#[doc = "Field `REGCMDRINGBASEHI` writer - REG_CMD_RING_BASE_HI"]
pub type RegcmdringbasehiW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_CMD_RING_BASE_HI"]
    #[inline(always)]
    pub fn regcmdringbasehi(&self) -> RegcmdringbasehiR {
        RegcmdringbasehiR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_CMD_RING_BASE_HI"]
    #[inline(always)]
    pub fn regcmdringbasehi(&mut self) -> RegcmdringbasehiW<Hcirhs064Spec> {
        RegcmdringbasehiW::new(self, 0)
    }
}
#[doc = "RH\\_CMD\\_RING\\_BASE\\_HI\n\nYou can [`read`](crate::Reg::read) this register and get [`hcirhs064::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcirhs064::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcirhs064Spec;
impl crate::RegisterSpec for Hcirhs064Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcirhs064::R`](R) reader structure"]
impl crate::Readable for Hcirhs064Spec {}
#[doc = "`write(|w| ..)` method takes [`hcirhs064::W`](W) writer structure"]
impl crate::Writable for Hcirhs064Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIRHS064 to value 0"]
impl crate::Resettable for Hcirhs064Spec {}
