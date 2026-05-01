#[doc = "Register `SCU90C` reader"]
pub type R = crate::R<Scu90cSpec>;
#[doc = "Register `SCU90C` writer"]
pub type W = crate::W<Scu90cSpec>;
#[doc = "Field `SCUPSPICACHEABLE` reader - SCU_PSP_I_CACHEABLE"]
pub type ScupspicacheableR = crate::FieldReader<u32>;
#[doc = "Field `SCUPSPICACHEABLE` writer - SCU_PSP_I_CACHEABLE"]
pub type ScupspicacheableW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_PSP_I_CACHEABLE"]
    #[inline(always)]
    pub fn scupspicacheable(&self) -> ScupspicacheableR {
        ScupspicacheableR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_PSP_I_CACHEABLE"]
    #[inline(always)]
    pub fn scupspicacheable(&mut self) -> ScupspicacheableW<Scu90cSpec> {
        ScupspicacheableW::new(self, 0)
    }
}
#[doc = "\\PSP\\ Service Processor Control Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`scu90c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu90c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu90cSpec;
impl crate::RegisterSpec for Scu90cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu90c::R`](R) reader structure"]
impl crate::Readable for Scu90cSpec {}
#[doc = "`write(|w| ..)` method takes [`scu90c::W`](W) writer structure"]
impl crate::Writable for Scu90cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU90C to value 0"]
impl crate::Resettable for Scu90cSpec {}
