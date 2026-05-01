#[doc = "Register `SCU6FC` reader"]
pub type R = crate::R<Scu6fcSpec>;
#[doc = "Register `SCU6FC` writer"]
pub type W = crate::W<Scu6fcSpec>;
#[doc = "Field `SCUENGPIOPTDEB` reader - SCU_EN_GPIO_PT_DEB"]
pub type ScuengpioptdebR = crate::FieldReader;
#[doc = "Field `SCUENGPIOPTDEB` writer - SCU_EN_GPIO_PT_DEB"]
pub type ScuengpioptdebW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - SCU_EN_GPIO_PT_DEB"]
    #[inline(always)]
    pub fn scuengpioptdeb(&self) -> ScuengpioptdebR {
        ScuengpioptdebR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - SCU_EN_GPIO_PT_DEB"]
    #[inline(always)]
    pub fn scuengpioptdeb(&mut self) -> ScuengpioptdebW<Scu6fcSpec> {
        ScuengpioptdebW::new(self, 0)
    }
}
#[doc = "GPIO Passthrough Debounce Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu6fc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu6fc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu6fcSpec;
impl crate::RegisterSpec for Scu6fcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu6fc::R`](R) reader structure"]
impl crate::Readable for Scu6fcSpec {}
#[doc = "`write(|w| ..)` method takes [`scu6fc::W`](W) writer structure"]
impl crate::Writable for Scu6fcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU6FC to value 0"]
impl crate::Resettable for Scu6fcSpec {}
