#[doc = "Register `GPIO8C4` reader"]
pub type R = crate::R<Gpio8c4Spec>;
#[doc = "Register `GPIO8C4` writer"]
pub type W = crate::W<Gpio8c4Spec>;
#[doc = "Field `GPIO180WrPrivilegeOfMaster` reader - GPIO180 Write Privilege of Master"]
pub type Gpio180wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO180WrPrivilegeOfMaster` writer - GPIO180 Write Privilege of Master"]
pub type Gpio180wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO181WrPrivilegeOfMaster` reader - GPIO181 Write Privilege of Master"]
pub type Gpio181wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO181WrPrivilegeOfMaster` writer - GPIO181 Write Privilege of Master"]
pub type Gpio181wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO182WrPrivilegeOfMaster` reader - GPIO182 Write Privilege of Master"]
pub type Gpio182wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO182WrPrivilegeOfMaster` writer - GPIO182 Write Privilege of Master"]
pub type Gpio182wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO183WrPrivilegeOfMaster` reader - GPIO183 Write Privilege of Master"]
pub type Gpio183wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO183WrPrivilegeOfMaster` writer - GPIO183 Write Privilege of Master"]
pub type Gpio183wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO180 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio180wr_privilege_of_master(&self) -> Gpio180wrPrivilegeOfMasterR {
        Gpio180wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO181 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio181wr_privilege_of_master(&self) -> Gpio181wrPrivilegeOfMasterR {
        Gpio181wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO182 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio182wr_privilege_of_master(&self) -> Gpio182wrPrivilegeOfMasterR {
        Gpio182wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO183 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio183wr_privilege_of_master(&self) -> Gpio183wrPrivilegeOfMasterR {
        Gpio183wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO180 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio180wr_privilege_of_master(&mut self) -> Gpio180wrPrivilegeOfMasterW<Gpio8c4Spec> {
        Gpio180wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO181 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio181wr_privilege_of_master(&mut self) -> Gpio181wrPrivilegeOfMasterW<Gpio8c4Spec> {
        Gpio181wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO182 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio182wr_privilege_of_master(&mut self) -> Gpio182wrPrivilegeOfMasterW<Gpio8c4Spec> {
        Gpio182wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO183 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio183wr_privilege_of_master(&mut self) -> Gpio183wrPrivilegeOfMasterW<Gpio8c4Spec> {
        Gpio183wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#45\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio8c4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio8c4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio8c4Spec;
impl crate::RegisterSpec for Gpio8c4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio8c4::R`](R) reader structure"]
impl crate::Readable for Gpio8c4Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio8c4::W`](W) writer structure"]
impl crate::Writable for Gpio8c4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO8C4 to value 0xffff_ffff"]
impl crate::Resettable for Gpio8c4Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
