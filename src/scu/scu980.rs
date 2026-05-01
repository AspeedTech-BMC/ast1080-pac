#[doc = "Register `SCU980` reader"]
pub type R = crate::R<Scu980Spec>;
#[doc = "Register `SCU980` writer"]
pub type W = crate::W<Scu980Spec>;
#[doc = "Field `SCUEFUSECTRL` reader - SCU_EFUSE_CTRL"]
pub type ScuefusectrlR = crate::FieldReader<u32>;
#[doc = "Field `SCUEFUSECTRL` writer - SCU_EFUSE_CTRL"]
pub type ScuefusectrlW<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
#[doc = "Field `SCUEFUSESTATE` reader - SCU_EFUSE_STATE"]
pub type ScuefusestateR = crate::FieldReader;
#[doc = "Field `SCUEFUSEBUSY` reader - SCU_EFUSE_BUSY"]
pub type ScuefusebusyR = crate::BitReader;
#[doc = "Field `SCUEFUSEPROGRAM` reader - SCU_EFUSE_PROGRAM"]
pub type ScuefuseprogramR = crate::BitReader;
#[doc = "Field `SCUEFUSEREADDONE` reader - SCU_EFUSE_READ_DONE"]
pub type ScuefusereaddoneR = crate::BitReader;
#[doc = "Field `SCUEFUSEPGMDONE` reader - SCU_EFUSE_PGM_DONE"]
pub type ScuefusepgmdoneR = crate::BitReader;
impl R {
    #[doc = "Bits 0:22 - SCU_EFUSE_CTRL"]
    #[inline(always)]
    pub fn scuefusectrl(&self) -> ScuefusectrlR {
        ScuefusectrlR::new(self.bits & 0x007f_ffff)
    }
    #[doc = "Bits 23:27 - SCU_EFUSE_STATE"]
    #[inline(always)]
    pub fn scuefusestate(&self) -> ScuefusestateR {
        ScuefusestateR::new(((self.bits >> 23) & 0x1f) as u8)
    }
    #[doc = "Bit 28 - SCU_EFUSE_BUSY"]
    #[inline(always)]
    pub fn scuefusebusy(&self) -> ScuefusebusyR {
        ScuefusebusyR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SCU_EFUSE_PROGRAM"]
    #[inline(always)]
    pub fn scuefuseprogram(&self) -> ScuefuseprogramR {
        ScuefuseprogramR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - SCU_EFUSE_READ_DONE"]
    #[inline(always)]
    pub fn scuefusereaddone(&self) -> ScuefusereaddoneR {
        ScuefusereaddoneR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SCU_EFUSE_PGM_DONE"]
    #[inline(always)]
    pub fn scuefusepgmdone(&self) -> ScuefusepgmdoneR {
        ScuefusepgmdoneR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:22 - SCU_EFUSE_CTRL"]
    #[inline(always)]
    pub fn scuefusectrl(&mut self) -> ScuefusectrlW<Scu980Spec> {
        ScuefusectrlW::new(self, 0)
    }
}
#[doc = "EFUSE Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu980::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu980::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu980Spec;
impl crate::RegisterSpec for Scu980Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu980::R`](R) reader structure"]
impl crate::Readable for Scu980Spec {}
#[doc = "`write(|w| ..)` method takes [`scu980::W`](W) writer structure"]
impl crate::Writable for Scu980Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU980 to value 0"]
impl crate::Resettable for Scu980Spec {}
