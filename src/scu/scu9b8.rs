#[doc = "Register `SCU9B8` reader"]
pub type R = crate::R<Scu9b8Spec>;
#[doc = "Register `SCU9B8` writer"]
pub type W = crate::W<Scu9b8Spec>;
#[doc = "Field `SCUEFUSEREADTIMING0` reader - SCU_EFUSE_READ_TIMING_0"]
pub type Scuefusereadtiming0R = crate::FieldReader<u32>;
#[doc = "Field `SCUEFUSEREADTIMING0` writer - SCU_EFUSE_READ_TIMING_0"]
pub type Scuefusereadtiming0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_EFUSE_READ_TIMING_0"]
    #[inline(always)]
    pub fn scuefusereadtiming0(&self) -> Scuefusereadtiming0R {
        Scuefusereadtiming0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_EFUSE_READ_TIMING_0"]
    #[inline(always)]
    pub fn scuefusereadtiming0(&mut self) -> Scuefusereadtiming0W<Scu9b8Spec> {
        Scuefusereadtiming0W::new(self, 0)
    }
}
#[doc = "EFUSE Read Timing Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`scu9b8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu9b8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu9b8Spec;
impl crate::RegisterSpec for Scu9b8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu9b8::R`](R) reader structure"]
impl crate::Readable for Scu9b8Spec {}
#[doc = "`write(|w| ..)` method takes [`scu9b8::W`](W) writer structure"]
impl crate::Writable for Scu9b8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU9B8 to value 0x0041_1218"]
impl crate::Resettable for Scu9b8Spec {
    const RESET_VALUE: u32 = 0x0041_1218;
}
