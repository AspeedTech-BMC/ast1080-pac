#[doc = "Register `HCIRHS060` reader"]
pub type R = crate::R<Hcirhs060Spec>;
#[doc = "Register `HCIRHS060` writer"]
pub type W = crate::W<Hcirhs060Spec>;
#[doc = "Field `REGCMDRINGBASELO` reader - REG_CMD_RING_BASE_LO"]
pub type RegcmdringbaseloR = crate::FieldReader<u32>;
#[doc = "Field `REGCMDRINGBASELO` writer - REG_CMD_RING_BASE_LO"]
pub type RegcmdringbaseloW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_CMD_RING_BASE_LO"]
    #[inline(always)]
    pub fn regcmdringbaselo(&self) -> RegcmdringbaseloR {
        RegcmdringbaseloR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_CMD_RING_BASE_LO"]
    #[inline(always)]
    pub fn regcmdringbaselo(&mut self) -> RegcmdringbaseloW<Hcirhs060Spec> {
        RegcmdringbaseloW::new(self, 0)
    }
}
#[doc = "RH\\_CMD\\_RING\\_BASE\\_LO\n\nYou can [`read`](crate::Reg::read) this register and get [`hcirhs060::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcirhs060::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcirhs060Spec;
impl crate::RegisterSpec for Hcirhs060Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcirhs060::R`](R) reader structure"]
impl crate::Readable for Hcirhs060Spec {}
#[doc = "`write(|w| ..)` method takes [`hcirhs060::W`](W) writer structure"]
impl crate::Writable for Hcirhs060Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIRHS060 to value 0"]
impl crate::Resettable for Hcirhs060Spec {}
