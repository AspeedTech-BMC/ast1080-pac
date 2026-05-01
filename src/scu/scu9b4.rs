#[doc = "Register `SCU9B4` reader"]
pub type R = crate::R<Scu9b4Spec>;
#[doc = "Register `SCU9B4` writer"]
pub type W = crate::W<Scu9b4Spec>;
#[doc = "Field `SCUEFUSEPGMTIMING1` reader - SCU_EFUSE_PGM_TIMING_1"]
pub type Scuefusepgmtiming1R = crate::FieldReader<u32>;
#[doc = "Field `SCUEFUSEPGMTIMING1` writer - SCU_EFUSE_PGM_TIMING_1"]
pub type Scuefusepgmtiming1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_EFUSE_PGM_TIMING_1"]
    #[inline(always)]
    pub fn scuefusepgmtiming1(&self) -> Scuefusepgmtiming1R {
        Scuefusepgmtiming1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_EFUSE_PGM_TIMING_1"]
    #[inline(always)]
    pub fn scuefusepgmtiming1(&mut self) -> Scuefusepgmtiming1W<Scu9b4Spec> {
        Scuefusepgmtiming1W::new(self, 0)
    }
}
#[doc = "EFUSE PGM Timing Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu9b4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu9b4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu9b4Spec;
impl crate::RegisterSpec for Scu9b4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu9b4::R`](R) reader structure"]
impl crate::Readable for Scu9b4Spec {}
#[doc = "`write(|w| ..)` method takes [`scu9b4::W`](W) writer structure"]
impl crate::Writable for Scu9b4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU9B4 to value 0x2211"]
impl crate::Resettable for Scu9b4Spec {
    const RESET_VALUE: u32 = 0x2211;
}
