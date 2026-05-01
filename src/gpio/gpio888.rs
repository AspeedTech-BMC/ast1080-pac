#[doc = "Register `GPIO888` reader"]
pub type R = crate::R<Gpio888Spec>;
#[doc = "Register `GPIO888` writer"]
pub type W = crate::W<Gpio888Spec>;
#[doc = "Field `GPIO120WrPrivilegeOfMaster` reader - GPIO120 Write Privilege of Master"]
pub type Gpio120wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO120WrPrivilegeOfMaster` writer - GPIO120 Write Privilege of Master"]
pub type Gpio120wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO121WrPrivilegeOfMaster` reader - GPIO121 Write Privilege of Master"]
pub type Gpio121wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO121WrPrivilegeOfMaster` writer - GPIO121 Write Privilege of Master"]
pub type Gpio121wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO122WrPrivilegeOfMaster` reader - GPIO122 Write Privilege of Master"]
pub type Gpio122wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO122WrPrivilegeOfMaster` writer - GPIO122 Write Privilege of Master"]
pub type Gpio122wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO123WrPrivilegeOfMaster` reader - GPIO123 Write Privilege of Master"]
pub type Gpio123wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO123WrPrivilegeOfMaster` writer - GPIO123 Write Privilege of Master"]
pub type Gpio123wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO120 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio120wr_privilege_of_master(&self) -> Gpio120wrPrivilegeOfMasterR {
        Gpio120wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO121 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio121wr_privilege_of_master(&self) -> Gpio121wrPrivilegeOfMasterR {
        Gpio121wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO122 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio122wr_privilege_of_master(&self) -> Gpio122wrPrivilegeOfMasterR {
        Gpio122wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO123 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio123wr_privilege_of_master(&self) -> Gpio123wrPrivilegeOfMasterR {
        Gpio123wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO120 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio120wr_privilege_of_master(&mut self) -> Gpio120wrPrivilegeOfMasterW<Gpio888Spec> {
        Gpio120wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO121 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio121wr_privilege_of_master(&mut self) -> Gpio121wrPrivilegeOfMasterW<Gpio888Spec> {
        Gpio121wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO122 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio122wr_privilege_of_master(&mut self) -> Gpio122wrPrivilegeOfMasterW<Gpio888Spec> {
        Gpio122wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO123 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio123wr_privilege_of_master(&mut self) -> Gpio123wrPrivilegeOfMasterW<Gpio888Spec> {
        Gpio123wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#30\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio888::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio888::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio888Spec;
impl crate::RegisterSpec for Gpio888Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio888::R`](R) reader structure"]
impl crate::Readable for Gpio888Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio888::W`](W) writer structure"]
impl crate::Writable for Gpio888Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO888 to value 0xffff_ffff"]
impl crate::Resettable for Gpio888Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
