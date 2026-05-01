#[doc = "Register `SCU9B0` reader"]
pub type R = crate::R<Scu9b0Spec>;
#[doc = "Register `SCU9B0` writer"]
pub type W = crate::W<Scu9b0Spec>;
#[doc = "Field `SCUEFUSEPGMTIMING0` reader - SCU_EFUSE_PGM_TIMING_0"]
pub type Scuefusepgmtiming0R = crate::FieldReader<u32>;
#[doc = "Field `SCUEFUSEPGMTIMING0` writer - SCU_EFUSE_PGM_TIMING_0"]
pub type Scuefusepgmtiming0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_EFUSE_PGM_TIMING_0"]
    #[inline(always)]
    pub fn scuefusepgmtiming0(&self) -> Scuefusepgmtiming0R {
        Scuefusepgmtiming0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_EFUSE_PGM_TIMING_0"]
    #[inline(always)]
    pub fn scuefusepgmtiming0(&mut self) -> Scuefusepgmtiming0W<Scu9b0Spec> {
        Scuefusepgmtiming0W::new(self, 0)
    }
}
#[doc = "EFUSE PGM Timing Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`scu9b0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu9b0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu9b0Spec;
impl crate::RegisterSpec for Scu9b0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu9b0::R`](R) reader structure"]
impl crate::Readable for Scu9b0Spec {}
#[doc = "`write(|w| ..)` method takes [`scu9b0::W`](W) writer structure"]
impl crate::Writable for Scu9b0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU9B0 to value 0x0141_1218"]
impl crate::Resettable for Scu9b0Spec {
    const RESET_VALUE: u32 = 0x0141_1218;
}
