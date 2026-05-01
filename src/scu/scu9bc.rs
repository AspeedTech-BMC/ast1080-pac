#[doc = "Register `SCU9BC` reader"]
pub type R = crate::R<Scu9bcSpec>;
#[doc = "Register `SCU9BC` writer"]
pub type W = crate::W<Scu9bcSpec>;
#[doc = "Field `SCUEFUSEREADTIMING1` reader - SCU_EFUSE_READ_TIMING_1"]
pub type Scuefusereadtiming1R = crate::FieldReader<u32>;
#[doc = "Field `SCUEFUSEREADTIMING1` writer - SCU_EFUSE_READ_TIMING_1"]
pub type Scuefusereadtiming1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_EFUSE_READ_TIMING_1"]
    #[inline(always)]
    pub fn scuefusereadtiming1(&self) -> Scuefusereadtiming1R {
        Scuefusereadtiming1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_EFUSE_READ_TIMING_1"]
    #[inline(always)]
    pub fn scuefusereadtiming1(&mut self) -> Scuefusereadtiming1W<Scu9bcSpec> {
        Scuefusereadtiming1W::new(self, 0)
    }
}
#[doc = "EFUSE Read Timing Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu9bc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu9bc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu9bcSpec;
impl crate::RegisterSpec for Scu9bcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu9bc::R`](R) reader structure"]
impl crate::Readable for Scu9bcSpec {}
#[doc = "`write(|w| ..)` method takes [`scu9bc::W`](W) writer structure"]
impl crate::Writable for Scu9bcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU9BC to value 0x2211"]
impl crate::Resettable for Scu9bcSpec {
    const RESET_VALUE: u32 = 0x2211;
}
